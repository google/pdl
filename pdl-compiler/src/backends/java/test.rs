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
    prelude::{java, quote_fn, quote_in},
    tokens::FormatInto,
    Tokens,
};
use heck::ToUpperCamelCase;
use serde_json::{Number, Value};
use std::{
    fs, iter,
    path::{Path, PathBuf},
    str,
};

use super::{import, write_tokens_to_file};
use crate::backends::common::test::{Packet, TestVector};

pub fn generate_tests(input_file: &str, output_dir: &Path, package: String) -> Result<(), String> {
    let packets_for_test = ["Packet_Scalar_Field"];

    let mut dir = PathBuf::from(output_dir);
    dir.extend(package.split("."));
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    JavaTestFile { packets: get_test_cases(input_file, &packets_for_test)?, package, dir }
        .generate()
}

fn get_test_cases(file: &str, packet_names: &[&str]) -> Result<Vec<Packet>, String> {
    eprintln!("Reading test vectors from {file}, will use {} packets", packet_names.len());

    let data = fs::read_to_string(file).map_err(|err| err.to_string())?;
    let mut packets: Vec<Packet> = serde_json::from_str(&data).map_err(|err| err.to_string())?;
    packets.retain(|p| packet_names.contains(&p.name.as_str()));
    Ok(packets)
}

struct JavaTestFile {
    package: String,
    dir: PathBuf,
    packets: Vec<Packet>,
}

impl JavaTestFile {
    pub fn generate(&self) -> Result<(), String> {
        let mut tokens = java::Tokens::new();
        self.format_into(&mut tokens);

        write_tokens_to_file(
            self.dir.join("PdlTests").with_extension("java"),
            &self.package,
            tokens,
        )
    }
}

impl FormatInto<Java> for &JavaTestFile {
    fn format_into(self, tokens: &mut java::Tokens) {
        quote_in!(*tokens =>
            final class PdlTests {
                $(for packet in self.packets.iter() => $packet)

                public static void main(String[] args) {
                    $(for packet in self.packets.iter() {
                        $(for (i, _) in packet.tests.iter().enumerate() {
                            $(&packet.name).testEncode$i();
                            $(&packet.name).testDecode$i();
                        })
                    })
                    System.out.println("All tests passed!");
                }
            }
        )
    }
}

impl FormatInto<Java> for &Packet {
    fn format_into(self, tokens: &mut Tokens<Java>) {
        let packet_name = &self.name.to_upper_camel_case();
        quote_in!(*tokens =>
            static final class $(&self.name) {
                $(for (i, test_case) in self.tests.iter().enumerate() {
                    $(test_case.encoder_test(packet_name, i))

                    $(test_case.decoder_test(packet_name, i))
                })
            }
        )
    }
}

impl TestVector {
    fn encoder_test<'a>(&'a self, packet_name: &'a str, id: usize) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(java::block_comment(iter::once(format!("0x{}", &self.packed))))$['\n']
            static void testEncode$id() {
                $packet_name packet = $(build_packet_from_fields(packet_name, &self.unpacked));
                byte[] encodedPacket = packet.toBytes();
                byte[] expectedBytes = $(hex_to_array(&self.packed));
                assert $(&*import::ARRAYS).equals(expectedBytes, encodedPacket);
            }
        }
    }

    fn decoder_test<'a>(&'a self, packet_name: &'a str, id: usize) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(java::block_comment(iter::once(format!("{}", &self.unpacked))))$['\n']
            static void testDecode$id() {
                $packet_name decodedPacket = $packet_name.fromBytes($(hex_to_array(&self.packed)));
                $(gen_asserts_for_fields(&self.unpacked))
            }
        }
    }
}

fn build_packet_from_fields<'a>(name: &'a str, fields: &'a Value) -> impl FormatInto<Java> + 'a {
    let fields = fields.as_object().unwrap();

    quote_fn! {
        new $name.Builder()
            $(for (field, value) in fields.iter() => .set$(field.to_upper_camel_case())(
                $(match value {
                    Value::Number(n) => $(num_to_lit(n)),
                    _ => todo!(),
                })))
            .build()
    }
}

fn gen_asserts_for_fields<'a>(fields: &'a Value) -> impl FormatInto<Java> + 'a {
    let fields = fields.as_object().unwrap();

    quote_fn! {
        $(for (field, value) in fields.iter() =>
            $(match value {
                Value::Number(n) => assert $(num_to_lit(n)) == decodedPacket.get$(field.to_upper_camel_case())();,
                _ => todo!(),
            }))
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

fn num_to_lit(n: &Number) -> impl FormatInto<Java> + '_ {
    quote_fn! {
        $(let n = n.as_u64().unwrap()) $n$(if n > u32::MAX as u64 => L)
    }
}
