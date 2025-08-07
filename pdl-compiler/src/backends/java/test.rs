// Copyright 2025 Google LLC
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

use genco::{
    lang::Java,
    prelude::{java, quote_fn},
    quote,
    tokens::{quoted, FormatInto},
    Tokens,
};
use heck::ToUpperCamelCase;
use serde_json::{Map, Value};
use std::{
    collections::{HashMap, HashSet},
    fs, iter,
    path::{Path, PathBuf},
    slice::Iter,
    str,
};

use super::{import, Class, Integral, JavaFile};
use crate::{
    ast::{self, Decl, DeclDesc, Field, FieldDesc},
    backends::{
        common::test::{Packet, TestVector},
        java::codegen::expr::literal,
    },
    parser,
};

pub fn generate_tests(
    input_file: &str,
    output_dir: &Path,
    package: String,
    pdl_file_under_test: &str,
    exclude_packets: &[String],
) -> Result<(), String> {
    let mut dir = PathBuf::from(output_dir);
    dir.extend(package.split("."));
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let output_file = dir.join("PdlTests").with_extension("java");

    let decls: HashMap<String, Decl> =
        parser::parse_file(&mut ast::SourceDatabase::new(), pdl_file_under_test)
            .expect("failed to parse pdl file under test. Please verify that the file compiles.")
            .declarations
            .into_iter()
            .flat_map(|decl| decl.id().map(String::from).map(|id| (id, decl)))
            .collect();

    JavaTest(get_test_cases(input_file, exclude_packets)?).write_to_fs(
        &output_file,
        &package,
        input_file,
        decls,
    )
}

fn get_test_cases(file: &str, exclude_packets: &[String]) -> Result<Vec<Packet>, String> {
    let data = fs::read_to_string(file).map_err(|err| err.to_string())?;
    let mut packets: Vec<Packet> = serde_json::from_str(&data).map_err(|err| err.to_string())?;

    eprintln!("Read {} test vectors from {file}", packets.len());

    packets.retain(|p| !exclude_packets.contains(&p.name));
    for packet in packets.iter_mut() {
        packet.tests.retain(|t| !t.packet.as_ref().is_some_and(|p| exclude_packets.contains(p)));
    }

    Ok(packets)
}

struct JavaTest(Vec<Packet>);

impl JavaFile<HashMap<String, Decl>> for JavaTest {
    fn generate(self, context: HashMap<String, Decl>) -> Tokens<Java> {
        quote! {
            final class PdlTests {
                $(for packet in self.0.iter() => $(packet.generate(&context)))

                public static void main(String[] args) {
                    $(for packet in self.0.iter() {
                        $(for (i, _) in packet.tests.iter().enumerate() {
                            System.out.println("┌─[TEST] " + $(quoted(&packet.name)) + $i);
                            TEST_$(&packet.name).testEncode$i();
                            TEST_$(&packet.name).testDecode$i();
                            System.out.println("└─[PASS]");
                        })
                    })
                    System.out.println("All tests passed!");
                }
            }
        }
    }
}

impl Packet {
    fn generate<'a>(&'a self, decls: &'a HashMap<String, Decl>) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            static final class TEST_$(&self.name) {
                $(for (i, test_case) in self.tests.iter().enumerate() {
                    $(test_case.encoder_test(&self.name, i, decls))
                    $(test_case.decoder_test(&self.name, i, decls))
                })
            }
        }
    }
}

impl TestVector {
    fn encoder_test<'a>(
        &'a self,
        id: &'a String,
        test_id: usize,
        decls: &'a HashMap<String, Decl>,
    ) -> impl FormatInto<Java> + 'a {
        let maybe_child = self.packet.as_ref().unwrap_or(id);

        quote_fn! {
            $(java::block_comment(iter::once(format!("0x{}", &self.packed))))$['\n']
            static void testEncode$test_id() {
                $(Class::name_from_id(maybe_child)) packet = $(build_packet_from_fields(
                    self.packet.as_ref().unwrap_or(id),
                    self.unpacked.as_object().unwrap(),
                    decls
                ));
                byte[] encodedPacket = packet.toBytes();
                byte[] expectedBytes = $(hex_to_array(&self.packed));
                assert $(&*import::ARRAYS).equals(expectedBytes, encodedPacket);
            }
        }
    }

    fn decoder_test<'a>(
        &'a self,
        id: &'a String,
        test_id: usize,
        decls: &'a HashMap<String, Decl>,
    ) -> impl FormatInto<Java> + 'a {
        let packet_name = format!(
            "{}{}",
            if self.unpacked.as_object().unwrap().contains_key("payload") { "Unknown" } else { "" },
            self.packet
                .as_ref()
                .map(|child_id| Class::name_from_id(child_id))
                .unwrap_or(Class::name_from_id(id))
        );

        quote_fn! {
            $(java::block_comment(iter::once(format!("{}", &self.unpacked))))$['\n']
            static void testDecode$test_id() {
                $(&packet_name) decodedPacket = $(&packet_name).fromBytes($(hex_to_array(&self.packed)));
                $(self.gen_asserts_for_fields(id, decls))
            }
        }
    }

    fn gen_asserts_for_fields<'a>(
        &'a self,
        id: &'a String,
        decls: &'a HashMap<String, Decl>,
    ) -> impl FormatInto<Java> + 'a {
        let fields = self.unpacked.as_object().unwrap();
        let maybe_child = self.packet.as_ref().unwrap_or(id);

        quote_fn! {
            $(for (field_id, value) in fields.iter() {
                $(let field = get_field(maybe_child, field_id, decls))
                assert $(field.equals(
                    field.construct(value, decls),
                    quote!(decodedPacket.get$(field_id.to_upper_camel_case())())));
            })
        }
    }
}

