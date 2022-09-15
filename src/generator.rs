// The `format-push-string` lint was briefly enabled present in Rust
// 1.62. It is now moved the disabled "restriction" category instead.
// See https://github.com/rust-lang/rust-clippy/issues/9077 for the
// problems with this lint.
//
// Remove this when we use Rust 1.63 or later.
#![allow(clippy::format_push_string)]

use crate::ast;
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::path::Path;
use syn::parse_quote;

/// Generate a block of code.
///
/// Like `quote!`, but the code block will be followed by an empty
/// line of code. This makes the generated code more readable.
macro_rules! quote_block {
    ($($tt:tt)*) => {
        format!("{}\n\n", quote!($($tt)*))
    }
}

/// Generate the file preamble.
fn generate_preamble(path: &Path) -> String {
    let mut code = String::new();
    let filename = path.file_name().unwrap().to_str().expect("non UTF-8 filename");
    code.push_str(&format!("// @generated rust packets from {filename}\n\n"));

    code.push_str(&quote_block! {
        use bytes::{BufMut, Bytes, BytesMut};
        use num_derive::{FromPrimitive, ToPrimitive};
        use num_traits::{FromPrimitive, ToPrimitive};
        use std::convert::{TryFrom, TryInto};
        use std::fmt;
        use std::sync::Arc;
        use thiserror::Error;
    });

    code.push_str(&quote_block! {
        type Result<T> = std::result::Result<T, Error>;
    });

    code.push_str(&quote_block! {
        #[derive(Debug, Error)]
        pub enum Error {
            #[error("Packet parsing failed")]
            InvalidPacketError,
            #[error("{field} was {value:x}, which is not known")]
            ConstraintOutOfBounds { field: String, value: u64 },
            #[error("when parsing {obj}.{field} needed length of {wanted} but got {got}")]
            InvalidLengthError { obj: String, field: String, wanted: usize, got: usize },
            #[error("Due to size restrictions a struct could not be parsed.")]
            ImpossibleStructError,
            #[error("when parsing field {obj}.{field}, {value} is not a valid {type_} value")]
            InvalidEnumValueError { obj: String, field: String, value: u64, type_: String },
        }
    });

    code.push_str(&quote_block! {
        #[derive(Debug, Error)]
        #[error("{0}")]
        pub struct TryFromError(&'static str);
    });

    code.push_str(&quote_block! {
        pub trait Packet {
            fn to_bytes(self) -> Bytes;
            fn to_vec(self) -> Vec<u8>;
        }
    });

    code
}

/// Round up the bit width to a Rust integer size.
fn round_bit_width(width: usize) -> usize {
    match width {
        8 => 8,
        16 => 16,
        24 | 32 => 32,
        40 | 48 | 56 | 64 => 64,
        _ => todo!("unsupported field width: {width}"),
    }
}

/// Generate a Rust unsigned integer type large enough to hold
/// integers of the given bit width.
fn type_for_width(width: usize) -> syn::Type {
    let rounded_width = round_bit_width(width);
    syn::parse_str(&format!("u{rounded_width}")).unwrap()
}

fn generate_field(field: &ast::Field, visibility: syn::Visibility) -> proc_macro2::TokenStream {
    match field {
        ast::Field::Scalar { id, width, .. } => {
            let field_name = format_ident!("{id}");
            let field_type = type_for_width(*width);
            quote! {
                #visibility #field_name: #field_type
            }
        }
        _ => todo!("unsupported field: {:?}", field),
    }
}

fn generate_field_getter(packet_name: &syn::Ident, field: &ast::Field) -> proc_macro2::TokenStream {
    match field {
        ast::Field::Scalar { id, width, .. } => {
            // TODO(mgeisler): refactor with generate_field above.
            let getter_name = format_ident!("get_{id}");
            let field_name = format_ident!("{id}");
            let field_type = type_for_width(*width);
            quote! {
                pub fn #getter_name(&self) -> #field_type {
                    self.#packet_name.as_ref().#field_name
                }
            }
        }
        _ => todo!("unsupported field: {:?}", field),
    }
}

