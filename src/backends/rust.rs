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

mod declarations;
mod parser;
mod preamble;
mod serializer;
mod types;

use declarations::FieldDeclarations;
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
pub fn mask_bits(n: usize) -> syn::LitInt {
    // The literal needs a suffix if it's larger than an i32.
    let suffix = if n > 31 { "u64" } else { "" };
    syn::parse_str::<syn::LitInt>(&format!("{:#x}{suffix}", (1u64 << n) - 1)).unwrap()
}

/// Generate code for `ast::Decl::Packet` and `ast::Decl::Struct`
/// values.
fn generate_packet_decl(
    scope: &lint::Scope<'_>,
    //  File:
    endianness: ast::EndiannessValue,
    // Packet:
    id: &str,
    _constraints: &[ast::Constraint],
    fields: &[ast::Field],
    _parent_id: Option<&str>,
) -> proc_macro2::TokenStream {
    // TODO(mgeisler): use the convert_case crate to convert between
    // `FooBar` and `foo_bar` in the code below.
    let span = format_ident!("bytes");
    let serializer_span = format_ident!("buffer");
    let mut field_declarations = FieldDeclarations::new();
    let mut field_parser = FieldParser::new(scope, endianness, id, &span);
    let mut field_serializer = FieldSerializer::new(scope, endianness, id, &serializer_span);
    for field in fields {
        field_declarations.add(field);
        field_parser.add(field);
        field_serializer.add(field);
    }
    field_parser.done();

    let id_lower = format_ident!("{}", id.to_lowercase());
    let id_packet = format_ident!("{id}");
    let id_data = format_ident!("{id}Data");
    let id_builder = format_ident!("{id}Builder");

    let fields_with_ids = fields.iter().filter(|f| f.id().is_some()).collect::<Vec<_>>();
    let field_names =
        fields_with_ids.iter().map(|f| format_ident!("{}", f.id().unwrap())).collect::<Vec<_>>();
    let field_types = fields_with_ids.iter().map(|f| types::rust_type(f)).collect::<Vec<_>>();
    let getter_names = field_names.iter().map(|id| format_ident!("get_{id}"));

    let packet_size =
        syn::Index::from(fields.iter().filter_map(|f| f.width(scope)).sum::<usize>() / 8);
    let conforms = if packet_size.index == 0 {
        quote! { true }
    } else {
        quote! { #span.len() >= #packet_size }
    };

    quote! {
        #[derive(Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        struct #id_data {
            #field_declarations
        }

        #[derive(Debug, Clone)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct #id_packet {
            #[cfg_attr(feature = "serde", serde(flatten))]
            #id_lower: Arc<#id_data>,
        }

        #[derive(Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct #id_builder {
            #(pub #field_names: #field_types),*
        }

        impl #id_data {
            fn conforms(#span: &[u8]) -> bool {
                #conforms
            }

            fn parse(mut #span: &[u8]) -> Result<Self> {
                #field_parser
                Ok(Self { #(#field_names),* })
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

        impl Packet for #id_packet {
            fn to_bytes(self) -> Bytes {
                let mut buffer = BytesMut::with_capacity(self.#id_lower.get_total_size());
                self.#id_lower.write_to(&mut buffer);
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

        impl #id_packet {
            pub fn parse(mut bytes: &[u8]) -> Result<Self> {
                Ok(Self::new(Arc::new(#id_data::parse(bytes)?)).unwrap())
            }
            fn new(root: Arc<#id_data>) -> std::result::Result<Self, &'static str> {
                let #id_lower = root;
                Ok(Self { #id_lower })
            }

            #(pub fn #getter_names(&self) -> #field_types {
                self.#id_lower.as_ref().#field_names
            })*
        }

        impl #id_builder {
            pub fn build(self) -> #id_packet {
                let #id_lower = Arc::new(#id_data {
                    #(#field_names: self.#field_names),*
                });
                #id_packet::new(#id_lower).unwrap()
            }
        }
    }
}

fn generate_enum_decl(id: &str, tags: &[ast::Tag]) -> proc_macro2::TokenStream {
    let name = format_ident!("{id}");
    let variants = tags.iter().map(|t| format_ident!("{}", t.id)).collect::<Vec<_>>();
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

fn generate_decl(scope: &lint::Scope<'_>, file: &ast::File, decl: &ast::Decl) -> String {
    match decl {
        ast::Decl::Packet { id, constraints, fields, parent_id, .. }
        | ast::Decl::Struct { id, constraints, fields, parent_id, .. } => generate_packet_decl(
            scope,
            file.endianness.value,
            id,
            constraints,
            fields,
            parent_id.as_deref(),
        )
        .to_string(),
        ast::Decl::Enum { id, tags, .. } => generate_enum_decl(id, tags).to_string(),
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

    test_pdl!(
        packet_decl_reserved_field,
        "
          packet Foo {
            _reserved_: 40,
          }
        "
    );
}
