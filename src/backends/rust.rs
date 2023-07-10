// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Rust compiler backend.

use crate::{analyzer, ast};
use quote::{format_ident, quote};
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::path::Path;
use syn::LitInt;

use crate::analyzer::ast as analyzer_ast;

mod parser;
mod preamble;
mod serializer;
mod types;

use parser::FieldParser;
use serializer::FieldSerializer;

#[cfg(not(tm_mainline_prod))]
pub use heck::ToUpperCamelCase;

#[cfg(tm_mainline_prod)]
pub trait ToUpperCamelCase {
    fn to_upper_camel_case(&self) -> String;
}

#[cfg(tm_mainline_prod)]
impl ToUpperCamelCase for str {
    fn to_upper_camel_case(&self) -> String {
        use heck::CamelCase;
        let camel_case = self.to_camel_case();
        if camel_case.is_empty() {
            camel_case
        } else {
            // PDL identifiers are a-zA-z0-9, so we're dealing with
            // simple ASCII text.
            format!("{}{}", &camel_case[..1].to_ascii_uppercase(), &camel_case[1..])
        }
    }
}

/// Generate a block of code.
///
/// Like `quote!`, but the code block will be followed by an empty
/// line of code. This makes the generated code more readable.
#[macro_export]
macro_rules! quote_block {
    ($($tt:tt)*) => {
        format!("{}\n\n", ::quote::quote!($($tt)*))
    }
}

/// Generate a bit-mask which masks out `n` least significant bits.
///
/// Literal integers in Rust default to the `i32` type. For this
/// reason, if `n` is larger than 31, a suffix is added to the
/// `LitInt` returned. This should either be `u64` or `usize`
/// depending on where the result is used.
pub fn mask_bits(n: usize, suffix: &str) -> syn::LitInt {
    let suffix = if n > 31 { format!("_{suffix}") } else { String::new() };
    // Format the hex digits as 0x1111_2222_3333_usize.
    let hex_digits = format!("{:x}", (1u64 << n) - 1)
        .as_bytes()
        .rchunks(4)
        .rev()
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<&str>>()
        .join("_");
    syn::parse_str::<syn::LitInt>(&format!("0x{hex_digits}{suffix}")).unwrap()
}

