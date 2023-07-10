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

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(warnings, missing_docs)]
#![allow(clippy::all)]
// this is now stable
#![feature(mixed_integer_ops)]

include!(concat!(env!("OUT_DIR"), "/_packets.rs"));

fn hex_to_word(hex: u8) -> u8 {
    if b'0' <= hex && hex <= b'9' {
        hex - b'0'
    } else if b'A' <= hex && hex <= b'F' {
        hex - b'A' + 0xa
    } else {
        hex - b'a' + 0xa
    }
}

fn hex_str_to_byte_vector(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks_exact(2)
        .map(|chunk| hex_to_word(chunk[1]) + (hex_to_word(chunk[0]) << 4))
        .collect()
}
