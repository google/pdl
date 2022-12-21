//! Rust compiler backend.

// The `format-push-string` lint was briefly enabled present in Rust
// 1.62. It is now moved the disabled "restriction" category instead.
// See https://github.com/rust-lang/rust-clippy/issues/9077 for the
// problems with this lint.
//
// Remove this when we use Rust 1.63 or later.
#![allow(clippy::format_push_string)]

use crate::{ast, lint};
use quote::{format_ident, quote};
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

/// Generate a bit-mask which masks out `n` least significant bits.
pub fn mask_bits(n: usize) -> syn::LitInt {
    syn::parse_str::<syn::LitInt>(&format!("{:#x}", (1u64 << n) - 1)).unwrap()
}

/// Generate code for an `ast::Decl::Packet` enum value.
fn generate_packet_decl(
    scope: &lint::Scope<'_>,
    file: &ast::File,
    id: &str,
    fields: &[Field],
    parent_id: &Option<String>,
) -> String {
    // TODO(mgeisler): use the convert_case crate to convert between
    // `FooBar` and `foo_bar` in the code below.
    let mut code = String::new();
    let child_ids = scope
        .typedef
        .values()
        .filter_map(|p| match p {
            ast::Decl::Packet { id, parent_id, .. } if parent_id.as_deref() == Some(id) => Some(id),
            _ => None,
        })
        .collect::<Vec<_>>();
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
    let plain_fields = fields.iter().map(|field| field.generate_decl(parse_quote!()));
    code.push_str(&quote_block! {
        #[derive(Debug)]
        struct #data_name {
            #(#plain_fields,)*
            #child_field
        }
    });

    let parent = parent_id.as_ref().map(|parent_id| match scope.typedef.get(parent_id.as_str()) {
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
    let pub_fields = fields.iter().map(|field| field.generate_decl(parse_quote!(pub)));
    code.push_str(&quote_block! {
        #[derive(Debug)]
        pub struct #builder_name {
            #(#pub_fields,)*
        }
    });

    let mut chunk_width = 0;
    let chunks = fields.split_inclusive(|field| {
        chunk_width += field.width();
        chunk_width % 8 == 0
    });
    let mut field_parsers = Vec::new();
    let mut field_writers = Vec::new();
    for fields in chunks {
        let chunk = Chunk::new(fields);
        field_parsers.push(chunk.generate_read(id, file.endianness.value));
        field_writers.push(chunk.generate_write(file.endianness.value));
    }

    let field_names = fields.iter().map(Field::ident).collect::<Vec<_>>();

    let packet_size_bits = Chunk::new(fields).width();
    if packet_size_bits % 8 != 0 {
        panic!("packet {id} does not end on a byte boundary, size: {packet_size_bits} bits",);
    }
    let packet_size_bytes = syn::Index::from(packet_size_bits / 8);

    let conforms = if packet_size_bytes.index == 0 {
        quote! { true }
    } else {
        quote! { bytes.len() >= #packet_size_bytes }
    };

    code.push_str(&quote_block! {
        impl #data_name {
            fn conforms(bytes: &[u8]) -> bool {
                #conforms
            }

            fn parse(mut bytes: &[u8]) -> Result<Self> {
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
                #packet_size_bytes
            }
        }
    });

    code.push_str(&quote_block! {
        impl Packet for #packet_name {
            fn to_bytes(self) -> Bytes {
                let mut buffer = BytesMut::with_capacity(self.#ident.get_total_size());
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
    let field_getters = fields.iter().map(|field| field.generate_getter(&ident));
    code.push_str(&quote_block! {
        impl #packet_name {
            pub fn parse(mut bytes: &[u8]) -> Result<Self> {
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

fn generate_decl(scope: &lint::Scope<'_>, file: &ast::File, decl: &ast::Decl) -> String {
    match decl {
        ast::Decl::Packet { id, fields, parent_id, .. } => {
            let fields = fields.iter().map(Field::from).collect::<Vec<_>>();
            generate_packet_decl(scope, file, id, &fields, parent_id)
        }
        _ => todo!("unsupported Decl::{:?}", decl),
    }
}

/// Generate Rust code from an AST.
///
/// The code is not formatted, pipe it through `rustfmt` to get
/// readable source code.
pub fn generate(sources: &ast::SourceDatabase, file: &ast::File) -> String {
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

    test_pdl!(
        packet_decl_simple,
        r#"
          packet Foo {
            x: 8,
            y: 16,
            z: 24,
          }
        "#
    );

    test_pdl!(
        packet_decl_complex,
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
}