fn generate_packet_size_getter<'a>(
    scope: &analyzer::Scope<'a>,
    fields: impl Iterator<Item = &'a analyzer_ast::Field>,
    is_packet: bool,
) -> (usize, proc_macro2::TokenStream) {
    let mut constant_width = 0;
    let mut dynamic_widths = Vec::new();

    for field in fields {
        if let Some(width) = field.annot.static_() {
            constant_width += width;
            continue;
        }

        let decl = scope.get_type_declaration(field);
        dynamic_widths.push(match &field.desc {
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body { .. } => {
                if is_packet {
                    quote! {
                        self.child.get_total_size()
                    }
                } else {
                    quote! {
                        self.payload.len()
                    }
                }
            }
            ast::FieldDesc::Typedef { id, .. } => {
                let id = format_ident!("{id}");
                quote!(self.#id.get_size())
            }
            ast::FieldDesc::Array { id, width, .. } => {
                let id = format_ident!("{id}");
                match &decl {
                    Some(analyzer_ast::Decl {
                        desc: ast::DeclDesc::Struct { .. } | ast::DeclDesc::CustomField { .. },
                        ..
                    }) => {
                        quote! {
                            self.#id.iter().map(|elem| elem.get_size()).sum::<usize>()
                        }
                    }
                    Some(analyzer_ast::Decl {
                        desc: ast::DeclDesc::Enum { width, .. }, ..
                    }) => {
                        let width = syn::Index::from(width / 8);
                        let mul_width = (width.index > 1).then(|| quote!(* #width));
                        quote! {
                            self.#id.len() #mul_width
                        }
                    }
                    _ => {
                        let width = syn::Index::from(width.unwrap() / 8);
                        let mul_width = (width.index > 1).then(|| quote!(* #width));
                        quote! {
                            self.#id.len() #mul_width
                        }
                    }
                }
            }
            _ => panic!("Unsupported field type: {field:?}"),
        });
    }

    if constant_width > 0 {
        let width = syn::Index::from(constant_width / 8);
        dynamic_widths.insert(0, quote!(#width));
    }
    if dynamic_widths.is_empty() {
        dynamic_widths.push(quote!(0))
    }

    (
        constant_width,
        quote! {
            #(#dynamic_widths)+*
        },
    )
}

fn top_level_packet<'a>(
    scope: &analyzer::Scope<'a>,
    packet_name: &'a str,
) -> &'a analyzer_ast::Decl {
    let mut decl = scope.typedef[packet_name];
    while let ast::DeclDesc::Packet { parent_id: Some(parent_id), .. }
    | ast::DeclDesc::Struct { parent_id: Some(parent_id), .. } = &decl.desc
    {
        decl = scope.typedef[parent_id];
    }
    decl
}

/// Find parent fields which are constrained in child packets.
///
/// These fields are the fields which need to be passed in when
/// parsing a `id` packet since their values are needed for one or
/// more child packets.
fn find_constrained_parent_fields<'a>(
    scope: &analyzer::Scope<'a>,
    id: &str,
) -> Vec<&'a analyzer_ast::Field> {
    let all_parent_fields: HashMap<String, &'a analyzer_ast::Field> = HashMap::from_iter(
        scope
            .iter_parent_fields(scope.typedef[id])
            .filter_map(|f| f.id().map(|id| (id.to_string(), f))),
    );

    let mut fields = Vec::new();
    let mut field_names = BTreeSet::new();
    let mut children = scope.iter_children(scope.typedef[id]).collect::<Vec<_>>();

    while let Some(child) = children.pop() {
        if let ast::DeclDesc::Packet { id, constraints, .. }
        | ast::DeclDesc::Struct { id, constraints, .. } = &child.desc
        {
            for constraint in constraints {
                if field_names.insert(&constraint.id)
                    && all_parent_fields.contains_key(&constraint.id)
                {
                    fields.push(all_parent_fields[&constraint.id]);
                }
            }
            children.extend(scope.iter_children(scope.typedef[id]).collect::<Vec<_>>());
        }
    }

    fields
}

/// Generate the declaration and implementation for a data struct.
///
/// This struct will hold the data for a packet or a struct. It knows
/// how to parse and serialize its own fields.
fn generate_data_struct(
    scope: &analyzer::Scope<'_>,
    endianness: ast::EndiannessValue,
    id: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let decl = scope.typedef[id];
    let is_packet = matches!(&decl.desc, ast::DeclDesc::Packet { .. });

    let span = format_ident!("bytes");
    let serializer_span = format_ident!("buffer");
    let mut field_parser = FieldParser::new(scope, endianness, id, &span);
    let mut field_serializer = FieldSerializer::new(scope, endianness, id, &serializer_span);
    for field in decl.fields() {
        field_parser.add(field);
        field_serializer.add(field);
    }
    field_parser.done();

    let (parse_arg_names, parse_arg_types) = if is_packet {
        let fields = find_constrained_parent_fields(scope, id);
        let names = fields.iter().map(|f| format_ident!("{}", f.id().unwrap())).collect::<Vec<_>>();
        let types = fields.iter().map(|f| types::rust_type(f)).collect::<Vec<_>>();
        (names, types)
    } else {
        (Vec::new(), Vec::new()) // No extra arguments to parse in structs.
    };

    let (constant_width, packet_size) =
        generate_packet_size_getter(scope, decl.fields(), is_packet);
    let conforms = if constant_width == 0 {
        quote! { true }
    } else {
        let constant_width = syn::Index::from(constant_width / 8);
        quote! { #span.len() >= #constant_width }
    };

    let visibility = if is_packet { quote!() } else { quote!(pub) };
    let has_payload = decl.payload().is_some();
    let has_children = scope.iter_children(decl).next().is_some();

    let struct_name = if is_packet { format_ident!("{id}Data") } else { format_ident!("{id}") };
    let fields_with_ids = decl.fields().filter(|f| f.id().is_some()).collect::<Vec<_>>();
    let mut field_names =
        fields_with_ids.iter().map(|f| format_ident!("{}", f.id().unwrap())).collect::<Vec<_>>();
    let mut field_types = fields_with_ids.iter().map(|f| types::rust_type(f)).collect::<Vec<_>>();
    if has_children || has_payload {
        if is_packet {
            field_names.push(format_ident!("child"));
            let field_type = format_ident!("{id}DataChild");
            field_types.push(quote!(#field_type));
        } else {
            field_names.push(format_ident!("payload"));
            field_types.push(quote!(Vec<u8>));
        }
    }

    let data_struct_decl = quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct #struct_name {
            #(#visibility #field_names: #field_types,)*
        }
    };

    let data_struct_impl = quote! {
        impl #struct_name {
            fn conforms(#span: &[u8]) -> bool {
                #conforms
            }

            #visibility fn parse(
                #span: &[u8] #(, #parse_arg_names: #parse_arg_types)*
            ) -> Result<Self> {
                let mut cell = Cell::new(#span);
                let packet = Self::parse_inner(&mut cell #(, #parse_arg_names)*)?;
                // TODO(mgeisler): communicate back to user if !cell.get().is_empty()?
                Ok(packet)
            }

            fn parse_inner(
                mut #span: &mut Cell<&[u8]> #(, #parse_arg_names: #parse_arg_types)*
            ) -> Result<Self> {
                #field_parser
                Ok(Self {
                    #(#field_names,)*
                })
            }

            fn write_to(&self, buffer: &mut BytesMut) {
                #field_serializer
            }

            fn get_total_size(&self) -> usize {
                self.get_size()
            }

            fn get_size(&self) -> usize {
                #packet_size
            }
        }
    };

    (data_struct_decl, data_struct_impl)
}

/// Turn the constraint into a value (such as `10` or
/// `SomeEnum::Foo`).
pub fn constraint_to_value(
    all_fields: &HashMap<String, &'_ analyzer_ast::Field>,
    constraint: &ast::Constraint,
) -> proc_macro2::TokenStream {
    match constraint {
        ast::Constraint { value: Some(value), .. } => {
            let value = proc_macro2::Literal::usize_unsuffixed(*value);
            quote!(#value)
        }
        // TODO(mgeisler): include type_id in `ast::Constraint` and
        // drop the packet_scope argument.
        ast::Constraint { tag_id: Some(tag_id), .. } => {
            let type_id = match &all_fields[&constraint.id].desc {
                ast::FieldDesc::Typedef { type_id, .. } => format_ident!("{type_id}"),
                _ => unreachable!("Invalid constraint: {constraint:?}"),
            };
            let tag_id = format_ident!("{}", tag_id.to_upper_camel_case());
            quote!(#type_id::#tag_id)
        }
        _ => unreachable!("Invalid constraint: {constraint:?}"),
    }
}

/// Generate code for a `ast::Decl::Packet`.
fn generate_packet_decl(
    scope: &analyzer::Scope<'_>,
    endianness: ast::EndiannessValue,
    id: &str,
) -> proc_macro2::TokenStream {
    let decl = scope.typedef[id];
    let top_level = top_level_packet(scope, id);
    let top_level_id = top_level.id().unwrap();
    let top_level_packet = format_ident!("{top_level_id}");
    let top_level_data = format_ident!("{top_level_id}Data");
    let top_level_id_lower = format_ident!("{}", top_level_id.to_lowercase());

    // TODO(mgeisler): use the convert_case crate to convert between
    // `FooBar` and `foo_bar` in the code below.
    let span = format_ident!("bytes");
    let id_lower = format_ident!("{}", id.to_lowercase());
    let id_packet = format_ident!("{id}");
    let id_child = format_ident!("{id}Child");
    let id_data_child = format_ident!("{id}DataChild");
    let id_builder = format_ident!("{id}Builder");

    let mut parents = scope.iter_parents_and_self(decl).collect::<Vec<_>>();
    parents.reverse();

    let parent_ids = parents.iter().map(|p| p.id().unwrap()).collect::<Vec<_>>();
    let parent_shifted_ids = parent_ids.iter().skip(1).map(|id| format_ident!("{id}"));
    let parent_lower_ids =
        parent_ids.iter().map(|id| format_ident!("{}", id.to_lowercase())).collect::<Vec<_>>();
    let parent_shifted_lower_ids = parent_lower_ids.iter().skip(1).collect::<Vec<_>>();
    let parent_packet = parent_ids.iter().map(|id| format_ident!("{id}"));
    let parent_data = parent_ids.iter().map(|id| format_ident!("{id}Data"));
    let parent_data_child = parent_ids.iter().map(|id| format_ident!("{id}DataChild"));

    let all_fields = {
        let mut fields = scope.iter_fields(decl).filter(|d| d.id().is_some()).collect::<Vec<_>>();
        fields.sort_by_key(|f| f.id());
        fields
    };
    let all_named_fields =
        HashMap::from_iter(all_fields.iter().map(|f| (f.id().unwrap().to_string(), *f)));

    let all_field_names =
        all_fields.iter().map(|f| format_ident!("{}", f.id().unwrap())).collect::<Vec<_>>();
    let all_field_types = all_fields.iter().map(|f| types::rust_type(f)).collect::<Vec<_>>();
    let all_field_borrows =
        all_fields.iter().map(|f| types::rust_borrow(f, scope)).collect::<Vec<_>>();
    let all_field_getter_names = all_field_names.iter().map(|id| format_ident!("get_{id}"));
    let all_field_self_field = all_fields.iter().map(|f| {
        for (parent, parent_id) in parents.iter().zip(parent_lower_ids.iter()) {
            if parent.fields().any(|ff| ff.id() == f.id()) {
                return quote!(self.#parent_id);
            }
        }
        unreachable!("Could not find {f:?} in parent chain");
    });

    let all_constraints = HashMap::<String, _>::from_iter(
        scope.iter_constraints(decl).map(|c| (c.id.to_string(), c)),
    );

    let unconstrained_fields = all_fields
        .iter()
        .filter(|f| !all_constraints.contains_key(f.id().unwrap()))
        .collect::<Vec<_>>();
    let unconstrained_field_names = unconstrained_fields
        .iter()
        .map(|f| format_ident!("{}", f.id().unwrap()))
        .collect::<Vec<_>>();
    let unconstrained_field_types = unconstrained_fields.iter().map(|f| types::rust_type(f));

    let rev_parents = parents.iter().rev().collect::<Vec<_>>();
    let builder_assignments = rev_parents.iter().enumerate().map(|(idx, parent)| {
        let parent_id = parent.id().unwrap();
        let parent_id_lower = format_ident!("{}", parent_id.to_lowercase());
        let parent_data = format_ident!("{parent_id}Data");
        let parent_data_child = format_ident!("{parent_id}DataChild");

        let named_fields = {
            let mut names = parent.fields().filter_map(ast::Field::id).collect::<Vec<_>>();
            names.sort_unstable();
            names
        };

        let mut field = named_fields.iter().map(|id| format_ident!("{id}")).collect::<Vec<_>>();
        let mut value = named_fields
            .iter()
            .map(|&id| match all_constraints.get(id) {
                Some(constraint) => constraint_to_value(&all_named_fields, constraint),
                None => {
                    let id = format_ident!("{id}");
                    quote!(self.#id)
                }
            })
            .collect::<Vec<_>>();

        if parent.payload().is_some() {
            field.push(format_ident!("child"));
            if idx == 0 {
                // Top-most parent, the child is simply created from
                // our payload.
                value.push(quote! {
                    match self.payload {
                        None => #parent_data_child::None,
                        Some(bytes) => #parent_data_child::Payload(bytes),
                    }
                });
            } else {
                // Child is created from the previous parent.
                let prev_parent_id = rev_parents[idx - 1].id().unwrap();
                let prev_parent_id_lower = format_ident!("{}", prev_parent_id.to_lowercase());
                let prev_parent_id = format_ident!("{prev_parent_id}");
                value.push(quote! {
                    #parent_data_child::#prev_parent_id(#prev_parent_id_lower)
                });
            }
        } else if scope.iter_children(parent).next().is_some() {
            field.push(format_ident!("child"));
            value.push(quote! { #parent_data_child::None });
        }

        quote! {
            let #parent_id_lower = #parent_data {
                #(#field: #value,)*
            };
        }
    });

    let children = scope.iter_children(decl).collect::<Vec<_>>();
    let has_payload = decl.payload().is_some();
    let has_children_or_payload = !children.is_empty() || has_payload;
    let child =
        children.iter().map(|child| format_ident!("{}", child.id().unwrap())).collect::<Vec<_>>();
    let child_data = child.iter().map(|child| format_ident!("{child}Data")).collect::<Vec<_>>();
    let get_payload = (children.is_empty() && has_payload).then(|| {
        quote! {
            pub fn get_payload(&self) -> &[u8] {
                match &self.#id_lower.child {
                    #id_data_child::Payload(bytes) => &bytes,
                    #id_data_child::None => &[],
                }
            }
        }
    });
    let child_declaration = has_children_or_payload.then(|| {
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #id_data_child {
                #(#child(#child_data),)*
                Payload(Bytes),
                None,
            }

            impl #id_data_child {
                fn get_total_size(&self) -> usize {
                    match self {
                        #(#id_data_child::#child(value) => value.get_total_size(),)*
                        #id_data_child::Payload(bytes) => bytes.len(),
                        #id_data_child::None => 0,
                    }
                }
            }

            #[derive(Debug, Clone, PartialEq, Eq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #id_child {
                #(#child(#child),)*
                Payload(Bytes),
                None,
            }
        }
    });
    let specialize = has_children_or_payload.then(|| {
        quote! {
            pub fn specialize(&self) -> #id_child {
                match &self.#id_lower.child {
                    #(
                        #id_data_child::#child(_) =>
                        #id_child::#child(#child::new(self.#top_level_id_lower.clone()).unwrap()),
                    )*
                    #id_data_child::Payload(payload) => #id_child::Payload(payload.clone()),
                    #id_data_child::None => #id_child::None,
                }
            }
        }
    });

    let builder_payload_field = has_children_or_payload.then(|| {
        quote! {
            pub payload: Option<Bytes>
        }
    });

    let ancestor_packets =
        parent_ids[..parent_ids.len() - 1].iter().map(|id| format_ident!("{id}"));
    let impl_from_and_try_from = (top_level_id != id).then(|| {
        quote! {
            #(
                impl From<#id_packet> for #ancestor_packets {
                    fn from(packet: #id_packet) -> #ancestor_packets {
                        #ancestor_packets::new(packet.#top_level_id_lower).unwrap()
                    }
                }
            )*

            impl TryFrom<#top_level_packet> for #id_packet {
                type Error = Error;
                fn try_from(packet: #top_level_packet) -> Result<#id_packet> {
                    #id_packet::new(packet.#top_level_id_lower)
                }
            }
        }
    });

    let (data_struct_decl, data_struct_impl) = generate_data_struct(scope, endianness, id);

    quote! {
        #child_declaration

        #data_struct_decl

        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct #id_packet {
            #(
                #[cfg_attr(feature = "serde", serde(flatten))]
                #parent_lower_ids: #parent_data,
            )*
        }

        #[derive(Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct #id_builder {
            #(pub #unconstrained_field_names: #unconstrained_field_types,)*
            #builder_payload_field
        }

        #data_struct_impl

        impl Packet for #id_packet {
            fn to_bytes(self) -> Bytes {
                let mut buffer = BytesMut::with_capacity(self.#top_level_id_lower.get_size());
                self.#top_level_id_lower.write_to(&mut buffer);
                buffer.freeze()
            }

            fn to_vec(self) -> Vec<u8> {
                self.to_bytes().to_vec()
            }
        }

        impl From<#id_packet> for Bytes {
            fn from(packet: #id_packet) -> Self {
                packet.to_bytes()
            }
        }

        impl From<#id_packet> for Vec<u8> {
            fn from(packet: #id_packet) -> Self {
                packet.to_vec()
            }
        }

        #impl_from_and_try_from

        impl #id_packet {
            pub fn parse(#span: &[u8]) -> Result<Self> {
                let mut cell = Cell::new(#span);
                let packet = Self::parse_inner(&mut cell)?;
                // TODO(mgeisler): communicate back to user if !cell.get().is_empty()?
                Ok(packet)
            }

            fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
                let data = #top_level_data::parse_inner(&mut bytes)?;
                Self::new(data)
            }

            #specialize

            fn new(#top_level_id_lower: #top_level_data) -> Result<Self> {
                #(
                    let #parent_shifted_lower_ids = match &#parent_lower_ids.child {
                        #parent_data_child::#parent_shifted_ids(value) => value.clone(),
                        _ => return Err(Error::InvalidChildError {
                            expected: stringify!(#parent_data_child::#parent_shifted_ids),
                            actual: format!("{:?}", &#parent_lower_ids.child),
                        }),
                    };
                )*
                Ok(Self { #(#parent_lower_ids),* })
            }

            #(pub fn #all_field_getter_names(&self) -> #all_field_borrows #all_field_types {
                #all_field_borrows #all_field_self_field.#all_field_names
            })*

            #get_payload

            fn write_to(&self, buffer: &mut BytesMut) {
                self.#id_lower.write_to(buffer)
            }

            pub fn get_size(&self) -> usize {
                self.#top_level_id_lower.get_size()
            }
        }

        impl #id_builder {
            pub fn build(self) -> #id_packet {
                #(#builder_assignments;)*
                #id_packet::new(#top_level_id_lower).unwrap()
            }
        }

        #(
            impl From<#id_builder> for #parent_packet {
                fn from(builder: #id_builder) -> #parent_packet {
                    builder.build().into()
                }
            }
        )*
    }
}

