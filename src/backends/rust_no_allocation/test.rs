use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::Deserialize;

use crate::{ast, parser::parse_inline, quote_block};

#[derive(Deserialize)]
struct PacketTest {
    packet: String,
    tests: Box<[PacketTestCase]>,
}

#[derive(Deserialize)]
struct PacketTestCase {
    packed: String,
    unpacked: UnpackedTestFields,
    packet: Option<String>,
}

#[derive(Deserialize)]
struct UnpackedTestFields(HashMap<String, Field>);

// fields can be scalars, lists, or structs
#[derive(Deserialize)]
#[serde(untagged)]
enum Field {
    Number(usize),
    Struct(UnpackedTestFields),
    List(Box<[ListEntry]>),
}

// lists can either contain scalars or structs
#[derive(Deserialize)]
#[serde(untagged)]
enum ListEntry {
    Number(usize),
    Struct(UnpackedTestFields),
}

fn generate_matchers(
    base: TokenStream,
    value: &UnpackedTestFields,
    filter_fields: &dyn Fn(&str) -> Result<bool, String>,
    curr_type: &str,
    type_lookup: &HashMap<&str, HashMap<&str, Option<&str>>>,
) -> Result<TokenStream, String> {
    let mut out = vec![];

    for (field_name, field_value) in value.0.iter() {
        if !filter_fields(field_name)? {
            continue;
        }
        let getter_ident = format_ident!("get_{field_name}");
        match field_value {
            Field::Number(num) => {
                let num = *num as u64;
                if let Some(field_type) = type_lookup[curr_type][field_name.as_str()] {
                    let field_ident = format_ident!("{field_type}");
                    out.push(quote! { assert_eq!(#base.#getter_ident(), #field_ident::new(#num as _).unwrap()); });
                } else {
                    out.push(quote! { assert_eq!(u64::from(#base.#getter_ident()), #num); });
                }
            }
            Field::List(lst) => {
                if field_name == "payload" {
                    let reference = lst
                        .iter()
                        .map(|val| match val {
                            ListEntry::Number(val) => *val as u8,
                            _ => unreachable!(),
                        })
                        .collect::<Vec<_>>();
                    out.push(quote! {
                        assert_eq!(#base.get_raw_payload().collect::<Vec<_>>(), vec![#(#reference),*]);
                    })
                } else {
                    let get_iter_ident = format_ident!("get_{field_name}_iter");
                    let vec_ident = format_ident!("{field_name}_vec");
                    out.push(
                        quote! { let #vec_ident = #base.#get_iter_ident().collect::<Vec<_>>(); },
                    );

                    for (i, val) in lst.iter().enumerate() {
                        let list_elem = quote! { #vec_ident[#i] };
                        out.push(match val {
                            ListEntry::Number(num) => {
                                if let Some(field_type) = type_lookup[curr_type][field_name.as_str()] {
                                    let field_ident = format_ident!("{field_type}");
                                    quote! { assert_eq!(#list_elem, #field_ident::new(#num as _).unwrap()); }
                                } else {
                                    quote! { assert_eq!(u64::from(#list_elem), #num as u64); }
                                }
                            }
                            ListEntry::Struct(fields) => {
                                generate_matchers(list_elem, fields, &|_| Ok(true), type_lookup[curr_type][field_name.as_str()].unwrap(), type_lookup)?
                            }
                        })
                    }
                }
            }
            Field::Struct(fields) => {
                out.push(generate_matchers(
                    quote! { #base.#getter_ident() },
                    fields,
                    &|_| Ok(true),
                    type_lookup[curr_type][field_name.as_str()].unwrap(),
                    type_lookup,
                )?);
            }
        }
    }
    Ok(quote! { { #(#out)* } })
}

fn generate_builder(
    curr_type: &str,
    child_type: Option<&str>,
    type_lookup: &HashMap<&str, HashMap<&str, Option<&str>>>,
    value: &UnpackedTestFields,
) -> TokenStream {
    let builder_ident = format_ident!("{curr_type}Builder");
    let child_ident = format_ident!("{curr_type}Child");

    let curr_fields = &type_lookup[curr_type];

    let fields = value.0.iter().filter_map(|(field_name, field_value)| {
        let curr_field_info = curr_fields.get(field_name.as_str());

        if let Some(curr_field_info) = curr_field_info {
            let field_name_ident = if field_name == "payload" {
                format_ident!("_child_")
            } else {
                format_ident!("{field_name}")
            };
            let val = match field_value {
                Field::Number(val) => {
                    if let Some(field) = curr_field_info {
                        let field_ident = format_ident!("{field}");
                        quote! { #field_ident::new(#val as _).unwrap() }
                    } else {
                        quote! { (#val as u64).try_into().unwrap() }
                    }
                }
                Field::Struct(fields) => {
                    generate_builder(curr_field_info.unwrap(), None, type_lookup, fields)
                }
                Field::List(lst) => {
                    let elems = lst.iter().map(|entry| match entry {
                        ListEntry::Number(val) => {
                            if let Some(field) = curr_field_info {
                                let field_ident = format_ident!("{field}");
                                quote! { #field_ident::new(#val as _).unwrap() }
                            } else {
                                quote! { (#val as u64).try_into().unwrap() }
                            }
                        }
                        ListEntry::Struct(fields) => {
                            generate_builder(curr_field_info.unwrap(), None, type_lookup, fields)
                        }
                    });
                    quote! { vec![#(#elems),*].into_boxed_slice() }
                }
            };

            Some(if field_name == "payload" {
                quote! { #field_name_ident: #child_ident::RawData(#val) }
            } else {
                quote! { #field_name_ident: #val }
            })
        } else {
            None
        }
    });

    let child_field = if let Some(child_type) = child_type {
        let child_builder = generate_builder(child_type, None, type_lookup, value);
        Some(quote! {
            _child_: #child_builder.into(),
        })
    } else {
        None
    };

    quote! {
        #builder_ident {
            #child_field
            #(#fields),*
        }
    }
}

pub fn generate_test_file() -> Result<String, String> {
    let mut out = String::new();

    out.push_str(include_str!("test_preamble.rs"));

    let file = include_str!("../../../tests/canonical/le_test_vectors.json");
    let test_vectors: Box<[_]> =
        serde_json::from_str(file).map_err(|_| "could not parse test vectors")?;

    let pdl = include_str!("../../../tests/canonical/le_rust_noalloc_test_file.pdl");
    let ast = parse_inline(&mut ast::SourceDatabase::new(), "test.pdl".to_owned(), pdl.to_owned())
        .expect("could not parse reference PDL");
    let packet_lookup = ast
        .declarations
        .iter()
        .filter_map(|decl| match decl {
            ast::Decl::Packet { id, fields, .. } | ast::Decl::Struct { id, fields, .. } => Some((
                id.as_str(),
                fields
                    .iter()
                    .filter_map(|field| match field {
                        ast::Field::Body { .. } | ast::Field::Payload { .. } => {
                            Some(("payload", None))
                        }
                        ast::Field::Array { id, type_id, .. } => match type_id {
                            Some(type_id) => Some((id.as_str(), Some(type_id.as_str()))),
                            None => Some((id.as_str(), None)),
                        },
                        ast::Field::Typedef { id, type_id, .. } => {
                            Some((id.as_str(), Some(type_id.as_str())))
                        }
                        ast::Field::Scalar { id, .. } => Some((id.as_str(), None)),
                        _ => None,
                    })
                    .collect::<HashMap<_, _>>(),
            )),
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    for PacketTest { packet, tests } in test_vectors.iter() {
        if !pdl.contains(packet) {
            // huge brain hack to skip unsupported test vectors
            continue;
        }

        for (i, PacketTestCase { packed, unpacked, packet: sub_packet }) in tests.iter().enumerate()
        {
            if let Some(sub_packet) = sub_packet {
                if !pdl.contains(sub_packet) {
                    // huge brain hack to skip unsupported test vectors
                    continue;
                }
            }

            let test_name_ident = format_ident!("test_{packet}_{i}");
            let packet_ident = format_ident!("{packet}_instance");
            let packet_view = format_ident!("{packet}View");

            let mut leaf_packet = packet;

            let specialization = if let Some(sub_packet) = sub_packet {
                let sub_packet_ident = format_ident!("{}_instance", sub_packet);
                let sub_packet_view_ident = format_ident!("{}View", sub_packet);

                leaf_packet = sub_packet;
                quote! { let #sub_packet_ident = #sub_packet_view_ident::try_parse(#packet_ident).unwrap(); }
            } else {
                quote! {}
            };

            let leaf_packet_ident = format_ident!("{leaf_packet}_instance");

            let packet_matchers = generate_matchers(
                quote! { #packet_ident },
                unpacked,
                &|field| {
                    Ok(packet_lookup
                        .get(packet.as_str())
                        .ok_or(format!("could not find packet {packet}"))?
                        .contains_key(field))
                },
                packet,
                &packet_lookup,
            )?;

            let sub_packet_matchers = generate_matchers(
                quote! { #leaf_packet_ident },
                unpacked,
                &|field| {
                    Ok(packet_lookup
                        .get(leaf_packet.as_str())
                        .ok_or(format!("could not find packet {packet}"))?
                        .contains_key(field))
                },
                sub_packet.as_ref().unwrap_or(packet),
                &packet_lookup,
            )?;

            out.push_str(&quote_block! {
              #[test]
              fn #test_name_ident() {
                let base = hex_str_to_byte_vector(#packed);
                let #packet_ident = #packet_view::try_parse(SizedBitSlice::from(&base[..]).into()).unwrap();

                #specialization

                #packet_matchers
                #sub_packet_matchers
              }
            });

            let builder = generate_builder(packet, sub_packet.as_deref(), &packet_lookup, unpacked);

            let test_name_ident = format_ident!("test_{packet}_builder_{i}");
            out.push_str(&quote_block! {
              #[test]
              fn #test_name_ident() {
                let packed = hex_str_to_byte_vector(#packed);
                let serialized = #builder.to_vec().unwrap();
                assert_eq!(packed, serialized);
              }
            });
        }
    }

    Ok(out)
}
