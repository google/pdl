//! Rust compiler backend.

// The `format-push-string` lint was briefly enabled present in Rust
// 1.62. It is now moved the disabled "restriction" category instead.
// See https://github.com/rust-lang/rust-clippy/issues/9077 for the
// problems with this lint.
//
// Remove this when we use Rust 1.63 or later.
#![allow(clippy::format_push_string)]

use crate::{ast, lint};
use heck::ToUpperCamelCase;
use quote::{format_ident, quote};
use std::collections::BTreeSet;
use std::path::Path;

use crate::parser::ast as parser_ast;

mod parser;
mod preamble;
mod serializer;
mod types;

use parser::FieldParser;
use serializer::FieldSerializer;

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

fn generate_packet_size_getter(
    scope: &lint::Scope<'_>,
    fields: &[&parser_ast::Field],
    is_packet: bool,
) -> (usize, proc_macro2::TokenStream) {
    let mut constant_width = 0;
    let mut dynamic_widths = Vec::new();

    for field in fields {
        if let Some(width) = field.width(scope, false) {
            constant_width += width;
            continue;
        }

        let decl = field.declaration(scope);
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
                    Some(parser_ast::Decl {
                        desc: ast::DeclDesc::Struct { .. } | ast::DeclDesc::CustomField { .. },
                        ..
                    }) => {
                        quote! {
                            self.#id.iter().map(|elem| elem.get_size()).sum::<usize>()
                        }
                    }
                    Some(parser_ast::Decl { desc: ast::DeclDesc::Enum { .. }, .. }) => {
                        let width =
                            syn::Index::from(decl.unwrap().width(scope, false).unwrap() / 8);
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

fn top_level_packet<'a>(scope: &lint::Scope<'a>, packet_name: &'a str) -> &'a parser_ast::Decl {
    let mut decl = scope.typedef[packet_name];
    while let ast::DeclDesc::Packet { parent_id: Some(parent_id), .. }
    | ast::DeclDesc::Struct { parent_id: Some(parent_id), .. } = &decl.desc
    {
        decl = scope.typedef[parent_id];
    }
    decl
}

fn get_packet_children<'a>(scope: &'a lint::Scope<'_>, id: &str) -> &'a [&'a parser_ast::Decl] {
    scope.children.get(id).map(Vec::as_slice).unwrap_or_default()
}

/// Find all constrained fields in children of `id`.
fn find_constrained_fields<'a>(
    scope: &'a lint::Scope<'a>,
    id: &'a str,
) -> Vec<&'a parser_ast::Field> {
    let mut fields = Vec::new();
    let mut field_names = BTreeSet::new();
    let mut children = Vec::from(get_packet_children(scope, id));

    while let Some(child) = children.pop() {
        if let ast::DeclDesc::Packet { id, constraints, .. }
        | ast::DeclDesc::Struct { id, constraints, .. } = &child.desc
        {
            let packet_scope = &scope.scopes[&scope.typedef[id]];
            for constraint in constraints {
                if field_names.insert(&constraint.id) {
                    fields.push(packet_scope.all_fields[&constraint.id]);
                }
            }
            children.extend(get_packet_children(scope, id));
        }
    }

    fields
}

/// Find parent fields which are constrained in child packets.
///
/// These fields are the fields which need to be passed in when
/// parsing a `id` packet since their values are needed for one or
/// more child packets.
fn find_constrained_parent_fields<'a>(
    scope: &'a lint::Scope<'a>,
    id: &'a str,
) -> impl Iterator<Item = &'a parser_ast::Field> {
    let packet_scope = &scope.scopes[&scope.typedef[id]];
    find_constrained_fields(scope, id).into_iter().filter(|field| {
        let id = field.id().unwrap();
        packet_scope.all_fields.contains_key(id) && !packet_scope.named.contains_key(id)
    })
}

