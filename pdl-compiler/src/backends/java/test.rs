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
    tokens::FormatInto,
    Tokens,
};
use heck::ToUpperCamelCase;
use serde_json::{Number, Value};
use std::{
    collections::{HashMap, HashSet},
    fs, iter,
    path::{Path, PathBuf},
    str,
};

use super::{import, Integral, JavaFile};
use crate::{
    ast::{self, Decl, DeclDesc, Field},
    backends::common::test::{Packet, TestVector},
    parser,
};

pub fn generate_tests(
    input_file: &str,
    output_dir: &Path,
    package: String,
    pdl_file_under_test: &str,
    exclude_packets: &Vec<String>,
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
            .flat_map(
                |decl| if let Some(id) = decl.id() { Some((String::from(id), decl)) } else { None },
            )
            .collect();

    // dbg!(&decls, decls.contains_key("ScalarParent"));

    JavaTest(get_test_cases(input_file, exclude_packets)?).write_to_fs(
        &output_file,
        &package,
        input_file,
        decls,
    )
}

fn get_test_cases(file: &str, exclude_packets: &Vec<String>) -> Result<Vec<Packet>, String> {
    let data = fs::read_to_string(file).map_err(|err| err.to_string())?;
    let mut packets: Vec<Packet> = serde_json::from_str(&data).map_err(|err| err.to_string())?;

    eprintln!("Read {} test vectors from {file}", packets.len());

    packets.retain(|p| !exclude_packets.contains(&p.name));
    for packet in packets.iter_mut() {
        packet.tests.retain(|t| !t.packet.as_ref().is_some_and(|p| exclude_packets.contains(&p)));
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
                            TEST_$(&packet.name).testEncode$i();
                            TEST_$(&packet.name).testDecode$i();
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
                $(maybe_child.to_upper_camel_case()) packet = $(self.build_packet_from_fields(id, decls));
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
        let packet_name = id.to_upper_camel_case();

        quote_fn! {
            $(java::block_comment(iter::once(format!("{}", &self.unpacked))))$['\n']
            static void testDecode$test_id() {
                $(if let Some(child) = &self.packet {
                    $(&packet_name) genericDecodedPacket = $(&packet_name).fromBytes($(hex_to_array(&self.packed)));
                    if (!(genericDecodedPacket instanceof $(child.to_upper_camel_case()) decodedPacket))
                        throw new AssertionError();
                } else {
                    $(&packet_name) decodedPacket = $(&packet_name).fromBytes($(hex_to_array(&self.packed)));
                })
                $(self.gen_asserts_for_fields(id, decls))
            }
        }
    }

    fn build_packet_from_fields<'a>(
        &'a self,
        id: &'a String,
        decls: &'a HashMap<String, Decl>,
    ) -> impl FormatInto<Java> + 'a {
        let fields_json = self.unpacked.as_object().unwrap();
        let maybe_child = self.packet.as_ref().unwrap_or(id);
        let decl = get_decl(maybe_child, decls);
        let constraints: HashSet<&String> = decl.constraints().map(|c| &c.id).collect();

        quote_fn! {
            new $(maybe_child.to_upper_camel_case()).Builder()
                $(for (field_id, value) in fields_json.iter() {
                    $(if !constraints.contains(field_id) {
                        .set$(field_id.to_upper_camel_case())(
                            $(get_field(maybe_child, field_id, decls).construct(value, decls)))
                    })
                })
                .build()
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
            ast::FieldDesc::Scalar { width, .. } => {
                Integral::fitting(*width).literal(value.as_number().unwrap().as_u64())
            }
            ast::FieldDesc::Typedef { type_id, .. } => match &get_decl(&type_id, decls).desc {
                DeclDesc::Enum { width, .. } => {
                    let ty = Integral::fitting(*width);
                    quote!(
                        $(type_id.to_upper_camel_case())
                            .from$(ty.capitalized())($(ty.literal(value.as_number().unwrap().as_u64()))))
                }
                DeclDesc::Checksum { id, function, width } => todo!(),
                DeclDesc::CustomField { id, width, function } => todo!(),
                DeclDesc::Packet { id, constraints, fields, parent_id } => todo!(),
                DeclDesc::Struct { id, constraints, fields, parent_id } => todo!(),
                DeclDesc::Group { id, fields } => todo!(),
                DeclDesc::Test { type_id, test_cases } => todo!(),
            },
            ast::FieldDesc::Checksum { field_id } => todo!(),
            ast::FieldDesc::Padding { size } => todo!(),
            ast::FieldDesc::Size { field_id, width } => todo!(),
            ast::FieldDesc::Count { field_id, width } => todo!(),
            ast::FieldDesc::ElementSize { field_id, width } => todo!(),
            ast::FieldDesc::Body => todo!(),
            ast::FieldDesc::Payload { size_modifier } => todo!(),
            ast::FieldDesc::FixedScalar { width, value } => todo!(),
            ast::FieldDesc::FixedEnum { enum_id, tag_id } => todo!(),
            ast::FieldDesc::Reserved { width } => todo!(),
            ast::FieldDesc::Array { id, width, type_id, size_modifier, size } => todo!(),
            ast::FieldDesc::Flag { id, optional_field_ids } => todo!(),
            ast::FieldDesc::Group { group_id, constraints } => todo!(),
        }
    }

    fn equals<'a>(
        &'a self,
        field: impl FormatInto<Java> + 'a,
        other: impl FormatInto<Java> + 'a,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(match &self.desc {
                ast::FieldDesc::Scalar { .. } => $field == $other,
                ast::FieldDesc::Body
                | ast::FieldDesc::Payload { .. } => Arrays.equals($field, $other),
                ast::FieldDesc::Typedef { .. }
                | ast::FieldDesc::FixedEnum { .. } => $field.equals($other),
                ast::FieldDesc::Checksum { .. } => todo!(),
                ast::FieldDesc::Padding { .. } => todo!(),
                ast::FieldDesc::Size { .. } => todo!(),
                ast::FieldDesc::Count { .. } => todo!(),
                ast::FieldDesc::ElementSize { .. } => todo!(),
                ast::FieldDesc::FixedScalar { .. } => todo!(),
                ast::FieldDesc::Reserved { .. } => todo!(),
                ast::FieldDesc::Array { .. } => todo!(),
                ast::FieldDesc::Flag { .. } => todo!(),
                ast::FieldDesc::Group { .. } => todo!(),
            })
        }
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
    decls.get(id).expect(&format!("Could not find decl {}", id))
}

fn get_field<'a>(id: &'a str, field_id: &'a str, decls: &'a HashMap<String, Decl>) -> &'a Field {
    let decl = get_decl(id, decls);
    let field = decl.fields().find(|field| field.id().is_some_and(|id| id == field_id));
    // dbg!(decl, id, field_id);

    if let Some(field) = field {
        field
    } else if let Some(parent_id) = decl.parent_id() {
        get_field(parent_id, field_id, decls)
    } else {
        panic!("packet {} not found in pdl file under test", id);
    }
}
