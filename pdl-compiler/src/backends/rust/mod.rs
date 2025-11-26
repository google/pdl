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
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;
use syn::LitInt;

mod decoder;
mod encoder;
mod preamble;
pub mod test;
mod types;

use decoder::FieldParser;
pub use heck::ToUpperCamelCase;

pub trait ToIdent {
    /// Generate a sanitized rust identifier.
    /// Rust specific keywords are renamed for validity.
    fn to_ident(self) -> proc_macro2::Ident;
}

impl ToIdent for &'_ str {
    fn to_ident(self) -> proc_macro2::Ident {
        match self {
            "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern"
            | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod"
            | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self" | "static" | "struct"
            | "super" | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while"
            | "async" | "await" | "dyn" | "abstract" | "become" | "box" | "do" | "final"
            | "macro" | "override" | "priv" | "typeof" | "unsized" | "virtual" | "yield"
            | "try" => format_ident!("r#{}", self),
            _ => format_ident!("{}", self),
        }
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

/// Return the list of fields that will appear in the generated
/// rust structs (<Packet> and <Packet>Builder).
///
///  - must be a named field
///  - must not be a flag
///  - must not appear in the packet constraints.
///
/// The fields are presented in declaration order, with ancestor
/// fields declared first.
/// The payload field _ if declared _ is handled separately.
fn packet_data_fields<'a>(
    scope: &'a analyzer::Scope<'a>,
    decl: &'a ast::Decl,
) -> Vec<&'a ast::Field> {
    let all_constraints = HashMap::<String, _>::from_iter(
        scope.iter_constraints(decl).map(|c| (c.id.to_string(), c)),
    );

    scope
        .iter_fields(decl)
        .filter(|f| f.id().is_some())
        .filter(|f| !matches!(&f.desc, ast::FieldDesc::Flag { .. }))
        .filter(|f| !all_constraints.contains_key(f.id().unwrap()))
        .collect::<Vec<_>>()
}

/// Return the list of fields that have a constant value.
/// The fields are presented in declaration order, with ancestor
/// fields declared first.
fn packet_constant_fields<'a>(
    scope: &'a analyzer::Scope<'a>,
    decl: &'a ast::Decl,
) -> Vec<&'a ast::Field> {
    let all_constraints = HashMap::<String, _>::from_iter(
        scope.iter_constraints(decl).map(|c| (c.id.to_string(), c)),
    );

    scope
        .iter_fields(decl)
        .filter(|f| f.id().is_some())
        .filter(|f| all_constraints.contains_key(f.id().unwrap()))
        .collect::<Vec<_>>()
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Hash)]
enum ConstraintValue {
    Scalar(usize),
    Tag(String, String),
}

impl quote::ToTokens for ConstraintValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            ConstraintValue::Scalar(s) => {
                let s = proc_macro2::Literal::usize_unsuffixed(*s);
                quote!(#s)
            }
            ConstraintValue::Tag(e, t) => {
                let tag_id = format_ident!("{}", t.to_upper_camel_case());
                let type_id = format_ident!("{}", e);
                quote!(#type_id::#tag_id)
            }
        })
    }
}

