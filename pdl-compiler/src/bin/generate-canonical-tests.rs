// Copyright 2023 Google LLC
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

//! Generate Rust unit tests for canonical test vectors.

use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Packet {
    #[serde(rename = "packet")]
    name: String,
    tests: Vec<TestVector>,
}

#[derive(Debug, Deserialize)]
struct TestVector {
    packed: String,
    unpacked: Value,
    packet: Option<String>,
}

/// Convert a string of hexadecimal characters into a Rust vector of
/// bytes.
///
/// The string `"80038302"` becomes `vec![0x80, 0x03, 0x83, 0x02]`.
fn hexadecimal_to_vec(hex: &str) -> proc_macro2::TokenStream {
    assert!(hex.len() % 2 == 0, "Expects an even number of hex digits");
    let bytes = hex.as_bytes().chunks_exact(2).map(|chunk| {
        let number = format!("0x{}", std::str::from_utf8(chunk).unwrap());
        syn::parse_str::<syn::LitInt>(&number).unwrap()
    });

    quote! {
        vec![#(#bytes),*]
    }
}

/// Convert `value` to a JSON string literal.
///
/// The string literal is a raw literal to avoid escaping
/// double-quotes.
fn to_json<T: Serialize>(value: &T) -> syn::LitStr {
    let json = serde_json::to_string(value).unwrap();
    assert!(!json.contains("\"#"), "Please increase number of # for {json:?}");
    syn::parse_str::<syn::LitStr>(&format!("r#\" {json} \"#")).unwrap()
}

fn generate_unit_tests(input: &str, packet_names: &[&str], module_name: &str) {
    eprintln!("Reading test vectors from {input}, will use {} packets", packet_names.len());

    let data = std::fs::read_to_string(input)
        .unwrap_or_else(|err| panic!("Could not read {input}: {err}"));
    let packets: Vec<Packet> = serde_json::from_str(&data).expect("Could not parse JSON");

    let module = format_ident!("{}", module_name);
    let mut tests = Vec::new();
    for packet in &packets {
        for (i, test_vector) in packet.tests.iter().enumerate() {
            let test_packet = test_vector.packet.as_deref().unwrap_or(packet.name.as_str());
            if !packet_names.contains(&test_packet) {
                eprintln!("Skipping packet {}", test_packet);
                continue;
            }
            eprintln!("Generating tests for packet {}", test_packet);

            let parse_test_name = format_ident!(
                "test_parse_{}_vector_{}_0x{}",
                test_packet,
                i + 1,
                &test_vector.packed
            );
            let serialize_test_name = format_ident!(
                "test_serialize_{}_vector_{}_0x{}",
                test_packet,
                i + 1,
                &test_vector.packed
            );
            let packed = hexadecimal_to_vec(&test_vector.packed);
            let packet_name = format_ident!("{}", test_packet);
            let builder_name = format_ident!("{}Builder", test_packet);

            let object = test_vector.unpacked.as_object().unwrap_or_else(|| {
                panic!("Expected test vector object, found: {}", test_vector.unpacked)
            });
            let assertions = object.iter().map(|(key, value)| {
                let getter = format_ident!("get_{key}");
                let expected = format_ident!("expected_{key}");
                let json = to_json(&value);
                quote! {
                    let #expected: serde_json::Value = serde_json::from_str(#json)
                        .expect("Could not create expected value from canonical JSON data");
                    assert_eq!(json!(actual.#getter()), #expected);
                }
            });

            let json = to_json(&object);
            tests.push(quote! {
                #[test]
                fn #parse_test_name() {
                    let packed = #packed;
                    let actual = #module::#packet_name::parse(&packed).unwrap();
                    #(#assertions)*
                }

                #[test]
                fn #serialize_test_name() {
                    let builder: #module::#builder_name = serde_json::from_str(#json)
                        .expect("Could not create builder from canonical JSON data");
                    let packet = builder.build();
                    let packed: Vec<u8> = #packed;
                    assert_eq!(packet.to_vec(), packed);
                }
            });
        }
    }

    // TODO(mgeisler): make the generated code clean from warnings.
    let code = quote! {
        #![allow(warnings, missing_docs)]

        use #module::Packet;
        use serde_json::json;

        #(#tests)*
    };
    let syntax_tree = syn::parse2::<syn::File>(code).expect("Could not parse {code:#?}");
    println!("{}", prettyplease::unparse(&syntax_tree));
}

fn main() {
    let input_path = std::env::args().nth(1).expect("Need path to JSON file with test vectors");
    let module_name = std::env::args().nth(2).expect("Need name for the generated module");
    // TODO(mgeisler): remove the `packet_names` argument when we
    // support all canonical packets.
    generate_unit_tests(
        &input_path,
        &[
            "EnumChild_A",
            "EnumChild_B",
            "Packet_Array_Field_ByteElement_ConstantSize",
            "Packet_Array_Field_ByteElement_UnknownSize",
            "Packet_Array_Field_ByteElement_VariableCount",
            "Packet_Array_Field_ByteElement_VariableSize",
            "Packet_Array_Field_EnumElement",
            "Packet_Array_Field_EnumElement_ConstantSize",
            "Packet_Array_Field_EnumElement_UnknownSize",
            "Packet_Array_Field_EnumElement_VariableCount",
            "Packet_Array_Field_EnumElement_VariableCount",
            "Packet_Array_Field_ScalarElement",
            "Packet_Array_Field_ScalarElement_ConstantSize",
            "Packet_Array_Field_ScalarElement_UnknownSize",
            "Packet_Array_Field_ScalarElement_VariableCount",
            "Packet_Array_Field_ScalarElement_VariableSize",
            "Packet_Array_Field_SizedElement_ConstantSize",
            "Packet_Array_Field_SizedElement_UnknownSize",
            "Packet_Array_Field_SizedElement_VariableCount",
            "Packet_Array_Field_SizedElement_VariableSize",
            "Packet_Array_Field_UnsizedElement_ConstantSize",
            "Packet_Array_Field_UnsizedElement_UnknownSize",
            "Packet_Array_Field_UnsizedElement_VariableCount",
            "Packet_Array_Field_UnsizedElement_VariableSize",
            "Packet_Array_Field_SizedElement_VariableSize_Padded",
            "Packet_Array_Field_UnsizedElement_VariableCount_Padded",
            "Packet_Body_Field_UnknownSize",
            "Packet_Body_Field_UnknownSize_Terminal",
            "Packet_Body_Field_VariableSize",
            "Packet_Count_Field",
            "Packet_Enum8_Field",
            "Packet_Enum_Field",
            "Packet_FixedEnum_Field",
            "Packet_FixedScalar_Field",
            "Packet_Payload_Field_UnknownSize",
            "Packet_Payload_Field_UnknownSize_Terminal",
            "Packet_Payload_Field_VariableSize",
            "Packet_Reserved_Field",
            "Packet_Scalar_Field",
            "Packet_Size_Field",
            "Packet_Struct_Field",
            "ScalarChild_A",
            "ScalarChild_B",
            "Struct_Count_Field",
            "Struct_Array_Field_ByteElement_ConstantSize",
            "Struct_Array_Field_ByteElement_UnknownSize",
            "Struct_Array_Field_ByteElement_UnknownSize",
            "Struct_Array_Field_ByteElement_VariableCount",
            "Struct_Array_Field_ByteElement_VariableCount",
            "Struct_Array_Field_ByteElement_VariableSize",
            "Struct_Array_Field_ByteElement_VariableSize",
            "Struct_Array_Field_EnumElement_ConstantSize",
            "Struct_Array_Field_EnumElement_UnknownSize",
            "Struct_Array_Field_EnumElement_UnknownSize",
            "Struct_Array_Field_EnumElement_VariableCount",
            "Struct_Array_Field_EnumElement_VariableCount",
            "Struct_Array_Field_EnumElement_VariableSize",
            "Struct_Array_Field_EnumElement_VariableSize",
            "Struct_Array_Field_ScalarElement_ConstantSize",
            "Struct_Array_Field_ScalarElement_UnknownSize",
            "Struct_Array_Field_ScalarElement_UnknownSize",
            "Struct_Array_Field_ScalarElement_VariableCount",
            "Struct_Array_Field_ScalarElement_VariableCount",
            "Struct_Array_Field_ScalarElement_VariableSize",
            "Struct_Array_Field_ScalarElement_VariableSize",
            "Struct_Array_Field_SizedElement_ConstantSize",
            "Struct_Array_Field_SizedElement_UnknownSize",
            "Struct_Array_Field_SizedElement_UnknownSize",
            "Struct_Array_Field_SizedElement_VariableCount",
            "Struct_Array_Field_SizedElement_VariableCount",
            "Struct_Array_Field_SizedElement_VariableSize",
            "Struct_Array_Field_SizedElement_VariableSize",
            "Struct_Array_Field_UnsizedElement_ConstantSize",
            "Struct_Array_Field_UnsizedElement_UnknownSize",
            "Struct_Array_Field_UnsizedElement_UnknownSize",
            "Struct_Array_Field_UnsizedElement_VariableCount",
            "Struct_Array_Field_UnsizedElement_VariableCount",
            "Struct_Array_Field_UnsizedElement_VariableSize",
            "Struct_Array_Field_UnsizedElement_VariableSize",
            "Struct_Array_Field_SizedElement_VariableSize_Padded",
            "Struct_Array_Field_UnsizedElement_VariableCount_Padded",
            "Struct_Enum_Field",
            "Struct_FixedEnum_Field",
            "Struct_FixedScalar_Field",
            "Struct_Size_Field",
            "Struct_Struct_Field",
        ],
        &module_name,
    );
}