/// Generate the declaration and implementation for a data struct.
///
/// This struct will hold the data for a packet or a struct. It knows
/// how to parse and serialize its own fields.
fn generate_data_struct(
    scope: &lint::Scope<'_>,
    endianness: ast::EndiannessValue,
    id: &str,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let decl = scope.typedef[id];
    let packet_scope = &scope.scopes[&decl];
    let is_packet = matches!(&decl.desc, ast::DeclDesc::Packet { .. });

    let span = format_ident!("bytes");
    let serializer_span = format_ident!("buffer");
    let mut field_parser = FieldParser::new(scope, endianness, id, &span);
    let mut field_serializer = FieldSerializer::new(scope, endianness, id, &serializer_span);
    for field in &packet_scope.fields {
        field_parser.add(field);
        field_serializer.add(field);
    }
    field_parser.done();

    let (parse_arg_names, parse_arg_types) = if is_packet {
        let fields = find_constrained_parent_fields(scope, id).collect::<Vec<_>>();
        let names = fields.iter().map(|f| format_ident!("{}", f.id().unwrap())).collect::<Vec<_>>();
        let types = fields.iter().map(|f| types::rust_type(f)).collect::<Vec<_>>();
        (names, types)
    } else {
        (Vec::new(), Vec::new()) // No extra arguments to parse in structs.
    };

    let (constant_width, packet_size) =
        generate_packet_size_getter(scope, &packet_scope.fields, is_packet);
    let conforms = if constant_width == 0 {
        quote! { true }
    } else {
        let constant_width = syn::Index::from(constant_width / 8);
        quote! { #span.len() >= #constant_width }
    };

    let visibility = if is_packet { quote!() } else { quote!(pub) };
    let has_payload = packet_scope.payload.is_some();
    let children = get_packet_children(scope, id);
    let has_children_or_payload = !children.is_empty() || has_payload;
    let struct_name = if is_packet { format_ident!("{id}Data") } else { format_ident!("{id}") };
    let fields_with_ids =
        packet_scope.fields.iter().filter(|f| f.id().is_some()).collect::<Vec<_>>();
    let mut field_names =
        fields_with_ids.iter().map(|f| format_ident!("{}", f.id().unwrap())).collect::<Vec<_>>();
    let mut field_types = fields_with_ids.iter().map(|f| types::rust_type(f)).collect::<Vec<_>>();
    if has_children_or_payload {
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

/// Find all parents from `id`.
///
/// This includes the `Decl` for `id` itself.
fn find_parents<'a>(scope: &lint::Scope<'a>, id: &str) -> Vec<&'a parser_ast::Decl> {
    let mut decl = scope.typedef[id];
    let mut parents = vec![decl];
    while let ast::DeclDesc::Packet { parent_id: Some(parent_id), .. }
    | ast::DeclDesc::Struct { parent_id: Some(parent_id), .. } = &decl.desc
    {
        decl = scope.typedef[parent_id];
        parents.push(decl);
    }
    parents.reverse();
    parents
}

/// Turn the constraint into a value (such as `10` or
/// `SomeEnum::Foo`).
pub fn constraint_to_value(
    packet_scope: &lint::PacketScope<'_>,
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
            let type_id = match &packet_scope.all_fields[&constraint.id].desc {
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
    scope: &lint::Scope<'_>,
    endianness: ast::EndiannessValue,
    id: &str,
) -> proc_macro2::TokenStream {
    let packet_scope = &scope.scopes[&scope.typedef[id]];

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

    let parents = find_parents(scope, id);
    let parent_ids = parents.iter().map(|p| p.id().unwrap()).collect::<Vec<_>>();
    let parent_shifted_ids = parent_ids.iter().skip(1).map(|id| format_ident!("{id}"));
    let parent_lower_ids =
        parent_ids.iter().map(|id| format_ident!("{}", id.to_lowercase())).collect::<Vec<_>>();
    let parent_shifted_lower_ids = parent_lower_ids.iter().skip(1).collect::<Vec<_>>();
    let parent_packet = parent_ids.iter().map(|id| format_ident!("{id}"));
    let parent_data = parent_ids.iter().map(|id| format_ident!("{id}Data"));
    let parent_data_child = parent_ids.iter().map(|id| format_ident!("{id}DataChild"));

    let all_fields = {
        let mut fields = packet_scope.all_fields.values().collect::<Vec<_>>();
        fields.sort_by_key(|f| f.id());
        fields
    };
    let all_field_names =
        all_fields.iter().map(|f| format_ident!("{}", f.id().unwrap())).collect::<Vec<_>>();
    let all_field_types = all_fields.iter().map(|f| types::rust_type(f)).collect::<Vec<_>>();
    let all_field_borrows =
        all_fields.iter().map(|f| types::rust_borrow(f, scope)).collect::<Vec<_>>();
    let all_field_getter_names = all_field_names.iter().map(|id| format_ident!("get_{id}"));
    let all_field_self_field = all_fields.iter().map(|f| {
        for (parent, parent_id) in parents.iter().zip(parent_lower_ids.iter()) {
            if scope.scopes[parent].fields.contains(f) {
                return quote!(self.#parent_id);
            }
        }
        unreachable!("Could not find {f:?} in parent chain");
    });

    let unconstrained_fields = all_fields
        .iter()
        .filter(|f| !packet_scope.all_constraints.contains_key(f.id().unwrap()))
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
        let parent_packet_scope = &scope.scopes[&scope.typedef[parent_id]];

        let named_fields = {
            let mut names = parent_packet_scope.named.keys().collect::<Vec<_>>();
            names.sort();
            names
        };

        let mut field = named_fields.iter().map(|id| format_ident!("{id}")).collect::<Vec<_>>();
        let mut value = named_fields
            .iter()
            .map(|&id| match packet_scope.all_constraints.get(id) {
                Some(constraint) => constraint_to_value(packet_scope, constraint),
                None => {
                    let id = format_ident!("{id}");
                    quote!(self.#id)
                }
            })
            .collect::<Vec<_>>();

        if parent_packet_scope.payload.is_some() {
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
        }

        quote! {
            let #parent_id_lower = Arc::new(#parent_data {
                #(#field: #value,)*
            });
        }
    });

    let children = get_packet_children(scope, id);
    let has_payload = packet_scope.payload.is_some();
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
                #(#child(Arc<#child_data>),)*
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
                type Error = TryFromError;
                fn try_from(packet: #top_level_packet) -> std::result::Result<#id_packet, TryFromError> {
                    #id_packet::new(packet.#top_level_id_lower).map_err(TryFromError)
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
                #parent_lower_ids: Arc<#parent_data>,
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
                Ok(Self::new(Arc::new(data)).unwrap())
            }

            #specialize

            fn new(#top_level_id_lower: Arc<#top_level_data>)
                   -> std::result::Result<Self, &'static str> {
                #(
                    let #parent_shifted_lower_ids = match &#parent_lower_ids.child {
                        #parent_data_child::#parent_shifted_ids(value) => value.clone(),
                        _ => return Err("Could not parse data, wrong child type"),
                    };
                )*
                Ok(Self { #(#parent_lower_ids),* })
            }

            #(pub fn #all_field_getter_names(&self) -> #all_field_borrows #all_field_types {
                #all_field_borrows #all_field_self_field.as_ref().#all_field_names
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
    scope: &lint::Scope<'_>,
    endianness: ast::EndiannessValue,
    id: &str,
) -> proc_macro2::TokenStream {
    let (struct_decl, struct_impl) = generate_data_struct(scope, endianness, id);
    quote! {
        #struct_decl
        #struct_impl
    }
}