/// Mask and rebind the field value (if necessary).
fn mask_field_value(field: &ast::Field) -> Option<proc_macro2::TokenStream> {
    match field {
        ast::Field::Scalar { id, width, .. } => {
            let field_name = format_ident!("{id}");
            let type_width = round_bit_width(*width);
            if *width != type_width {
                let mask =
                    syn::parse_str::<syn::LitInt>(&format!("{:#x}", (1u64 << *width) - 1)).unwrap();
                Some(quote! {
                    let #field_name = #field_name & #mask;
                })
            } else {
                None
            }
        }
        _ => todo!("unsupported field: {:?}", field),
    }
}

fn generate_field_parser(
    endianness_value: ast::EndiannessValue,
    packet_name: &str,
    field: &ast::Field,
    offset: usize,
) -> proc_macro2::TokenStream {
    match field {
        ast::Field::Scalar { id, width, .. } => {
            let field_name = format_ident!("{id}");
            let type_width = round_bit_width(*width);
            let field_type = type_for_width(*width);

            let getter = match endianness_value {
                ast::EndiannessValue::BigEndian => format_ident!("from_be_bytes"),
                ast::EndiannessValue::LittleEndian => format_ident!("from_le_bytes"),
            };

            // We need the padding on the MSB side of the payload, so
            // for big-endian, we need to padding on the left, for
            // little-endian we need it on the right.
            let padding = vec![syn::Index::from(0); (type_width - width) / 8];
            let (padding_before, padding_after) = match endianness_value {
                ast::EndiannessValue::BigEndian => (padding, vec![]),
                ast::EndiannessValue::LittleEndian => (vec![], padding),
            };

            let wanted_len = syn::Index::from(offset + width / 8);
            let indices = (offset..offset + width / 8).map(syn::Index::from);
            let mask = mask_field_value(field);

            quote! {
                // TODO(mgeisler): call a function instead to avoid
                // generating so much code for this.
                if bytes.len() < #wanted_len {
                    return Err(Error::InvalidLengthError {
                        obj: #packet_name.to_string(),
                        field: #id.to_string(),
                        wanted: #wanted_len,
                        got: bytes.len(),
                    });
                }
                let #field_name = #field_type::#getter([
                    #(#padding_before,)* #(bytes[#indices]),* #(, #padding_after)*
                ]);
                #mask
            }
        }
        _ => todo!("unsupported field: {:?}", field),
    }
}

