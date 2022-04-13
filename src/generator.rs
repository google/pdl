use crate::ast;
use anyhow::{anyhow, bail, Context, Result};
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
fn generate_preamble(path: &Path) -> Result<String> {
    let mut code = String::new();
    let filename = path
        .file_name()
        .and_then(|path| path.to_str())
        .ok_or_else(|| anyhow!("could not find filename in {:?}", path))?;
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

    Ok(code)
}

/// Round up the bit width to a Rust integer size.
fn round_bit_width(width: usize) -> Result<usize> {
    match width {
        8 => Ok(8),
        16 => Ok(16),
        24 | 32 => Ok(32),
        40 | 48 | 56 | 64 => Ok(64),
        _ => bail!("unsupported field width: {width}"),
    }
}

/// Generate a Rust unsigned integer type large enough to hold
/// integers of the given bit width.
fn type_for_width(width: usize) -> Result<syn::Type> {
    let rounded_width = round_bit_width(width)?;
    syn::parse_str(&format!("u{rounded_width}")).map_err(anyhow::Error::from)
}

fn generate_field(
    field: &ast::Field,
    visibility: syn::Visibility,
) -> Result<proc_macro2::TokenStream> {
    match field {
        ast::Field::Scalar { id, width, .. } => {
            let field_name = format_ident!("{id}");
            let field_type = type_for_width(*width)?;
            Ok(quote! {
                #visibility #field_name: #field_type
            })
        }
        _ => todo!("unsupported field: {:?}", field),
    }
}

fn generate_field_getter(
    packet_name: &syn::Ident,
    field: &ast::Field,
) -> Result<proc_macro2::TokenStream> {
    match field {
        ast::Field::Scalar { id, width, .. } => {
            // TODO(mgeisler): refactor with generate_field above.
            let getter_name = format_ident!("get_{id}");
            let field_name = format_ident!("{id}");
            let field_type = type_for_width(*width)?;
            Ok(quote! {
                pub fn #getter_name(&self) -> #field_type {
                    self.#packet_name.as_ref().#field_name
                }
            })
        }
        _ => todo!("unsupported field: {:?}", field),
    }
}

fn generate_field_parser(
    endianness_value: &ast::EndiannessValue,
    packet_name: &str,
    field: &ast::Field,
    offset: usize,
) -> Result<proc_macro2::TokenStream> {
    match field {
        ast::Field::Scalar { id, width, .. } => {
            let field_name = format_ident!("{id}");
            let type_width = round_bit_width(*width)?;
            let field_type = type_for_width(*width)?;

            let getter = match endianness_value {
                ast::EndiannessValue::BigEndian => format_ident!("from_be_bytes"),
                ast::EndiannessValue::LittleEndian => format_ident!("from_le_bytes"),
            };

            let wanted_len = syn::Index::from(offset + width / 8);
            let indices = (offset..offset + width / 8).map(syn::Index::from);
            let padding = vec![syn::Index::from(0); (type_width - width) / 8];
            let mask = if *width != type_width {
                Some(quote! {
                    let #field_name = #field_name & 0xfff;
                })
            } else {
                None
            };

            Ok(quote! {
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
                let #field_name = #field_type::#getter([#(bytes[#indices]),* #(, #padding)*]);
                #mask
            })
        }
        _ => todo!("unsupported field: {:?}", field),
    }
}