fn constraint_value_ast(
    fields: &[&'_ ast::Field],
    constraint: &ast::Constraint,
) -> ConstraintValue {
    match constraint {
        ast::Constraint { value: Some(value), .. } => ConstraintValue::Scalar(*value),
        ast::Constraint { tag_id: Some(tag_id), .. } => {
            let type_id = fields
                .iter()
                .filter_map(|f| match &f.desc {
                    ast::FieldDesc::Typedef { id, type_id } if id == &constraint.id => {
                        Some(type_id)
                    }
                    _ => None,
                })
                .next()
                .unwrap();
            ConstraintValue::Tag(type_id.clone(), tag_id.clone())
        }
        _ => unreachable!("Invalid constraint: {constraint:?}"),
    }
}

fn constraint_value(
    fields: &[&'_ ast::Field],
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
            let tag_id = format_ident!("{}", tag_id.to_upper_camel_case());
            let type_id = fields
                .iter()
                .filter_map(|f| match &f.desc {
                    ast::FieldDesc::Enum { id, enum_id, .. } if id == &constraint.id => {
                        Some(enum_id.to_ident())
                    }
                    _ => None,
                })
                .next()
                .unwrap();
            quote!(#type_id::#tag_id)
        }
        _ => unreachable!("Invalid constraint: {constraint:?}"),
    }
}

fn constraint_value_str(fields: &[&'_ ast::Field], constraint: &ast::Constraint) -> String {
    match constraint {
        ast::Constraint { value: Some(value), .. } => {
            format!("{value}")
        }
        ast::Constraint { tag_id: Some(tag_id), .. } => {
            let tag_id = format_ident!("{}", tag_id.to_upper_camel_case());
            let type_id = fields
                .iter()
                .filter_map(|f| match &f.desc {
                    ast::FieldDesc::Typedef { id, type_id } if id == &constraint.id => {
                        Some(type_id.to_ident())
                    }
                    _ => None,
                })
                .next()
                .unwrap();
            format!("{type_id}::{tag_id}")
        }
        _ => unreachable!("Invalid constraint: {constraint:?}"),
    }
}

fn implements_copy(scope: &analyzer::Scope<'_>, field: &ast::Field) -> bool {
    match &field.desc {
        ast::FieldDesc::Scalar { .. } => true,
        ast::FieldDesc::Enum { .. } => true,
        ast::FieldDesc::Typedef { type_id, .. } => match &scope.typedef[type_id].desc {
            ast::DeclDesc::CustomField { .. } => true,
            ast::DeclDesc::Struct { .. } => false,
            desc => unreachable!("unexpected declaration: {desc:?}"),
        },
        ast::FieldDesc::Array { .. } => false,
        _ => todo!(),
    }
}

/// Generate the implementation of the specialize method.
///
/// The function is generated after selecting the information from the parent
/// packet that can be used to identify with
/// _certainty_ the child packet.
///
/// The discriminant information is:
///     - field values
///     - payload size, to disambiguate between children of
///       identical constant size
///
/// The generator will raise warnings if ambiguities remain after all
/// information is taken into account, i.e. two child packets map to the same
/// constraints. In this case ambiguities are resolved by trying each child
/// in order of declaration.
fn generate_specialize_impl(
    scope: &analyzer::Scope<'_>,
    schema: &analyzer::Schema,
    decl: &ast::Decl,
    id: &str,
    data_fields: &[&ast::Field],
) -> Result<proc_macro2::TokenStream, String> {
    #[derive(PartialEq, Eq)]
    struct SpecializeCase {
        id: String,
        constraints: HashMap<String, ConstraintValue>,
        size: analyzer::Size,
    }

    fn gather_specialize_cases(
        scope: &analyzer::Scope<'_>,
        schema: &analyzer::Schema,
        id: &str,
        decl: &ast::Decl,
        data_fields: &[&ast::Field],
        constraints: &HashMap<String, ConstraintValue>,
        specialize_cases: &mut Vec<SpecializeCase>,
    ) {
        // Add local constraints to the context.
        let mut constraints = constraints.clone();
        for c in decl.constraints() {
            if data_fields.iter().any(|f| f.id() == Some(&c.id)) {
                constraints.insert(c.id.to_owned(), constraint_value_ast(data_fields, c));
            }
        }

        // Generate specialize cases for the child declarations.
        for decl in scope.iter_children(decl) {
            gather_specialize_cases(
                scope,
                schema,
                id,
                decl,
                data_fields,
                &constraints,
                specialize_cases,
            );
        }

        // Add a case for the current declaration.
        specialize_cases.push(SpecializeCase {
            id: id.to_owned(),
            constraints,
            size: schema.decl_size(decl.key) + schema.payload_size(decl.key),
        });
    }

    // Create match cases for each child declaration: the union of
    // tuple of constaint values and packet sizes that will specialize to this
    // declaration.
    let mut specialize_cases = Vec::new();
    for child_decl in scope.iter_children(decl) {
        gather_specialize_cases(
            scope,
            schema,
            child_decl.id().unwrap(),
            child_decl,
            data_fields,
            &HashMap::new(),
            &mut specialize_cases,
        )
    }

    // List the identifiers of fields constituting the
    // discriminant tuple.
    let ids = specialize_cases
        .iter()
        .flat_map(|case| case.constraints.keys())
        .cloned()
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();

    fn make_specialize_case(ids: &[String], case: &SpecializeCase) -> Vec<Option<ConstraintValue>> {
        ids.iter().map(|id| case.constraints.get(id).cloned()).collect::<Vec<_>>()
    }

    fn check_specialize_cases(
        ids: &[String],
        with_size: bool,
        specialize_cases: &[SpecializeCase],
    ) -> Result<(), String> {
        // Check unicity of constraints.
        let mut grouped_cases = HashMap::new();
        for case in specialize_cases {
            let constraints = make_specialize_case(ids, case);
            match grouped_cases.insert(
                (constraints, if with_size { case.size } else { analyzer::Size::Unknown }),
                case.id.clone(),
            ) {
                Some(id) if id != case.id => {
                    return Err(format!("{} and {} cannot be disambiguated", id, case.id))
                }
                _ => (),
            }
        }

        Ok(())
    }

    // Check if constraints are un-amiguous, and whether the packet size
    // is required to disambiguate.
    // TODO(henrichataing) ambiguities should be resolved by trying each
    // case until one is successfully parsed.
    check_specialize_cases(&ids, true, &specialize_cases)?;
    let with_size = check_specialize_cases(&ids, false, &specialize_cases).is_err();

    // Finally group match cases by matching child declaration.
    let mut grouped_cases = BTreeMap::new();
    for case in specialize_cases {
        let constraints = make_specialize_case(&ids, &case);
        let size = if with_size { case.size } else { analyzer::Size::Unknown };
        if constraints.iter().any(Option::is_some) || size != analyzer::Size::Unknown {
            grouped_cases
                .entry(case.id.clone())
                .or_insert(BTreeSet::new())
                .insert((constraints, size));
        }
    }

    // Build the case values and case branches.
    // The case are ordered by child declaration order.
    let mut case_values = vec![];
    let mut case_ids = vec![];
    let child_name = format_ident!("{id}Child");

    for (id, cases) in grouped_cases {
        case_ids.push(format_ident!("{id}"));
        case_values.push(
            cases
                .iter()
                .map(|(constraints, size)| {
                    let mut case = constraints
                        .iter()
                        .map(|v| match v {
                            Some(v) => quote!(#v),
                            None => quote!(_),
                        })
                        .collect::<Vec<_>>();
                    if with_size {
                        case.push(match size {
                            analyzer::Size::Static(s) => {
                                let s = proc_macro2::Literal::usize_unsuffixed(s / 8);
                                quote!(#s)
                            }
                            _ => quote!(_),
                        });
                    }
                    case
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut field_values = ids
        .iter()
        .map(|id| {
            let id = id.to_ident();
            quote!(self.#id)
        })
        .collect::<Vec<_>>();
    if with_size {
        field_values.push(quote!(self.payload.len()));
    }

    // TODO(henrichataing) the default case is necessary only if the match
    // is non-exhaustive.
    Ok(quote! {
        pub fn specialize(&self) -> Result<#child_name, DecodeError> {
            Ok(
                match ( #( #field_values ),* ) {
                    #( #( ( #( #case_values ),* ) )|* =>
                        #child_name::#case_ids(self.try_into()?), )*
                    _ => #child_name::None,
                }
            )
        }
    })
}

/// Generate code for a root packet declaration.
///
/// # Arguments
/// * `endianness` - File endianness
/// * `id` - Packet identifier.
fn generate_root_packet_decl(
    scope: &analyzer::Scope<'_>,
    schema: &analyzer::Schema,
    endianness: ast::EndiannessValue,
    id: &str,
) -> proc_macro2::TokenStream {
    let decl = scope.typedef[id];
    let name = id.to_ident();
    let child_name = format_ident!("{id}Child");

    // Return the list of fields that will appear in the generated
    // rust structs (<Packet> and <Packet>Builder).
    // The payload field _ if declared _ is handled separately.
    let data_fields = packet_data_fields(scope, decl);
    let data_field_ids = data_fields.iter().map(|f| f.id().unwrap().to_ident()).collect::<Vec<_>>();
    let data_field_types = data_fields.iter().map(|f| types::rust_type(f)).collect::<Vec<_>>();
    let data_field_borrows = data_fields
        .iter()
        .map(|f| {
            if implements_copy(scope, f) {
                quote! {}
            } else {
                quote! { & }
            }
        })
        .collect::<Vec<_>>();
    let payload_field = decl.payload().map(|_| quote! { pub payload: Vec<u8>, });
    let payload_accessor =
        decl.payload().map(|_| quote! { pub fn payload(&self) -> &[u8] { &self.payload } });

    let parser_span = format_ident!("buf");
    let mut field_parser = FieldParser::new(scope, schema, endianness, id, &parser_span);
    for field in decl.fields() {
        field_parser.add(field);
    }

    // For the implementation of decode_partial, sort the data field identifiers
    // between parsed fields (extracted from the payload), and copied fields
    // (copied from the parent).
    let mut parsed_field_ids = vec![];
    if decl.payload().is_some() {
        parsed_field_ids.push(format_ident!("payload"));
    }
    for f in &data_fields {
        let id = f.id().unwrap().to_ident();
        parsed_field_ids.push(id);
    }

    let (encode_fields, encoded_len) =
        encoder::encode(scope, schema, endianness, "buf".to_ident(), decl);

    let encode = quote! {
         fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
            #encode_fields
            Ok(())
        }
    };

    // Compute the encoded length of the packet.
    let encoded_len = quote! {
        fn encoded_len(&self) -> usize {
            #encoded_len
        }
    };

    // The implementation of decode for root packets contains the full
    // parser implementation.
    let decode = quote! {
       fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
           #field_parser
           Ok((Self { #( #parsed_field_ids, )* }, buf))
       }
    };

    // Provide the implementation of the enum listing child declarations of the
    // current declaration. This enum is only provided for declarations that
    // have child packets.
    let children_decl = scope.iter_children(decl).collect::<Vec<_>>();
    let child_struct = (!children_decl.is_empty()).then(|| {
        let children_ids = children_decl.iter().map(|decl| decl.id().unwrap().to_ident());
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #child_name {
                #( #children_ids(#children_ids), )*
                None,
            }
        }
    });

    // Provide the implementation of the specialization function.
    // The specialization function is only provided for declarations that have
    // child packets.
    let specialize = (!children_decl.is_empty())
        .then(|| generate_specialize_impl(scope, schema, decl, id, &data_fields).unwrap());

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct #name {
            #( pub #data_field_ids: #data_field_types, )*
            #payload_field
        }

        #child_struct

        impl #name {
            #specialize
            #payload_accessor

            #(
            pub fn #data_field_ids(&self) -> #data_field_borrows #data_field_types {
                #data_field_borrows self.#data_field_ids
            }
            )*
        }

        impl Packet for #name {
            #encoded_len
            #encode
            #decode
        }
    }
}

/// Generate code for a derived packet declaration
///
/// # Arguments
/// * `endianness` - File endianness
/// * `id` - Packet identifier.
fn generate_derived_packet_decl(
    scope: &analyzer::Scope<'_>,
    schema: &analyzer::Schema,
    endianness: ast::EndiannessValue,
    id: &str,
) -> proc_macro2::TokenStream {
    let decl = scope.typedef[id];
    let name = id.to_ident();
    let parent_decl = scope.get_parent(decl).unwrap();
    let parent_name = parent_decl.id().unwrap().to_ident();
    let child_name = format_ident!("{id}Child");

    // Extract all constraint values from the parent declarations.
    let all_constraints = HashMap::<String, _>::from_iter(
        scope.iter_constraints(decl).map(|c| (c.id.to_string(), c)),
    );

    let all_fields = scope.iter_fields(decl).collect::<Vec<_>>();

    // Return the list of fields that will appear in the generated
    // rust structs (<Packet> and <Packet>Builder).
    // The payload field _ if declared _ is handled separately.
    let data_fields = packet_data_fields(scope, decl);
    let data_field_ids = data_fields.iter().map(|f| f.id().unwrap().to_ident()).collect::<Vec<_>>();
    let data_field_types = data_fields.iter().map(|f| types::rust_type(f)).collect::<Vec<_>>();
    let data_field_borrows = data_fields
        .iter()
        .map(|f| {
            if implements_copy(scope, f) {
                quote! {}
            } else {
                quote! { & }
            }
        })
        .collect::<Vec<_>>();
    let payload_field = decl.payload().map(|_| quote! { pub payload: Vec<u8>, });
    let payload_accessor =
        decl.payload().map(|_| quote! { pub fn payload(&self) -> &[u8] { &self.payload } });

    let parent_data_fields = packet_data_fields(scope, parent_decl);

    // Return the list of fields that have a constant value.
    let constant_fields = packet_constant_fields(scope, decl);
    let constant_field_ids =
        constant_fields.iter().map(|f| f.id().unwrap().to_ident()).collect::<Vec<_>>();
    let constant_field_types =
        constant_fields.iter().map(|f| types::rust_type(f)).collect::<Vec<_>>();
    let constant_field_values = constant_fields.iter().map(|f| {
        let c = all_constraints.get(f.id().unwrap()).unwrap();
        constraint_value(&all_fields, c)
    });

    // Generate field parsing and serialization.
    let parser_span = format_ident!("buf");
    let mut field_parser = FieldParser::new(scope, schema, endianness, id, &parser_span);
    for field in decl.fields() {
        field_parser.add(field);
    }

    // For the implementation of decode_partial, sort the data field identifiers
    // between parsed fields (extracted from the payload), and copied fields
    // (copied from the parent).
    let mut parsed_field_ids = vec![];
    let mut copied_field_ids = vec![];
    let mut cloned_field_ids = vec![];
    if decl.payload().is_some() {
        parsed_field_ids.push(format_ident!("payload"));
    }
    for f in &data_fields {
        let id = f.id().unwrap().to_ident();
        if decl.fields().any(|ff| f.id() == ff.id()) {
            parsed_field_ids.push(id);
        } else if implements_copy(scope, f) {
            copied_field_ids.push(id);
        } else {
            cloned_field_ids.push(id);
        }
    }

    let (partial_field_serializer, field_serializer, encoded_len) =
        encoder::encode_partial(scope, schema, endianness, "buf".to_ident(), decl);

    let encode_partial = quote! {
        pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
            #partial_field_serializer
            Ok(())
        }
    };

    let encode = quote! {
         fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
            #field_serializer
            Ok(())
        }
    };

    // Compute the encoded length of the packet.
    let encoded_len = quote! {
        fn encoded_len(&self) -> usize {
            #encoded_len
        }
    };

    // Constraint checks are only run for constraints added to this declaration
    // and not parent constraints which are expected to have been validated
    // earlier.
    let constraint_checks = decl.constraints().map(|c| {
        let field_id = c.id.to_ident();
        let field_name = &c.id;
        let packet_name = id;
        let value = constraint_value(&parent_data_fields, c);
        let value_str = constraint_value_str(&parent_data_fields, c);
        quote! {
            if parent.#field_id() != #value {
                return Err(DecodeError::InvalidFieldValue {
                    packet: #packet_name,
                    field: #field_name,
                    expected: #value_str,
                    actual: format!("{:?}", parent.#field_id()),
                })
            }
        }
    });

    let decode_partial = if parent_decl.payload().is_some() {
        // Generate an implementation of decode_partial that will decode
        // data fields present in the parent payload.
        // TODO(henrichataing) add constraint validation to decode_partial,
        // return DecodeError::InvalidConstraint.
        quote! {
            fn decode_partial(parent: &#parent_name) -> Result<Self, DecodeError> {
                let mut buf: &[u8] = &parent.payload;
                #( #constraint_checks )*
                #field_parser
                if buf.is_empty() {
                    Ok(Self {
                        #( #parsed_field_ids, )*
                        #( #copied_field_ids: parent.#copied_field_ids, )*
                        #( #cloned_field_ids: parent.#cloned_field_ids.clone(), )*
                    })
                } else {
                    Err(DecodeError::TrailingBytes)
                }
            }
        }
    } else {
        // Generate an implementation of decode_partial that will only copy
        // data fields present in the parent.
        // TODO(henrichataing) add constraint validation to decode_partial,
        // return DecodeError::InvalidConstraint.
        quote! {
            fn decode_partial(parent: &#parent_name) -> Result<Self, DecodeError> {
                #( #constraint_checks )*
                Ok(Self {
                    #( #copied_field_ids: parent.#copied_field_ids, )*
                })
            }
        }
    };

    let decode =
        // The implementation of decode for derived packets relies on
        // the parent packet parser.
        quote! {
            fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
                let (parent, trailing_bytes) = #parent_name::decode(buf)?;
                let packet = Self::decode_partial(&parent)?;
                Ok((packet, trailing_bytes))
            }
        };

    // Provide the implementation of conversion helpers from
    // the current packet to its parent packets. The implementation
    // is explicit for the immediate parent, and derived using other
    // Into<> implementations for the ancestors.
    let into_parent = {
        let parent_data_field_ids = parent_data_fields.iter().map(|f| f.id().unwrap().to_ident());
        let parent_data_field_values = parent_data_fields.iter().map(|f| {
            let id = f.id().unwrap().to_ident();
            match all_constraints.get(f.id().unwrap()) {
                Some(c) => constraint_value(&parent_data_fields, c),
                None => quote! { packet.#id },
            }
        });
        if parent_decl.payload().is_some() {
            quote! {
                impl TryFrom<&#name> for #parent_name {
                    type Error = EncodeError;
                    fn try_from(packet: &#name) -> Result<#parent_name, Self::Error> {
                        let mut payload = Vec::new();
                        packet.encode_partial(&mut payload)?;
                        Ok(#parent_name {
                            #( #parent_data_field_ids: #parent_data_field_values, )*
                            payload,
                        })
                    }
                }

                impl TryFrom<#name> for #parent_name {
                    type Error = EncodeError;
                    fn try_from(packet: #name) -> Result<#parent_name, Self::Error> {
                        (&packet).try_into()
                    }
                }
            }
        } else {
            quote! {
                impl From<&#name> for #parent_name {
                    fn from(packet: &#name) -> #parent_name {
                        #parent_name {
                            #( #parent_data_field_ids: #parent_data_field_values, )*
                        }
                    }
                }

                impl From<#name> for #parent_name {
                    fn from(packet: #name) -> #parent_name {
                        (&packet).into()
                    }
                }
            }
        }
    };

    let into_ancestors = scope.iter_parents(parent_decl).map(|ancestor_decl| {
        let ancestor_name = ancestor_decl.id().unwrap().to_ident();
        quote! {
            impl TryFrom<&#name> for #ancestor_name {
                type Error = EncodeError;
                fn try_from(packet: &#name) -> Result<#ancestor_name, Self::Error> {
                    (&#parent_name::try_from(packet)?).try_into()
                }
            }

            impl TryFrom<#name> for #ancestor_name {
                type Error = EncodeError;
                fn try_from(packet: #name) -> Result<#ancestor_name, Self::Error> {
                    (&packet).try_into()
                }
            }
        }
    });

    // Provide the implementation of conversion helper from
    // the parent packet. This function is actually the parse
    // implementation. This helper is provided only if the packet has a
    // parent declaration.
    let from_parent = quote! {
        impl TryFrom<&#parent_name> for #name {
            type Error = DecodeError;
            fn try_from(parent: &#parent_name) -> Result<#name, Self::Error> {
                #name::decode_partial(&parent)
            }
        }

        impl TryFrom<#parent_name> for #name {
            type Error = DecodeError;
            fn try_from(parent: #parent_name) -> Result<#name, Self::Error> {
                (&parent).try_into()
            }
        }
    };

    // Provide the implementation of conversion helpers from
    // the ancestor packets.
    let from_ancestors = scope.iter_parents(parent_decl).map(|ancestor_decl| {
        let ancestor_name = ancestor_decl.id().unwrap().to_ident();
        quote! {
            impl TryFrom<&#ancestor_name> for #name {
                type Error = DecodeError;
                fn try_from(packet: &#ancestor_name) -> Result<#name, Self::Error> {
                    (&#parent_name::try_from(packet)?).try_into()
                }
            }

            impl TryFrom<#ancestor_name> for #name {
                type Error = DecodeError;
                fn try_from(packet: #ancestor_name) -> Result<#name, Self::Error> {
                    (&packet).try_into()
                }
            }
        }
    });

    // Provide the implementation of the enum listing child declarations of the
    // current declaration. This enum is only provided for declarations that
    // have child packets.
    let children_decl = scope.iter_children(decl).collect::<Vec<_>>();
    let child_struct = (!children_decl.is_empty()).then(|| {
        let children_ids = children_decl.iter().map(|decl| decl.id().unwrap().to_ident());
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub enum #child_name {
                #( #children_ids(#children_ids), )*
                None,
            }
        }
    });

    // Provide the implementation of the specialization function.
    // The specialization function is only provided for declarations that have
    // child packets.
    let specialize = (!children_decl.is_empty())
        .then(|| generate_specialize_impl(scope, schema, decl, id, &data_fields).unwrap());

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct #name {
            #( pub #data_field_ids: #data_field_types, )*
            #payload_field
        }

        #into_parent
        #from_parent
        #( #into_ancestors )*
        #( #from_ancestors )*

        #child_struct

        impl #name {
            #specialize
            #decode_partial
            #encode_partial
            #payload_accessor

            #(
            pub fn #data_field_ids(&self) -> #data_field_borrows #data_field_types {
                #data_field_borrows self.#data_field_ids
            }
            )*

            #(
            pub fn #constant_field_ids(&self) -> #constant_field_types {
                #constant_field_values
            }
            )*
        }

        impl Packet for #name {
            #encoded_len
            #encode
            #decode
        }
    }
}