fn generate_enum_decl(id: &str, tags: &[ast::Tag]) -> proc_macro2::TokenStream {
    let name = format_ident!("{id}");
    let variants =
        tags.iter().map(|t| format_ident!("{}", t.id.to_upper_camel_case())).collect::<Vec<_>>();
    let values = tags
        .iter()
        .map(|t| syn::parse_str::<syn::LitInt>(&format!("{:#x}", t.value)).unwrap())
        .collect::<Vec<_>>();
    let visitor_name = format_ident!("{id}Visitor");

    quote! {
        #[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
        #[repr(u64)]
        pub enum #name {
            #(#variants = #values,)*
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u64(*self as u64)
            }
        }

        #[cfg(feature = "serde")]
        struct #visitor_name;

        #[cfg(feature = "serde")]
        impl<'de> serde::de::Visitor<'de> for #visitor_name {
            type Value = #name;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid discriminant")
            }

            fn visit_u64<E>(self, value: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    #(#values => Ok(#name::#variants),)*
                    _ => Err(E::custom(format!("invalid discriminant: {value}"))),
                }
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_u64(#visitor_name)
            }
        }
    }
}

fn generate_decl(
    scope: &lint::Scope<'_>,
    file: &parser_ast::File,
    decl: &parser_ast::Decl,
) -> String {
    match &decl.desc {
        ast::DeclDesc::Packet { id, .. } => {
            generate_packet_decl(scope, file.endianness.value, id).to_string()
        }
        ast::DeclDesc::Struct { id, parent_id: None, .. } => {
            // TODO(mgeisler): handle structs with parents. We could
            // generate code for them, but the code is not useful
            // since it would require the caller to unpack everything
            // manually. We either need to change the API, or
            // implement the recursive (de)serialization.
            generate_struct_decl(scope, file.endianness.value, id).to_string()
        }
        ast::DeclDesc::Enum { id, tags, .. } => generate_enum_decl(id, tags).to_string(),
        _ => todo!("unsupported Decl::{:?}", decl),
    }
}