/// Generate code for a `ast::Decl::Struct`.
fn generate_struct_decl(
    scope: &analyzer::Scope<'_>,
    endianness: ast::EndiannessValue,
    id: &str,
) -> proc_macro2::TokenStream {
    let (struct_decl, struct_impl) = generate_data_struct(scope, endianness, id);
    quote! {
        #struct_decl
        #struct_impl
    }
}

/// Generate an enum declaration.
///
/// # Arguments
/// * `id` - Enum identifier.
/// * `tags` - List of enum tags.
/// * `width` - Width of the backing type of the enum, in bits.
/// * `open` - Whether to generate an open or closed enum. Open enums have
///            an additional Unknown case for unmatched valued. Complete
///            enums (where the full range of values is covered) are
///            automatically closed.
fn generate_enum_decl(id: &str, tags: &[ast::Tag], width: usize) -> proc_macro2::TokenStream {
    // Determine if the enum is open, i.e. a default tag is defined.
    fn enum_default_tag(tags: &[ast::Tag]) -> Option<ast::TagOther> {
        tags.iter()
            .filter_map(|tag| match tag {
                ast::Tag::Other(tag) => Some(tag.clone()),
                _ => None,
            })
            .next()
    }

    // Determine if the enum is complete, i.e. all values in the backing
    // integer range have a matching tag in the original declaration.
    fn enum_is_complete(tags: &[ast::Tag], max: usize) -> bool {
        let mut ranges = tags
            .iter()
            .filter_map(|tag| match tag {
                ast::Tag::Value(tag) => Some((tag.value, tag.value)),
                ast::Tag::Range(tag) => Some(tag.range.clone().into_inner()),
                _ => None,
            })
            .collect::<Vec<_>>();
        ranges.sort_unstable();
        ranges.first().unwrap().0 == 0
            && ranges.last().unwrap().1 == max
            && ranges.windows(2).all(|window| {
                if let [left, right] = window {
                    left.1 == right.0 - 1
                } else {
                    false
                }
            })
    }

    // Determine if the enum is primitive, i.e. does not contain any tag range.
    fn enum_is_primitive(tags: &[ast::Tag]) -> bool {
        tags.iter().all(|tag| matches!(tag, ast::Tag::Value(_)))
    }

    // Return the maximum value for the scalar type.
    fn scalar_max(width: usize) -> usize {
        if width >= usize::BITS as usize {
            usize::MAX
        } else {
            (1 << width) - 1
        }
    }

    // Format an enum tag identifier to rust upper caml case.
    fn format_tag_ident(id: &str) -> proc_macro2::TokenStream {
        let id = format_ident!("{}", id.to_upper_camel_case());
        quote! { #id }
    }

    // Format a constant value as hexadecimal constant.
    fn format_value(value: usize) -> LitInt {
        syn::parse_str::<syn::LitInt>(&format!("{:#x}", value)).unwrap()
    }

    // Backing type for the enum.
    let backing_type = types::Integer::new(width);
    let backing_type_str = proc_macro2::Literal::string(&format!("u{}", backing_type.width));
    let range_max = scalar_max(width);
    let default_tag = enum_default_tag(tags);
    let is_open = default_tag.is_some();
    let is_complete = enum_is_complete(tags, scalar_max(width));
    let is_primitive = enum_is_primitive(tags);
    let name = format_ident!("{id}");

    // Generate the variant cases for the enum declaration.
    // Tags declared in ranges are flattened in the same declaration.
    let use_variant_values = is_primitive && (is_complete || !is_open);
    let repr_u64 = use_variant_values.then(|| quote! { #[repr(u64)] });
    let mut variants = vec![];
    for tag in tags.iter() {
        match tag {
            ast::Tag::Value(tag) if use_variant_values => {
                let id = format_tag_ident(&tag.id);
                let value = format_value(tag.value);
                variants.push(quote! { #id = #value })
            }
            ast::Tag::Value(tag) => variants.push(format_tag_ident(&tag.id)),
            ast::Tag::Range(tag) => {
                variants.extend(tag.tags.iter().map(|tag| format_tag_ident(&tag.id)));
                let id = format_tag_ident(&tag.id);
                variants.push(quote! { #id(Private<#backing_type>) })
            }
            ast::Tag::Other(_) => (),
        }
    }

    // Generate the cases for parsing the enum value from an integer.
    let mut from_cases = vec![];
    for tag in tags.iter() {
        match tag {
            ast::Tag::Value(tag) => {
                let id = format_tag_ident(&tag.id);
                let value = format_value(tag.value);
                from_cases.push(quote! { #value => Ok(#name::#id) })
            }
            ast::Tag::Range(tag) => {
                from_cases.extend(tag.tags.iter().map(|tag| {
                    let id = format_tag_ident(&tag.id);
                    let value = format_value(tag.value);
                    quote! { #value => Ok(#name::#id) }
                }));
                let id = format_tag_ident(&tag.id);
                let start = format_value(*tag.range.start());
                let end = format_value(*tag.range.end());
                from_cases.push(quote! { #start ..= #end => Ok(#name::#id(Private(value))) })
            }
            ast::Tag::Other(_) => (),
        }
    }

    // Generate the cases for serializing the enum value to an integer.
    let mut into_cases = vec![];
    for tag in tags.iter() {
        match tag {
            ast::Tag::Value(tag) => {
                let id = format_tag_ident(&tag.id);
                let value = format_value(tag.value);
                into_cases.push(quote! { #name::#id => #value })
            }
            ast::Tag::Range(tag) => {
                into_cases.extend(tag.tags.iter().map(|tag| {
                    let id = format_tag_ident(&tag.id);
                    let value = format_value(tag.value);
                    quote! { #name::#id => #value }
                }));
                let id = format_tag_ident(&tag.id);
                into_cases.push(quote! { #name::#id(Private(value)) => *value })
            }
            ast::Tag::Other(_) => (),
        }
    }

    // Generate a default case if the enum is open and incomplete.
    if !is_complete && is_open {
        let unknown_id = format_tag_ident(&default_tag.unwrap().id);
        let range_max = format_value(range_max);
        variants.push(quote! { #unknown_id(Private<#backing_type>) });
        from_cases.push(quote! { 0..=#range_max => Ok(#name::#unknown_id(Private(value))) });
        into_cases.push(quote! { #name::#unknown_id(Private(value)) => *value });
    }

    // Generate an error case if the enum size is lower than the backing
    // type size, or if the enum is closed or incomplete.
    if backing_type.width != width || (!is_complete && !is_open) {
        from_cases.push(quote! { _ => Err(value) });
    }

    // Derive other Into<uN> and Into<iN> implementations from the explicit
    // implementation, where the type is larger than the backing type.
    let derived_signed_into_types = [8, 16, 32, 64]
        .into_iter()
        .filter(|w| *w > width)
        .map(|w| syn::parse_str::<syn::Type>(&format!("i{}", w)).unwrap());
    let derived_unsigned_into_types = [8, 16, 32, 64]
        .into_iter()
        .filter(|w| *w >= width && *w != backing_type.width)
        .map(|w| syn::parse_str::<syn::Type>(&format!("u{}", w)).unwrap());
    let derived_into_types = derived_signed_into_types.chain(derived_unsigned_into_types);

    quote! {
        #repr_u64
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(try_from = #backing_type_str, into = #backing_type_str))]
        pub enum #name {
            #(#variants,)*
        }

        impl TryFrom<#backing_type> for #name {
            type Error = #backing_type;
            fn try_from(value: #backing_type) -> std::result::Result<Self, Self::Error> {
                match value {
                    #(#from_cases,)*
                }
            }
        }

        impl From<&#name> for #backing_type {
            fn from(value: &#name) -> Self {
                match value {
                    #(#into_cases,)*
                }
            }
        }

        impl From<#name> for #backing_type {
            fn from(value: #name) -> Self {
                (&value).into()
            }
        }

        #(impl From<#name> for #derived_into_types {
            fn from(value: #name) -> Self {
                #backing_type::from(value) as Self
            }
        })*
    }
}

/// Generate the declaration for a custom field of static size.
///
/// * `id` - Enum identifier.
/// * `width` - Width of the backing type of the enum, in bits.
fn generate_custom_field_decl(id: &str, width: usize) -> proc_macro2::TokenStream {
    let id = format_ident!("{}", id);
    let backing_type = types::Integer::new(width);
    let backing_type_str = proc_macro2::Literal::string(&format!("u{}", backing_type.width));
    let max_value = mask_bits(width, &format!("u{}", backing_type.width));
    let common = quote! {
        impl From<&#id> for #backing_type {
            fn from(value: &#id) -> #backing_type {
                value.0
            }
        }

        impl From<#id> for #backing_type {
            fn from(value: #id) -> #backing_type {
                value.0
            }
        }
    };

    if backing_type.width == width {
        quote! {
            #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = "serde", serde(from = #backing_type_str, into = #backing_type_str))]
            pub struct #id(#backing_type);

            #common

            impl From<#backing_type> for #id {
                fn from(value: #backing_type) -> Self {
                    #id(value)
                }
            }
        }
    } else {
        quote! {
            #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = "serde", serde(try_from = #backing_type_str, into = #backing_type_str))]
            pub struct #id(#backing_type);

            #common

            impl TryFrom<#backing_type> for #id {
                type Error = #backing_type;
                fn try_from(value: #backing_type) -> std::result::Result<Self, Self::Error> {
                    if value > #max_value {
                        Err(value)
                    } else {
                        Ok(#id(value))
                    }
                }
            }
        }
    }
}

fn generate_decl(
    scope: &analyzer::Scope<'_>,
    file: &analyzer_ast::File,
    decl: &analyzer_ast::Decl,
) -> proc_macro2::TokenStream {
    match &decl.desc {
        ast::DeclDesc::Packet { id, .. } => generate_packet_decl(scope, file.endianness.value, id),
        ast::DeclDesc::Struct { id, parent_id: None, .. } => {
            // TODO(mgeisler): handle structs with parents. We could
            // generate code for them, but the code is not useful
            // since it would require the caller to unpack everything
            // manually. We either need to change the API, or
            // implement the recursive (de)serialization.
            generate_struct_decl(scope, file.endianness.value, id)
        }
        ast::DeclDesc::Enum { id, tags, width } => generate_enum_decl(id, tags, *width),
        ast::DeclDesc::CustomField { id, width: Some(width), .. } => {
            generate_custom_field_decl(id, *width)
        }
        _ => todo!("unsupported Decl::{:?}", decl),
    }
}

/// Generate Rust code from an AST.
///
/// The code is not formatted, pipe it through `rustfmt` to get
/// readable source code.
pub fn generate(sources: &ast::SourceDatabase, file: &analyzer_ast::File) -> String {
    let source = sources.get(file.file).expect("could not read source");
    let preamble = preamble::generate(Path::new(source.name()));

    let scope = analyzer::Scope::new(file).expect("could not create scope");
    let decls = file.declarations.iter().map(|decl| generate_decl(&scope, file, decl));
    let code = quote! {
        #preamble

        #(#decls)*
    };
    let syntax_tree = syn::parse2(code).expect("Could not parse code");
    prettyplease::unparse(&syntax_tree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer;
    use crate::ast;
    use crate::parser::parse_inline;
    use crate::test_utils::{assert_snapshot_eq, format_rust};
    use googletest::prelude::{elements_are, eq, expect_that};
    use paste::paste;

    /// Parse a string fragment as a PDL file.
    ///
    /// # Panics
    ///
    /// Panics on parse errors.
    pub fn parse_str(text: &str) -> analyzer_ast::File {
        let mut db = ast::SourceDatabase::new();
        let file =
            parse_inline(&mut db, String::from("stdin"), String::from(text)).expect("parse error");
        analyzer::analyze(&file).expect("analyzer error")
    }

    #[googletest::test]
    fn test_find_constrained_parent_fields() -> googletest::Result<()> {
        let code = "
              little_endian_packets
              packet Parent {
                a: 8,
                b: 8,
                c: 8,
                _payload_,
              }
              packet Child: Parent(a = 10) {
                x: 8,
                _payload_,
              }
              packet GrandChild: Child(b = 20) {
                y: 8,
                _payload_,
              }
              packet GrandGrandChild: GrandChild(c = 30) {
                z: 8,
              }
            ";
        let file = parse_str(code);
        let scope = analyzer::Scope::new(&file).unwrap();
        let find_fields = |id| {
            find_constrained_parent_fields(&scope, id)
                .iter()
                .map(|field| field.id().unwrap())
                .collect::<Vec<_>>()
        };

        expect_that!(find_fields("Parent"), elements_are![]);
        expect_that!(find_fields("Child"), elements_are![eq("b"), eq("c")]);
        expect_that!(find_fields("GrandChild"), elements_are![eq("c")]);
        expect_that!(find_fields("GrandGrandChild"), elements_are![]);
        Ok(())
    }

    /// Create a unit test for the given PDL `code`.
    ///
    /// The unit test will compare the generated Rust code for all
    /// declarations with previously saved snapshots. The snapshots
    /// are read from `"tests/generated/{name}_{endianness}_{id}.rs"`
    /// where `is` taken from the declaration.
    ///
    /// When adding new tests or modifying existing ones, use
    /// `UPDATE_SNAPSHOTS=1 cargo test` to automatically populate the
    /// snapshots with the expected output.
    ///
    /// The `code` cannot have an endianness declaration, instead you
    /// must supply either `little_endian` or `big_endian` as
    /// `endianness`.
    macro_rules! make_pdl_test {
        ($name:ident, $code:expr, $endianness:ident) => {
            paste! {
                #[test]
                fn [< test_ $name _ $endianness >]() {
                    let name = stringify!($name);
                    let endianness = stringify!($endianness);
                    let code = format!("{endianness}_packets\n{}", $code);
                    let mut db = ast::SourceDatabase::new();
                    let file = parse_inline(&mut db, String::from("test"), code).unwrap();
                    let file = analyzer::analyze(&file).unwrap();
                    let actual_code = generate(&db, &file);
                    assert_snapshot_eq(
                        &format!("tests/generated/{name}_{endianness}.rs"),
                        &format_rust(&actual_code),
                    );
                }
            }
        };
    }

    /// Create little- and bit-endian tests for the given PDL `code`.
    ///
    /// The `code` cannot have an endianness declaration: we will
    /// automatically generate unit tests for both
    /// "little_endian_packets" and "big_endian_packets".
    macro_rules! test_pdl {
        ($name:ident, $code:expr $(,)?) => {
            make_pdl_test!($name, $code, little_endian);
            make_pdl_test!($name, $code, big_endian);
        };
    }

    test_pdl!(packet_decl_empty, "packet Foo {}");

    test_pdl!(packet_decl_8bit_scalar, " packet Foo { x:  8 }");
    test_pdl!(packet_decl_24bit_scalar, "packet Foo { x: 24 }");
    test_pdl!(packet_decl_64bit_scalar, "packet Foo { x: 64 }");

    test_pdl!(
        enum_declaration,
        r#"
        enum IncompleteTruncatedClosed : 3 {
            A = 0,
            B = 1,
        }

        enum IncompleteTruncatedOpen : 3 {
            A = 0,
            B = 1,
            UNKNOWN = ..
        }

        enum IncompleteTruncatedClosedWithRange : 3 {
            A = 0,
            B = 1..6 {
                X = 1,
                Y = 2,
            }
        }

        enum IncompleteTruncatedOpenWithRange : 3 {
            A = 0,
            B = 1..6 {
                X = 1,
                Y = 2,
            },
            UNKNOWN = ..
        }

        enum CompleteTruncated : 3 {
            A = 0,
            B = 1,
            C = 2,
            D = 3,
            E = 4,
            F = 5,
            G = 6,
            H = 7,
        }

        enum CompleteTruncatedWithRange : 3 {
            A = 0,
            B = 1..7 {
                X = 1,
                Y = 2,
            }
        }

        enum CompleteWithRange : 8 {
            A = 0,
            B = 1,
            C = 2..255,
        }
        "#
    );

    test_pdl!(
        custom_field_declaration,
        r#"
        // Still unsupported.
        // custom_field Dynamic "dynamic"

        // Should generate a type with From<u32> implementation.
        custom_field ExactSize : 32 "exact_size"

        // Should generate a type with TryFrom<u32> implementation.
        custom_field TruncatedSize : 24 "truncated_size"
        "#
    );

    test_pdl!(
        packet_decl_simple_scalars,
        r#"
          packet Foo {
            x: 8,
            y: 16,
            z: 24,
          }
        "#
    );

    test_pdl!(
        packet_decl_complex_scalars,
        r#"
          packet Foo {
            a: 3,
            b: 8,
            c: 5,
            d: 24,
            e: 12,
            f: 4,
          }
        "#,
    );

    // Test that we correctly mask a byte-sized value in the middle of
    // a chunk.
    test_pdl!(
        packet_decl_mask_scalar_value,
        r#"
          packet Foo {
            a: 2,
            b: 24,
            c: 6,
          }
        "#,
    );

    test_pdl!(
        struct_decl_complex_scalars,
        r#"
          struct Foo {
            a: 3,
            b: 8,
            c: 5,
            d: 24,
            e: 12,
            f: 4,
          }
        "#,
    );

    test_pdl!(packet_decl_8bit_enum, " enum Foo :  8 { A = 1, B = 2 } packet Bar { x: Foo }");
    test_pdl!(packet_decl_24bit_enum, "enum Foo : 24 { A = 1, B = 2 } packet Bar { x: Foo }");
    test_pdl!(packet_decl_64bit_enum, "enum Foo : 64 { A = 1, B = 2 } packet Bar { x: Foo }");

    test_pdl!(
        packet_decl_mixed_scalars_enums,
        "
          enum Enum7 : 7 {
            A = 1,
            B = 2,
          }

          enum Enum9 : 9 {
            A = 1,
            B = 2,
          }

          packet Foo {
            x: Enum7,
            y: 5,
            z: Enum9,
            w: 3,
          }
        "
    );

    test_pdl!(packet_decl_8bit_scalar_array, " packet Foo { x:  8[3] }");
    test_pdl!(packet_decl_24bit_scalar_array, "packet Foo { x: 24[5] }");
    test_pdl!(packet_decl_64bit_scalar_array, "packet Foo { x: 64[7] }");

    test_pdl!(
        packet_decl_8bit_enum_array,
        "enum Foo :  8 { FOO_BAR = 1, BAZ = 2 } packet Bar { x: Foo[3] }"
    );
    test_pdl!(
        packet_decl_24bit_enum_array,
        "enum Foo : 24 { FOO_BAR = 1, BAZ = 2 } packet Bar { x: Foo[5] }"
    );
    test_pdl!(
        packet_decl_64bit_enum_array,
        "enum Foo : 64 { FOO_BAR = 1, BAZ = 2 } packet Bar { x: Foo[7] }"
    );

    test_pdl!(
        packet_decl_array_dynamic_count,
        "
          packet Foo {
            _count_(x): 5,
            padding: 3,
            x: 24[]
          }
        "
    );

    test_pdl!(
        packet_decl_array_dynamic_size,
        "
          packet Foo {
            _size_(x): 5,
            padding: 3,
            x: 24[]
          }
        "
    );

    test_pdl!(
        packet_decl_array_unknown_element_width_dynamic_size,
        "
          struct Foo {
            _count_(a): 40,
            a: 16[],
          }

          packet Bar {
            _size_(x): 40,
            x: Foo[],
          }
        "
    );

    test_pdl!(
        packet_decl_array_unknown_element_width_dynamic_count,
        "
          struct Foo {
            _count_(a): 40,
            a: 16[],
          }

          packet Bar {
            _count_(x): 40,
            x: Foo[],
          }
        "
    );

    test_pdl!(
        packet_decl_array_with_padding,
        "
          struct Foo {
            _count_(a): 40,
            a: 16[],
          }

          packet Bar {
            a: Foo[],
            _padding_ [128],
          }
        "
    );

    test_pdl!(
        packet_decl_reserved_field,
        "
          packet Foo {
            _reserved_: 40,
          }
        "
    );

    test_pdl!(
        packet_decl_custom_field,
        r#"
          custom_field Bar1 : 24 "exact"
          custom_field Bar2 : 32 "truncated"

          packet Foo {
            a: Bar1,
            b: Bar2,
          }
        "#
    );

    test_pdl!(
        packet_decl_fixed_scalar_field,
        "
          packet Foo {
            _fixed_ = 7 : 7,
            b: 57,
          }
        "
    );

    test_pdl!(
        packet_decl_fixed_enum_field,
        "
          enum Enum7 : 7 {
            A = 1,
            B = 2,
          }

          packet Foo {
              _fixed_ = A : Enum7,
              b: 57,
          }
        "
    );

    test_pdl!(
        packet_decl_payload_field_variable_size,
        "
          packet Foo {
              a: 8,
              _size_(_payload_): 8,
              _payload_,
              b: 16,
          }
        "
    );

    test_pdl!(
        packet_decl_payload_field_unknown_size,
        "
          packet Foo {
              a: 24,
              _payload_,
          }
        "
    );

    test_pdl!(
        packet_decl_payload_field_unknown_size_terminal,
        "
          packet Foo {
              _payload_,
              a: 24,
          }
        "
    );

    test_pdl!(
        packet_decl_child_packets,
        "
          enum Enum16 : 16 {
            A = 1,
            B = 2,
          }

          packet Foo {
              a: 8,
              b: Enum16,
              _size_(_payload_): 8,
              _payload_
          }

          packet Bar : Foo (a = 100) {
              x: 8,
          }

          packet Baz : Foo (b = B) {
              y: 16,
          }
        "
    );

    test_pdl!(
        packet_decl_grand_children,
        "
          enum Enum16 : 16 {
            A = 1,
            B = 2,
          }

          packet Parent {
              foo: Enum16,
              bar: Enum16,
              baz: Enum16,
              _size_(_payload_): 8,
              _payload_
          }

          packet Child : Parent (foo = A) {
              quux: Enum16,
              _payload_,
          }

          packet GrandChild : Child (bar = A, quux = A) {
              _body_,
          }

          packet GrandGrandChild : GrandChild (baz = A) {
              _body_,
          }
        "
    );

    test_pdl!(
        packet_decl_parent_with_no_payload,
        "
          enum Enum8 : 8 {
            A = 0,
          }

          packet Parent {
            v : Enum8,
          }

          packet Child : Parent (v = A) {
          }
        "
    );

    test_pdl!(
        packet_decl_parent_with_alias_child,
        "
          enum Enum8 : 8 {
            A = 0,
            B = 1,
            C = 2,
          }

          packet Parent {
            v : Enum8,
            _payload_,
          }

          packet AliasChild : Parent {
            _payload_
          }

          packet NormalChild : Parent (v = A) {
          }

          packet NormalGrandChild1 : AliasChild (v = B) {
          }

          packet NormalGrandChild2 : AliasChild (v = C) {
              _payload_
          }
        "
    );

    // TODO(mgeisler): enable this test when we have an approach to
    // struct fields with parents.
    //
    // test_pdl!(
    //     struct_decl_child_structs,
    //     "
    //       enum Enum16 : 16 {
    //         A = 1,
    //         B = 2,
    //       }
    //
    //       struct Foo {
    //           a: 8,
    //           b: Enum16,
    //           _size_(_payload_): 8,
    //           _payload_
    //       }
    //
    //       struct Bar : Foo (a = 100) {
    //           x: 8,
    //       }
    //
    //       struct Baz : Foo (b = B) {
    //           y: 16,
    //       }
    //     "
    // );
    //
    // test_pdl!(
    //     struct_decl_grand_children,
    //     "
    //       enum Enum16 : 16 {
    //         A = 1,
    //         B = 2,
    //       }
    //
    //       struct Parent {
    //           foo: Enum16,
    //           bar: Enum16,
    //           baz: Enum16,
    //           _size_(_payload_): 8,
    //           _payload_
    //       }
    //
    //       struct Child : Parent (foo = A) {
    //           quux: Enum16,
    //           _payload_,
    //       }
    //
    //       struct GrandChild : Child (bar = A, quux = A) {
    //           _body_,
    //       }
    //
    //       struct GrandGrandChild : GrandChild (baz = A) {
    //           _body_,
    //       }
    //     "
    // );
}