/// Generate an enum declaration.
///
/// # Arguments
/// * `id` - Enum identifier.
/// * `tags` - List of enum tags.
/// * `width` - Width of the backing type of the enum, in bits.
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
        syn::parse_str::<syn::LitInt>(&format!("{value:#x}")).unwrap()
    }

    // Backing type for the enum.
    let backing_type = types::Integer::new(width);
    let backing_type_str = proc_macro2::Literal::string(&format!("u{}", backing_type.width));
    let range_max = scalar_max(width);
    let default_tag = enum_default_tag(tags);
    let is_open = default_tag.is_some();
    let is_complete = enum_is_complete(tags, scalar_max(width));
    let is_primitive = enum_is_primitive(tags);
    let name = id.to_ident();

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
        .map(|w| syn::parse_str::<syn::Type>(&format!("i{w}")).unwrap());
    let derived_unsigned_into_types = [8, 16, 32, 64]
        .into_iter()
        .filter(|w| *w >= width && *w != backing_type.width)
        .map(|w| syn::parse_str::<syn::Type>(&format!("u{w}")).unwrap());
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
            fn try_from(value: #backing_type) -> Result<Self, Self::Error> {
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
fn generate_custom_field_decl(
    endianness: ast::EndiannessValue,
    id: &str,
    width: usize,
) -> proc_macro2::TokenStream {
    let name = id;
    let id = id.to_ident();
    let backing_type = types::Integer::new(width);
    let backing_type_str = proc_macro2::Literal::string(&format!("u{}", backing_type.width));
    let max_value = mask_bits(width, &format!("u{}", backing_type.width));
    let size = proc_macro2::Literal::usize_unsuffixed(width / 8);

    let read_value = types::get_uint(endianness, width, &format_ident!("buf"));
    let read_value = if [8, 16, 32, 64].contains(&width) {
        quote! { #read_value.into() }
    } else {
        // The value is masked when read, and the conversion must succeed.
        quote! { (#read_value).try_into().unwrap() }
    };

    let write_value = types::put_uint(
        endianness,
        &quote! { #backing_type::from(self) },
        width,
        &format_ident!("buf"),
    );

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

        impl Packet for #id {
            fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
                if buf.len() < #size {
                    return Err(DecodeError::InvalidLengthError {
                        obj: #name,
                        wanted: #size,
                        got: buf.len(),
                    })
                }

                Ok((#read_value, buf))
            }

            fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
                #write_value;
                Ok(())
            }

            fn encoded_len(&self) -> usize {
                #size
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
                fn try_from(value: #backing_type) -> Result<Self, Self::Error> {
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
    schema: &analyzer::Schema,
    file: &ast::File,
    decl: &ast::Decl,
) -> proc_macro2::TokenStream {
    match &decl.desc {
        ast::DeclDesc::Packet { id, .. } | ast::DeclDesc::Struct { id, .. } => {
            match scope.get_parent(decl) {
                None => generate_root_packet_decl(scope, schema, file.endianness.value, id),
                Some(_) => generate_derived_packet_decl(scope, schema, file.endianness.value, id),
            }
        }
        ast::DeclDesc::Enum { id, tags, width } => generate_enum_decl(id, tags, *width),
        ast::DeclDesc::CustomField { id, width: Some(width), .. } => {
            generate_custom_field_decl(file.endianness.value, id, *width)
        }
        ast::DeclDesc::CustomField { .. } => {
            // No need to generate anything for a custom field,
            // we just assume it will be in scope.
            quote!()
        }
        _ => todo!("unsupported Decl::{:?}", decl),
    }
}

/// Generate Rust code from an AST.
///
/// The code is not formatted, pipe it through `rustfmt` to get
/// readable source code.
pub fn generate_tokens(
    sources: &ast::SourceDatabase,
    file: &ast::File,
    custom_fields: &[String],
) -> proc_macro2::TokenStream {
    let source = sources.get(file.file).expect("could not read source");
    let preamble = preamble::generate(Path::new(source.name()));
    let scope = analyzer::Scope::new(file).expect("could not create scope");
    let schema = analyzer::Schema::new(file);
    let custom_fields = custom_fields.iter().map(|custom_field| {
        syn::parse_str::<syn::Path>(custom_field)
            .unwrap_or_else(|err| panic!("invalid path '{custom_field}': {err:?}"))
    });
    let decls = file.declarations.iter().map(|decl| generate_decl(&scope, &schema, file, decl));
    quote! {
        #preamble
        #(use #custom_fields;)*

        #(#decls)*
    }
}

/// Generate formatted Rust code from an AST.
///
/// The code is not formatted, pipe it through `rustfmt` to get
/// readable source code.
pub fn generate(
    sources: &ast::SourceDatabase,
    file: &ast::File,
    custom_fields: &[String],
) -> String {
    let syntax_tree =
        syn::parse2(generate_tokens(sources, file, custom_fields)).expect("Could not parse code");
    prettyplease::unparse(&syntax_tree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer;
    use crate::ast;
    use crate::parser::parse_inline;
    use crate::test_utils::{assert_snapshot_eq, format_rust};
    use paste::paste;

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
                    let file = parse_inline(&mut db, "test", code).unwrap();
                    let file = analyzer::analyze(&file).unwrap();
                    let actual_code = generate(&db, &file, &[]);
                    assert_snapshot_eq(
                        &format!("tests/generated/rust/{name}_{endianness}.rs"),
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
        packet_decl_array_dynamic_element_size,
        "
          struct Foo {
            inner: 8[]
          }
          packet Bar {
            _elementsize_(x): 5,
            padding: 3,
            x: Foo[]
          }
        "
    );

    test_pdl!(
        packet_decl_array_dynamic_element_size_dynamic_size,
        "
          struct Foo {
            inner: 8[]
          }
          packet Bar {
            _size_(x): 4,
            _elementsize_(x): 4,
            x: Foo[]
          }
        "
    );

    test_pdl!(
        packet_decl_array_dynamic_element_size_dynamic_count,
        "
          struct Foo {
            inner: 8[]
          }
          packet Bar {
            _count_(x): 4,
            _elementsize_(x): 4,
            x: Foo[]
          }
        "
    );

    test_pdl!(
        packet_decl_array_dynamic_element_size_static_count,
        "
          struct Foo {
            inner: 8[]
          }
          packet Bar {
            _elementsize_(x): 5,
            padding: 3,
            x: Foo[4]
          }
        "
    );

    test_pdl!(
        packet_decl_array_dynamic_element_size_static_count_1,
        "
          struct Foo {
            inner: 8[]
          }
          packet Bar {
            _elementsize_(x): 5,
            padding: 3,
            x: Foo[1]
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

    test_pdl!(
        reserved_identifier,
        "
          packet Test {
            type: 8,
          }
        "
    );

    test_pdl!(
        payload_with_size_modifier,
        "
        packet Test {
            _size_(_payload_): 8,
            _payload_ : [+1],
        }
        "
    );

    test_pdl!(
        struct_decl_child_structs,
        "
          enum Enum16 : 16 {
            A = 1,
            B = 2,
          }

          struct Foo {
              a: 8,
              b: Enum16,
              _size_(_payload_): 8,
              _payload_
          }

          struct Bar : Foo (a = 100) {
              x: 8,
          }

          struct Baz : Foo (b = B) {
              y: 16,
          }
        "
    );

    test_pdl!(
        struct_decl_grand_children,
        "
          enum Enum16 : 16 {
            A = 1,
            B = 2,
          }

          struct Parent {
              foo: Enum16,
              bar: Enum16,
              baz: Enum16,
              _size_(_payload_): 8,
              _payload_
          }

          struct Child : Parent (foo = A) {
              quux: Enum16,
              _payload_,
          }

          struct GrandChild : Child (bar = A, quux = A) {
              _body_,
          }

          struct GrandGrandChild : GrandChild (baz = A) {
              _body_,
          }
        "
    );
}
