//! Rust no-allocation backend
//!
//! The motivation for this backend is to be a more "idiomatic" backend than
//! the existing backend. Specifically, it should
//! 1. Use lifetimes, not reference counting
//! 2. Avoid expensive memory copies unless needed
//! 3. Use the intermediate Schema rather than doing all the logic from scratch
//!
//! One notable consequence is that we avoid .specialize(), as it has "magic" behavior
//! not defined in the spec. Instead we mimic the C++ approach of calling tryParse() and
//! getting a Result<> back.

mod computed_values;
mod enums;
mod packet_parser;
mod packet_serializer;
pub mod test;
mod utils;

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::ast;
use crate::parser;

use self::{
    enums::generate_enum, packet_parser::generate_packet,
    packet_serializer::generate_packet_serializer,
};

use super::intermediate::Schema;

pub fn generate(file: &parser::ast::File, schema: &Schema) -> Result<String, String> {
    match file.endianness.value {
        ast::EndiannessValue::LittleEndian => {}
        _ => unimplemented!("Only little_endian endianness supported"),
    };

    let mut out = String::new();

    out.push_str(include_str!("preamble.rs"));

    let mut children = HashMap::<&str, Vec<&str>>::new();
    for decl in &file.declarations {
        match &decl.desc {
            ast::DeclDesc::Packet { id, parent_id: Some(parent_id), .. }
            | ast::DeclDesc::Struct { id, parent_id: Some(parent_id), .. } => {
                children.entry(parent_id.as_str()).or_default().push(id.as_str());
            }
            _ => {}
        }
    }

    let declarations = file
        .declarations
        .iter()
        .map(|decl| generate_decl(decl, schema, &children))
        .collect::<Result<TokenStream, _>>()?;

    out.push_str(
        &quote! {
            #declarations
        }
        .to_string(),
    );

    Ok(out)
}

fn generate_decl(
    decl: &parser::ast::Decl,
    schema: &Schema,
    children: &HashMap<&str, Vec<&str>>,
) -> Result<TokenStream, String> {
    match &decl.desc {
        ast::DeclDesc::Enum { id, tags, width, .. } => Ok(generate_enum(id, tags, *width)),
        ast::DeclDesc::Packet { id, fields, parent_id, .. }
        | ast::DeclDesc::Struct { id, fields, parent_id, .. } => {
            let parser = generate_packet(
                id,
                fields,
                parent_id.as_deref(),
                schema,
                &schema.packets_and_structs[id.as_str()],
            )?;
            let serializer = generate_packet_serializer(
                id,
                parent_id.as_deref(),
                fields,
                schema,
                &schema.packets_and_structs[id.as_str()],
                children,
            );
            Ok(quote! {
                #parser
                #serializer
            })
        }
        _ => unimplemented!("Unsupported decl type"),
    }
}