fn generate_field_writer(
    grammar: &ast::Grammar,
    field: &ast::Field,
    offset: usize,
) -> Result<proc_macro2::TokenStream> {
    match field {
        ast::Field::Scalar { id, width, .. } => {
            let field_name = format_ident!("{id}");
            let bit_width = round_bit_width(*width)?;
            let start = syn::Index::from(offset);
            let end = syn::Index::from(offset + bit_width / 8);
            let byte_width = syn::Index::from(bit_width / 8);
            let writer = match grammar.endianness.value {
                ast::EndiannessValue::BigEndian => format_ident!("to_be_bytes"),
                ast::EndiannessValue::LittleEndian => format_ident!("to_le_bytes"),
            };
            Ok(quote! {
                let #field_name = self.#field_name;
                buffer[#start..#end].copy_from_slice(&#field_name.#writer()[0..#byte_width]);
            })
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
    grammar: &ast::Grammar,
    packets: &HashMap<&str, &ast::Decl>,
    child_ids: &[&str],
    id: &str,
    fields: &[ast::Field],
    parent_id: &Option<String>,
) -> Result<String> {
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
    let plain_fields = fields
        .iter()
        .map(|field| generate_field(field, parse_quote!()))
        .collect::<Result<Vec<_>>>()?;
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
    let pub_fields = fields
        .iter()
        .map(|field| generate_field(field, parse_quote!(pub)))
        .collect::<Result<Vec<_>>>()?;
    code.push_str(&quote_block! {
        #[derive(Debug)]
        pub struct #builder_name {
            #(#pub_fields,)*
        }
    });

    // TODO(mgeisler): use the `Buf` trait instead of tracking
    // the offset manually.
    let mut offset = 0;
    let field_parsers = fields
        .iter()
        .map(|field| {
            let parser = generate_field_parser(&grammar.endianness.value, id, field, offset);
            offset += get_field_size(field);
            parser
        })
        .collect::<Result<Vec<_>>>()?;
    let field_names = fields
        .iter()
        .map(|field| match field {
            ast::Field::Scalar { id, .. } => format_ident!("{id}"),
            _ => todo!("unsupported field: {:?}", field),
        })
        .collect::<Vec<_>>();
    let mut offset = 0;
    let field_writers = fields
        .iter()
        .map(|field| {
            let writer = generate_field_writer(grammar, field, offset);
            offset += get_field_size(field);
            writer
        })
        .collect::<Result<Vec<_>>>()?;

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
    let field_getters = fields
        .iter()
        .map(|field| generate_field_getter(&ident, field))
        .collect::<Result<Vec<_>>>()?;
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

    Ok(code)
}

fn generate_decl(
    grammar: &ast::Grammar,
    packets: &HashMap<&str, &ast::Decl>,
    children: &HashMap<&str, Vec<&str>>,
    decl: &ast::Decl,
) -> Result<String> {
    let empty: Vec<&str> = vec![];
    match decl {
        ast::Decl::Packet { id, fields, parent_id, .. } => generate_packet_decl(
            grammar,
            packets,
            children.get(id.as_str()).unwrap_or(&empty),
            id,
            fields,
            parent_id,
        ),
        _ => todo!("unsupported Decl::{:?}", decl),
    }
}

/// Generate Rust code from `grammar`.
///
/// The code is not formatted, pipe it through `rustfmt` to get
/// readable source code.
pub fn generate_rust(sources: &ast::SourceDatabase, grammar: &ast::Grammar) -> Result<String> {
    let source =
        sources.get(grammar.file).with_context(|| format!("could not read {}", grammar.file))?;

    let mut children = HashMap::new();
    let mut packets = HashMap::new();
    for decl in &grammar.declarations {
        if let ast::Decl::Packet { id, parent_id, .. } = decl {
            packets.insert(id.as_str(), decl);
            if let Some(parent_id) = parent_id {
                children.entry(parent_id.as_str()).or_insert_with(Vec::new).push(id.as_str());
            }
        }
    }

    let mut code = String::new();

    code.push_str(&generate_preamble(Path::new(source.name()))?);

    for decl in &grammar.declarations {
        let decl_code = generate_decl(grammar, &packets, &children, decl)
            .with_context(|| format!("failed to generating code for {:?}", decl))?;
        code.push_str(&decl_code);
        code.push_str("\n\n");
    }

    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;
    use crate::parser::parse_inline;
    use crate::test_utils::{assert_eq_with_diff, rustfmt};

    /// Parse a string fragment as a PDL file.
    ///
    /// # Panics
    ///
    /// Panics on parse errors.
    pub fn parse_str(text: &str) -> ast::Grammar {
        let mut db = ast::SourceDatabase::new();
        parse_inline(&mut db, String::from("stdin"), String::from(text)).expect("parse error")
    }

    #[test]
    fn test_generate_preamble() {
        let actual_code = generate_preamble(Path::new("some/path/foo.pdl")).unwrap();
        let expected_code = include_str!("../test/generated/preamble.rs");
        assert_eq_with_diff(&rustfmt(expected_code), &rustfmt(&actual_code));
    }

    #[test]
    fn test_generate_packet_decl_empty() {
        let grammar = parse_str(
            r#"
              big_endian_packets
              packet Foo {}
            "#,
        );
        let packets = HashMap::new();
        let children = HashMap::new();
        let decl = &grammar.declarations[0];
        let actual_code = generate_decl(&grammar, &packets, &children, decl).unwrap();
        let expected_code = include_str!("../test/generated/packet_decl_empty.rs");
        assert_eq_with_diff(&rustfmt(expected_code), &rustfmt(&actual_code));
    }

    #[test]
    fn test_generate_packet_decl_little_endian() {
        let grammar = parse_str(
            r#"
              little_endian_packets

              packet Foo {
                x: 8,
                y: 16,
              }
            "#,
        );
        let packets = HashMap::new();
        let children = HashMap::new();
        let decl = &grammar.declarations[0];
        let actual_code = generate_decl(&grammar, &packets, &children, decl).unwrap();
        let expected_code = include_str!("../test/generated/packet_decl_simple_little_endian.rs");
        assert_eq_with_diff(&rustfmt(expected_code), &rustfmt(&actual_code));
    }

    #[test]
    fn test_generate_packet_decl_simple_big_endian() {
        let grammar = parse_str(
            r#"
              big_endian_packets

              packet Foo {
                x: 8,
                y: 16,
              }
            "#,
        );
        let packets = HashMap::new();
        let children = HashMap::new();
        let decl = &grammar.declarations[0];
        let actual_code = generate_decl(&grammar, &packets, &children, decl).unwrap();
        let expected_code = include_str!("../test/generated/packet_decl_simple_big_endian.rs");
        assert_eq_with_diff(&rustfmt(expected_code), &rustfmt(&actual_code));
    }
}