/// Generate Rust code from an AST.
///
/// The code is not formatted, pipe it through `rustfmt` to get
/// readable source code.
pub fn generate(sources: &ast::SourceDatabase, file: &parser_ast::File) -> String {
    let mut code = String::new();

    let source = sources.get(file.file).expect("could not read source");
    code.push_str(&preamble::generate(Path::new(source.name())));

    let scope = lint::Scope::new(file).unwrap();
    for decl in &file.declarations {
        code.push_str(&generate_decl(&scope, file, decl));
        code.push_str("\n\n");
    }

    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;
    use crate::parser::parse_inline;
    use crate::test_utils::{assert_snapshot_eq, rustfmt};
    use paste::paste;

    /// Parse a string fragment as a PDL file.
    ///
    /// # Panics
    ///
    /// Panics on parse errors.
    pub fn parse_str(text: &str) -> parser_ast::File {
        let mut db = ast::SourceDatabase::new();
        parse_inline(&mut db, String::from("stdin"), String::from(text)).expect("parse error")
    }

    #[track_caller]
    fn assert_iter_eq<T: std::cmp::PartialEq + std::fmt::Debug>(
        left: impl IntoIterator<Item = T>,
        right: impl IntoIterator<Item = T>,
    ) {
        assert_eq!(left.into_iter().collect::<Vec<T>>(), right.into_iter().collect::<Vec<T>>());
    }

    #[test]
    fn test_find_constrained_parent_fields() {
        let code = "
              little_endian_packets
              packet Parent {
                a: 8,
                b: 8,
                c: 8,
              }
              packet Child: Parent(a = 10) {
                x: 8,
              }
              packet GrandChild: Child(b = 20) {
                y: 8,
              }
              packet GrandGrandChild: GrandChild(c = 30) {
                z: 8,
              }
            ";
        let file = parse_str(code);
        let scope = lint::Scope::new(&file).unwrap();
        let find_fields =
            |id| find_constrained_parent_fields(&scope, id).map(|field| field.id().unwrap());
        assert_iter_eq(find_fields("Parent"), vec![]);
        assert_iter_eq(find_fields("Child"), vec!["b", "c"]);
        assert_iter_eq(find_fields("GrandChild"), vec!["c"]);
        assert_iter_eq(find_fields("GrandGrandChild"), vec![]);
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
                    let actual_code = generate(&db, &file);
                    assert_snapshot_eq(
                        &format!("tests/generated/{name}_{endianness}.rs"),
                        &rustfmt(&actual_code),
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
        packet_decl_reserved_field,
        "
          packet Foo {
            _reserved_: 40,
          }
        "
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