fn generate_field_writer(
    file: &ast::File,
    field: &ast::Field,
    offset: usize,
) -> proc_macro2::TokenStream {
    match field {
        ast::Field::Scalar { id, width, .. } => {
            let field_name = format_ident!("{id}");
            let start = syn::Index::from(offset);
            let end = syn::Index::from(offset + width / 8);
            let byte_width = syn::Index::from(width / 8);
            let mask = mask_field_value(field);
            let writer = match file.endianness.value {
                ast::EndiannessValue::BigEndian => format_ident!("to_be_bytes"),
                ast::EndiannessValue::LittleEndian => format_ident!("to_le_bytes"),
            };
            quote! {
                let #field_name = self.#field_name;
                #mask
                buffer[#start..#end].copy_from_slice(&#field_name.#writer()[0..#byte_width]);
            }
        }
        _ => todo!("unsupported field: {:?}", field),
    }
}

fn get_field_size(field: &ast::Field) -> usize {
    match field {
        ast::Field::Scalar { width, .. } => width / 8,
        _ => todo!("unsupported field: {:?}", field),
    }
}

/// Generate code for an `ast::Decl::Packet` enum value.
fn generate_packet_decl(
    file: &ast::File,
    packets: &HashMap<&str, &ast::Decl>,
    child_ids: &[&str],
    id: &str,
    fields: &[ast::Field],
    parent_id: &Option<String>,
) -> String {
    // TODO(mgeisler): use the convert_case crate to convert between
    // `FooBar` and `foo_bar` in the code below.
    let mut code = String::new();

    let has_children = !child_ids.is_empty();
    let child_idents = child_ids.iter().map(|id| format_ident!("{id}")).collect::<Vec<_>>();

    let ident = format_ident!("{}", id.to_lowercase());
    let data_child_ident = format_ident!("{id}DataChild");
    let child_decl_packet_name =
        child_idents.iter().map(|ident| format_ident!("{ident}Packet")).collect::<Vec<_>>();
    let child_name = format_ident!("{id}Child");
    if has_children {
        let child_data_idents = child_idents.iter().map(|ident| format_ident!("{ident}Data"));
        code.push_str(&quote_block! {
            #[derive(Debug)]
            enum #data_child_ident {
                #(#child_idents(Arc<#child_data_idents>),)*
                None,
            }

            impl #data_child_ident {
                fn get_total_size(&self) -> usize {
                    // TODO(mgeisler): use Self instad of #data_child_ident.
                    match self {
                        #(#data_child_ident::#child_idents(value) => value.get_total_size(),)*
                        #data_child_ident::None => 0,
                    }
                }
            }

            #[derive(Debug)]
            pub enum #child_name {
                #(#child_idents(#child_decl_packet_name),)*
                None,
            }
        });
    }

    let data_name = format_ident!("{id}Data");
    let child_field = has_children.then(|| {
        quote! {
            child: #data_child_ident,
        }
    });
    let plain_fields = fields.iter().map(|field| generate_field(field, parse_quote!()));
    code.push_str(&quote_block! {
        #[derive(Debug)]
        struct #data_name {
            #(#plain_fields,)*
            #child_field
        }
    });

    let parent = parent_id.as_ref().map(|parent_id| match packets.get(parent_id.as_str()) {
        Some(ast::Decl::Packet { id, .. }) => {
            let parent_ident = format_ident!("{}", id.to_lowercase());
            let parent_data = format_ident!("{id}Data");
            quote! {
                #parent_ident: Arc<#parent_data>,
            }
        }
        _ => panic!("Could not find {parent_id}"),
    });

    let packet_name = format_ident!("{id}Packet");
    code.push_str(&quote_block! {
        #[derive(Debug, Clone)]
        pub struct #packet_name {
            #parent
            #ident: Arc<#data_name>,
        }
    });

    let builder_name = format_ident!("{id}Builder");
    let pub_fields = fields.iter().map(|field| generate_field(field, parse_quote!(pub)));
    code.push_str(&quote_block! {
        #[derive(Debug)]
        pub struct #builder_name {
            #(#pub_fields,)*
        }
    });

    // TODO(mgeisler): use the `Buf` trait instead of tracking
    // the offset manually.
    let mut offset = 0;
    let field_parsers = fields.iter().map(|field| {
        let parser = generate_field_parser(file.endianness.value, id, field, offset);
        offset += get_field_size(field);
        parser
    });
    let field_names = fields
        .iter()
        .map(|field| match field {
            ast::Field::Scalar { id, .. } => format_ident!("{id}"),
            _ => todo!("unsupported field: {:?}", field),
        })
        .collect::<Vec<_>>();
    let mut offset = 0;
    let field_writers = fields.iter().map(|field| {
        let writer = generate_field_writer(file, field, offset);
        offset += get_field_size(field);
        writer
    });

    let total_field_size = syn::Index::from(fields.iter().map(get_field_size).sum::<usize>());
    let get_size_adjustment = (total_field_size.index > 0).then(|| {
        Some(quote! {
            let ret = ret + #total_field_size;
        })
    });

    code.push_str(&quote_block! {
        impl #data_name {
            fn conforms(bytes: &[u8]) -> bool {
                // TODO(mgeisler): return Boolean expression directly.
                // TODO(mgeisler): skip when total_field_size == 0.
                if bytes.len() < #total_field_size {
                    return false;
                }
                true
            }

            fn parse(bytes: &[u8]) -> Result<Self> {
                #(#field_parsers)*
                Ok(Self { #(#field_names),* })
            }

            fn write_to(&self, buffer: &mut BytesMut) {
                #(#field_writers)*
            }

            fn get_total_size(&self) -> usize {
                self.get_size()
            }

            fn get_size(&self) -> usize {
                let ret = 0;
                #get_size_adjustment
                ret
            }
        }
    });

    code.push_str(&quote_block! {
        impl Packet for #packet_name {
            fn to_bytes(self) -> Bytes {
                let mut buffer = BytesMut::new();
                buffer.resize(self.#ident.get_total_size(), 0);
                self.#ident.write_to(&mut buffer);
                buffer.freeze()
            }
            fn to_vec(self) -> Vec<u8> {
                self.to_bytes().to_vec()
            }
        }
        impl From<#packet_name> for Bytes {
            fn from(packet: #packet_name) -> Self {
                packet.to_bytes()
            }
        }
        impl From<#packet_name> for Vec<u8> {
            fn from(packet: #packet_name) -> Self {
                packet.to_vec()
            }
        }
    });

    let specialize = has_children.then(|| {
        quote! {
            pub fn specialize(&self) -> #child_name {
                match &self.#ident.child {
                    #(#data_child_ident::#child_idents(_) =>
                      #child_name::#child_idents(
                          #child_decl_packet_name::new(self.#ident.clone()).unwrap()),)*
                    #data_child_ident::None => #child_name::None,
                }
            }
        }
    });
    let field_getters = fields.iter().map(|field| generate_field_getter(&ident, field));
    code.push_str(&quote_block! {
        impl #packet_name {
            pub fn parse(bytes: &[u8]) -> Result<Self> {
                Ok(Self::new(Arc::new(#data_name::parse(bytes)?)).unwrap())
            }

            #specialize

            fn new(root: Arc<#data_name>) -> std::result::Result<Self, &'static str> {
                let #ident = root;
                Ok(Self { #ident })
            }

            #(#field_getters)*
        }
    });

    let child = has_children.then(|| {
        quote! {
            child: #data_child_ident::None,
        }
    });
    code.push_str(&quote_block! {
        impl #builder_name {
            pub fn build(self) -> #packet_name {
                let #ident = Arc::new(#data_name {
                    #(#field_names: self.#field_names,)*
                    #child
                });
                #packet_name::new(#ident).unwrap()
            }
        }
    });

    code
}