impl Field {
    fn construct<'a>(
        &'a self,
        value: &'a Value,
        decls: &'a HashMap<String, Decl>,
    ) -> impl FormatInto<Java> + 'a {
        match &self.desc {
            FieldDesc::Scalar { width, .. }
            | FieldDesc::Size { width, .. }
            | FieldDesc::Count { width, .. } => {
                literal(Integral::fitting(*width), json_val_to_usize(value))
            }
            FieldDesc::Reserved { width } => literal(Integral::fitting(*width), 0),
            FieldDesc::Typedef { type_id, .. } => {
                quote!($(get_decl(type_id, decls).desc.construct(value, decls)))
            }

            FieldDesc::Payload { .. } => {
                quote!(new byte[]{
                    $(for value in value.as_array().unwrap() join (, ) {
                        $(literal(Integral::Byte, json_val_to_usize(value)))
                    })
                })
            }

            FieldDesc::Array { width, type_id, .. } => {
                if let Some(width) = width {
                    let ty = Integral::fitting(*width);
                    quote!(new $ty[]{
                        $(for value in value.as_array().unwrap() join (, ) {
                            $(literal(ty, json_val_to_usize(value)))
                        })
                    })
                } else if let Some(id) = type_id {
                    let ty = get_decl(id, decls);
                    quote!(new $(Class::name_from_id(id))[]{
                        $(for value in value.as_array().unwrap() join (, ) {
                            $(ty.desc.construct(value, decls))
                        })
                    })
                } else {
                    panic!("invalid array element")
                }
            }
            other => {
                dbg!(other);
                todo!()
            }
        }
    }

    fn equals<'a>(
        &'a self,
        field: impl FormatInto<Java> + 'a,
        other: impl FormatInto<Java> + 'a,
    ) -> impl FormatInto<Java> + 'a {
        match &self.desc {
            FieldDesc::Scalar { .. } | FieldDesc::Size { .. } | FieldDesc::Count { .. } => {
                quote!($field == $other)
            }
            FieldDesc::Body | FieldDesc::Payload { .. } | FieldDesc::Array { .. } => {
                quote!(Arrays.equals($field, $other))
            }
            FieldDesc::Typedef { .. } | FieldDesc::FixedEnum { .. } => {
                quote!($field.equals($other))
            }
            other => {
                dbg!(other);
                todo!()
            }
        }
    }
}

impl DeclDesc {
    fn construct<'a>(
        &'a self,
        value: &'a Value,
        decls: &'a HashMap<String, Decl>,
    ) -> impl FormatInto<Java> + 'a {
        match self {
            DeclDesc::Enum { id, width, .. } => {
                let ty = Integral::fitting(*width);
                quote!($(Class::name_from_id(id)).from$(ty.capitalized())(
                    $(literal(ty, json_val_to_usize(value)))
                ))
            }
            DeclDesc::Struct { id, .. } => {
                quote!($(build_packet_from_fields(id, value.as_object().unwrap(), decls)))
            }
            other => {
                dbg!(other);
                todo!()
            }
        }
    }
}

fn build_packet_from_fields<'a>(
    id: &'a str,
    fields_json: &'a Map<String, Value>,
    decls: &'a HashMap<String, Decl>,
) -> impl FormatInto<Java> + 'a {
    let decl = get_decl(id, decls);
    let constraints: HashSet<&String> = decl.constraints().map(|c| &c.id).collect();
    let is_unknown_child = fields_json.contains_key("payload");

    quote_fn! {
        new $(if is_unknown_child => Unknown)$(Class::name_from_id(id)).Builder()
            $(for (field_id, value) in fields_json.iter() {
                $(if !constraints.contains(field_id) {
                    .set$(field_id.to_upper_camel_case())(
                        $(get_field(id, field_id, decls).construct(value, decls)))
                })
            })
            .build()
    }
}

fn hex_to_array(hex: &str) -> impl FormatInto<Java> + '_ {
    let bytes = hex
        .as_bytes()
        .chunks_exact(2)
        .map(|chunk| format!("(byte) 0x{}", str::from_utf8(chunk).unwrap()));

    quote_fn! {
        new byte[]{$(for byte in bytes join (, ) => $byte)}
    }
}

fn get_decl<'a>(id: &'a str, decls: &'a HashMap<String, Decl>) -> &'a Decl {
    decls.get(id).unwrap_or_else(|| panic!("Could not find decl {id}"))
}

fn get_field<'a>(id: &'a str, field_id: &'a str, decls: &'a HashMap<String, Decl>) -> &'a Field {
    let decl = get_decl(id, decls);
    let field = json_id_to_field(field_id, decl.fields());
    // dbg!(decl, id, field_id, field);

    if let Some(field) = field {
        field
    } else if let Some(parent_id) = decl.parent_id() {
        get_field(parent_id, field_id, decls)
    } else {
        panic!("field {} not found in packet {} in pdl file under test", field_id, id);
    }
}

fn json_id_to_field<'a>(field_id: &'a str, mut fields: Iter<'a, Field>) -> Option<&'a Field> {
    fields.find(|field| match field.desc {
        _ if field.id().is_some_and(|id| id == field_id) => true,
        FieldDesc::Payload { .. } if field_id == "payload" => true,
        FieldDesc::Body if field_id == "body" => true,
        _ => false,
    })
}

fn json_val_to_usize(val: &Value) -> usize {
    val.as_number().unwrap().as_u64().unwrap() as usize
}
