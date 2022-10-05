//! Rust compiler backend.

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

mod chunk;
mod field;
mod preamble;
mod types;

use chunk::Chunk;
use field::Field;

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

/// Find byte indices covering `offset..offset+width` bits.
pub fn get_field_range(offset: usize, width: usize) -> std::ops::Range<usize> {
    let start = offset / 8;
    let mut end = (offset + width) / 8;
    if (offset + width) % 8 != 0 {
        end += 1;
    }
    start..end
}

fn generate_chunk_write_field_adjustments(chunk: &[ast::Field]) -> proc_macro2::TokenStream {
    // Work directly with the field name if we are writing a single
    // field. This generates simpler code.
    if let [ast::Field::Scalar { id, .. }] = chunk {
        // If there is a single field in the chunk, then we don't have to
        // shift, mask, or cast.
        let field_name = format_ident!("{id}");
        return quote! {
            let #field_name = self.#field_name;
        };
    }

    let chunk_fields = chunk.iter().map(Field::from).collect::<Vec<_>>();
    let chunk_width = Chunk::new(&chunk_fields).get_width();
    let chunk_type = types::Integer::new(chunk_width);

    let mut field_parsers = Vec::new();
    let mut field_offset = 0;
    for field in chunk {
        match field {
            ast::Field::Scalar { id, width, .. } => {
                let field_name = format_ident!("{id}");
                let field_type = types::Integer::new(*width);

                let mut field = quote! {
                    self.#field_name
                };

                if field_type.width < chunk_type.width {
                    field = quote! {
                        (#field as #chunk_type)
                    };
                }

                if *width < field_type.width {
                    let bit_mask = mask_bits(*width);
                    field = quote! {
                        (#field & #bit_mask)
                    };
                }

                if field_offset > 0 {
                    let field_offset = syn::Index::from(field_offset);
                    let op = syn::parse_str::<syn::BinOp>("<<").unwrap();
                    field = quote! {
                        (#field #op #field_offset)
                    };
                }

                field_offset += width;
                field_parsers.push(quote! {
                    let chunk = chunk | #field;
                });
            }
            _ => todo!("unsupported field: {:?}", field),
        }
    }

    quote! {
        let chunk = 0;
        #(#field_parsers)*
    }
}

/// Generate a bit-mask which masks out `n` least significant bits.
pub fn mask_bits(n: usize) -> syn::LitInt {
    syn::parse_str::<syn::LitInt>(&format!("{:#x}", (1u64 << n) - 1)).unwrap()
}

fn generate_chunk_write(
    endianness_value: ast::EndiannessValue,
    offset: usize,
    chunk: &[ast::Field],
) -> proc_macro2::TokenStream {
    let writer = match endianness_value {
        ast::EndiannessValue::BigEndian => format_ident!("to_be_bytes"),
        ast::EndiannessValue::LittleEndian => format_ident!("to_le_bytes"),
    };

    let chunk_fields = chunk.iter().map(Field::from).collect::<Vec<_>>();
    let chunk_width = Chunk::new(&chunk_fields).get_width();
    let chunk_name = Chunk::new(&chunk_fields).get_name();
    assert!(chunk_width % 8 == 0, "Chunks must have a byte size, got width: {chunk_width}");

    let range = get_field_range(offset, chunk_width);
    let start = syn::Index::from(range.start);
    let end = syn::Index::from(range.end);
    // TODO(mgeisler): let slice = (chunk_type_width > chunk_width).then( ... )
    let chunk_byte_width = syn::Index::from(chunk_width / 8);
    quote! {
        buffer[#start..#end].copy_from_slice(&#chunk_name.#writer()[0..#chunk_byte_width]);
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
    let plain_fields = fields.iter().map(|field| Field::from(field).generate_decl(parse_quote!()));
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
    let pub_fields = fields.iter().map(|field| Field::from(field).generate_decl(parse_quote!(pub)));
    code.push_str(&quote_block! {
        #[derive(Debug)]
        pub struct #builder_name {
            #(#pub_fields,)*
        }
    });

    let mut chunk_width = 0;
    let chunks = fields.split_inclusive(|field| {
        chunk_width += Field::from(field).get_width();
        chunk_width % 8 == 0
    });
    let mut field_parsers = Vec::new();
    let mut field_writers = Vec::new();
    let mut offset = 0;
    for chunk in chunks {
        let chunk_fields = chunk.iter().map(Field::from).collect::<Vec<_>>();
        field_parsers.push(Chunk::new(&chunk_fields).generate_read(
            id,
            file.endianness.value,
            offset,
        ));

        field_writers.push(generate_chunk_write_field_adjustments(chunk));
        field_writers.push(generate_chunk_write(file.endianness.value, offset, chunk));

        offset += Chunk::new(&chunk_fields).get_width();
    }

    let field_names = fields.iter().map(|field| Field::from(field).get_ident()).collect::<Vec<_>>();

    let chunk_fields = fields.iter().map(Field::from).collect::<Vec<_>>();
    let packet_size_bits = Chunk::new(&chunk_fields).get_width();
    if packet_size_bits % 8 != 0 {
        panic!("packet {id} does not end on a byte boundary, size: {packet_size_bits} bits",);
    }
    let packet_size_bytes = syn::Index::from(packet_size_bits / 8);
    let get_size_adjustment = (packet_size_bytes.index > 0).then(|| {
        Some(quote! {
            let ret = ret + #packet_size_bytes;
        })
    });

    code.push_str(&quote_block! {
        impl #data_name {
            fn conforms(bytes: &[u8]) -> bool {
                // TODO(mgeisler): return Boolean expression directly.
                // TODO(mgeisler): skip when total_field_size == 0.
                if bytes.len() < #packet_size_bytes {
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
    let field_getters = fields.iter().map(|field| Field::from(field).generate_getter(&ident));
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
pub fn generate(sources: &ast::SourceDatabase, file: &ast::File) -> String {
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

    code.push_str(&preamble::generate(Path::new(source.name())));

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
    use crate::test_utils::{assert_expr_eq, assert_snapshot_eq, rustfmt};

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
    fn test_generate_packet_decl_simple_little_endian() {
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

    #[test]
    fn test_generate_packet_decl_complex_little_endian() {
        let grammar = parse_str(
            r#"
              little_endian_packets

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
        let packets = HashMap::new();
        let children = HashMap::new();
        let decl = &grammar.declarations[0];
        let actual_code = generate_decl(&grammar, &packets, &children, decl);
        assert_snapshot_eq(
            "tests/generated/packet_decl_complex_little_endian.rs",
            &rustfmt(&actual_code),
        );
    }

    #[test]
    fn test_generate_packet_decl_complex_big_endian() {
        let grammar = parse_str(
            r#"
              big_endian_packets

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
        let packets = HashMap::new();
        let children = HashMap::new();
        let decl = &grammar.declarations[0];
        let actual_code = generate_decl(&grammar, &packets, &children, decl);
        assert_snapshot_eq(
            "tests/generated/packet_decl_complex_big_endian.rs",
            &rustfmt(&actual_code),
        );
    }

    #[test]
    fn test_get_field_range() {
        // Zero widths will give you an empty slice iff the offset is
        // byte aligned. In both cases, the slice covers the empty
        // width. In practice, PDL doesn't allow zero-width fields.
        assert_eq!(get_field_range(/*offset=*/ 0, /*width=*/ 0), (0..0));
        assert_eq!(get_field_range(/*offset=*/ 5, /*width=*/ 0), (0..1));
        assert_eq!(get_field_range(/*offset=*/ 8, /*width=*/ 0), (1..1));
        assert_eq!(get_field_range(/*offset=*/ 9, /*width=*/ 0), (1..2));

        // Non-zero widths work as expected.
        assert_eq!(get_field_range(/*offset=*/ 0, /*width=*/ 1), (0..1));
        assert_eq!(get_field_range(/*offset=*/ 0, /*width=*/ 5), (0..1));
        assert_eq!(get_field_range(/*offset=*/ 0, /*width=*/ 8), (0..1));
        assert_eq!(get_field_range(/*offset=*/ 0, /*width=*/ 20), (0..3));

        assert_eq!(get_field_range(/*offset=*/ 5, /*width=*/ 1), (0..1));
        assert_eq!(get_field_range(/*offset=*/ 5, /*width=*/ 3), (0..1));
        assert_eq!(get_field_range(/*offset=*/ 5, /*width=*/ 4), (0..2));
        assert_eq!(get_field_range(/*offset=*/ 5, /*width=*/ 20), (0..4));
    }

    #[test]
    fn test_generate_chunk_write_8bit() {
        let loc = ast::SourceRange::default();
        let fields = &[ast::Field::Scalar { loc, id: String::from("a"), width: 8 }];
        assert_expr_eq(
            generate_chunk_write(ast::EndiannessValue::BigEndian, 80, fields),
            quote! {
                buffer[10..11].copy_from_slice(&a.to_be_bytes()[0..1]);
            },
        );
    }

    #[test]
    fn test_generate_chunk_write_16bit() {
        let loc = ast::SourceRange::default();
        let fields = &[ast::Field::Scalar { loc, id: String::from("a"), width: 16 }];
        assert_expr_eq(
            generate_chunk_write(ast::EndiannessValue::BigEndian, 80, fields),
            quote! {
                buffer[10..12].copy_from_slice(&a.to_be_bytes()[0..2]);
            },
        );
    }

    #[test]
    fn test_generate_chunk_write_24bit() {
        let loc = ast::SourceRange::default();
        let fields = &[ast::Field::Scalar { loc, id: String::from("a"), width: 24 }];
        assert_expr_eq(
            generate_chunk_write(ast::EndiannessValue::BigEndian, 80, fields),
            quote! {
                buffer[10..13].copy_from_slice(&a.to_be_bytes()[0..3]);
            },
        );
    }

    #[test]
    fn test_generate_chunk_write_multiple_fields() {
        let loc = ast::SourceRange::default();
        let fields = &[
            ast::Field::Scalar { loc, id: String::from("a"), width: 16 },
            ast::Field::Scalar { loc, id: String::from("b"), width: 24 },
        ];
        assert_expr_eq(
            generate_chunk_write(ast::EndiannessValue::BigEndian, 80, fields),
            quote! {
                buffer[10..15].copy_from_slice(&chunk.to_be_bytes()[0..5]);
            },
        );
    }

    #[test]
    fn test_generate_chunk_write_field_adjustments_8bit() {
        let loc = ast::SourceRange::default();
        let fields = vec![
            ast::Field::Scalar { loc, id: String::from("a"), width: 3 },
            ast::Field::Scalar { loc, id: String::from("b"), width: 5 },
        ];
        assert_expr_eq(
            generate_chunk_write_field_adjustments(&fields),
            quote! {
                let chunk = 0;
                let chunk = chunk | (self.a & 0x7) ;
                let chunk = chunk | ((self.b & 0x1f) << 3);
            },
        );
    }

    #[test]
    fn test_generate_chunk_write_field_adjustments_48bit() {
        let loc = ast::SourceRange::default();
        let fields = vec![
            ast::Field::Scalar { loc, id: String::from("a"), width: 3 },
            ast::Field::Scalar { loc, id: String::from("b"), width: 8 },
            ast::Field::Scalar { loc, id: String::from("c"), width: 10 },
            ast::Field::Scalar { loc, id: String::from("d"), width: 18 },
            ast::Field::Scalar { loc, id: String::from("e"), width: 9 },
        ];
        assert_expr_eq(
            generate_chunk_write_field_adjustments(&fields),
            quote! {
                let chunk = 0;
                let chunk = chunk | ((self.a as u64) & 0x7);
                let chunk = chunk | ((self.b as u64) << 3);
                let chunk = chunk | (((self.c as u64) & 0x3ff) << 11);
                let chunk = chunk | (((self.d as u64) & 0x3ffff) << 21);
                let chunk = chunk | (((self.e as u64) & 0x1ff) << 39);
            },
        );
    }
}