fn generate_decl(
    file: &ast::File,
    packets: &HashMap<&str, &ast::Decl>,
    children: &HashMap<&str, Vec<&str>>,
    decl: &ast::Decl,
) -> String {
    let empty: Vec<&str> = vec![];
    match decl {
        ast::Decl::Packet { id, fields, parent_id, .. } => generate_packet_decl(
            file,
            packets,
            children.get(id.as_str()).unwrap_or(&empty),
            id,
            fields,
            parent_id,
        ),
        _ => todo!("unsupported Decl::{:?}", decl),
    }
}

/// Generate Rust code from an AST.
///
/// The code is not formatted, pipe it through `rustfmt` to get
/// readable source code.
pub fn generate_rust(sources: &ast::SourceDatabase, file: &ast::File) -> String {
    let source = sources.get(file.file).expect("could not read source");

    let mut children = HashMap::new();
    let mut packets = HashMap::new();
    for decl in &file.declarations {
        if let ast::Decl::Packet { id, parent_id, .. } = decl {
            packets.insert(id.as_str(), decl);
            if let Some(parent_id) = parent_id {
                children.entry(parent_id.as_str()).or_insert_with(Vec::new).push(id.as_str());
            }
        }
    }

    let mut code = String::new();

    code.push_str(&generate_preamble(Path::new(source.name())));

    for decl in &file.declarations {
        code.push_str(&generate_decl(file, &packets, &children, decl));
        code.push_str("\n\n");
    }

    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;
    use crate::parser::parse_inline;
    use crate::test_utils::{assert_eq_with_diff, assert_snapshot_eq, rustfmt};

    /// Parse a string fragment as a PDL file.
    ///
    /// # Panics
    ///
    /// Panics on parse errors.
    pub fn parse_str(text: &str) -> ast::File {
        let mut db = ast::SourceDatabase::new();
        parse_inline(&mut db, String::from("stdin"), String::from(text)).expect("parse error")
    }

    #[test]
    fn test_generate_preamble() {
        let actual_code = generate_preamble(Path::new("some/path/foo.pdl"));
        assert_snapshot_eq("tests/generated/preamble.rs", &rustfmt(&actual_code));
    }

    #[test]
    fn test_generate_packet_decl_empty() {
        let file = parse_str(
            r#"
              big_endian_packets
              packet Foo {}
            "#,
        );
        let packets = HashMap::new();
        let children = HashMap::new();
        let decl = &file.declarations[0];
        let actual_code = generate_decl(&file, &packets, &children, decl);
        assert_snapshot_eq("tests/generated/packet_decl_empty.rs", &rustfmt(&actual_code));
    }

    #[test]
    fn test_generate_packet_decl_little_endian() {
        let file = parse_str(
            r#"
              little_endian_packets

              packet Foo {
                x: 8,
                y: 16,
                z: 24,
              }
            "#,
        );
        let packets = HashMap::new();
        let children = HashMap::new();
        let decl = &file.declarations[0];
        let actual_code = generate_decl(&file, &packets, &children, decl);
        assert_snapshot_eq(
            "tests/generated/packet_decl_simple_little_endian.rs",
            &rustfmt(&actual_code),
        );
    }

    #[test]
    fn test_generate_packet_decl_simple_big_endian() {
        let file = parse_str(
            r#"
              big_endian_packets

              packet Foo {
                x: 8,
                y: 16,
                z: 24,
              }
            "#,
        );
        let packets = HashMap::new();
        let children = HashMap::new();
        let decl = &file.declarations[0];
        let actual_code = generate_decl(&file, &packets, &children, decl);
        assert_snapshot_eq(
            "tests/generated/packet_decl_simple_big_endian.rs",
            &rustfmt(&actual_code),
        );
    }

    // Assert that an expression equals the given expression.
    //
    // Both expressions are wrapped in a `main` function (so we can
    // format it with `rustfmt`) and a diff is be shown if they
    // differ.
    #[track_caller]
    fn assert_expr_eq(left: proc_macro2::TokenStream, right: proc_macro2::TokenStream) {
        let left = quote! {
            fn main() { #left }
        };
        let right = quote! {
            fn main() { #right }
        };
        assert_eq_with_diff(
            "left",
            &rustfmt(&left.to_string()),
            "right",
            &rustfmt(&right.to_string()),
        );
    }

    #[test]
    fn test_mask_field_value() {
        let loc = ast::SourceRange::default();
        let field = ast::Field::Scalar { loc, id: String::from("a"), width: 8 };
        assert_eq!(mask_field_value(&field).map(|m| m.to_string()), None);

        let field = ast::Field::Scalar { loc, id: String::from("a"), width: 24 };
        assert_expr_eq(mask_field_value(&field).unwrap(), quote! { let a = a & 0xffffff; });
    }

    #[test]
    fn test_generate_field_parser_no_padding() {
        let loc = ast::SourceRange::default();
        let field = ast::Field::Scalar { loc, id: String::from("a"), width: 8 };

        assert_expr_eq(
            generate_field_parser(ast::EndiannessValue::BigEndian, "Foo", &field, 10),
            quote! {
                if bytes.len() < 11 {
                    return Err(Error::InvalidLengthError {
                        obj: "Foo".to_string(),
                        field: "a".to_string(),
                        wanted: 11,
                        got: bytes.len(),
                    });
                }
                let a = u8::from_be_bytes([bytes[10]]);
            },
        );
    }

    #[test]
    fn test_generate_field_parser_little_endian_padding() {
        // Test with width != type width.
        let loc = ast::SourceRange::default();
        let field = ast::Field::Scalar { loc, id: String::from("a"), width: 24 };
        assert_expr_eq(
            generate_field_parser(ast::EndiannessValue::LittleEndian, "Foo", &field, 10),
            quote! {
                if bytes.len() < 13 {
                    return Err(Error::InvalidLengthError {
                        obj: "Foo".to_string(),
                        field: "a".to_string(),
                        wanted: 13,
                        got: bytes.len(),
                    });
                }
                let a = u32::from_le_bytes([bytes[10], bytes[11], bytes[12], 0]);
                let a = a & 0xffffff;
            },
        );
    }

    #[test]
    fn test_generate_field_parser_big_endian_padding() {
        // Test with width != type width.
        let loc = ast::SourceRange::default();
        let field = ast::Field::Scalar { loc, id: String::from("a"), width: 24 };
        assert_expr_eq(
            generate_field_parser(ast::EndiannessValue::BigEndian, "Foo", &field, 10),
            quote! {
                if bytes.len() < 13 {
                    return Err(Error::InvalidLengthError {
                        obj: "Foo".to_string(),
                        field: "a".to_string(),
                        wanted: 13,
                        got: bytes.len(),
                    });
                }
                let a = u32::from_be_bytes([0, bytes[10], bytes[11], bytes[12]]);
                let a = a & 0xffffff;
            },
        );
    }
}
