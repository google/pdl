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

//! PDL parser and analyzer.

pub mod analyzer;
pub mod ast;
pub mod backends;
pub mod parser;
#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rust_no_allocation_output_is_deterministic() {
        // The generated code should be deterministic, to avoid unnecessary rebuilds during
        // incremental builds.
        let src = r#"
little_endian_packets

enum Enum1 : 8 {
  ENUM_VARIANT_ONE = 0x01,
  ENUM_VARIANT_TWO = 0x02,
}

packet Packet1 {
  opcode : Enum1,
  _payload_,
}

struct Struct1 {
  handle : 16,
}

struct Struct2 {
  _payload_
}

struct Struct3 {
  handle : Struct1,
  value : Struct2,
}

packet Packet2 : Packet1(opcode = ENUM_VARIANT_ONE) {
  handle : Struct1,
  value : Struct2,
}
"#
        .to_owned();

        let mut sources1 = ast::SourceDatabase::new();
        let mut sources2 = ast::SourceDatabase::new();
        let mut sources3 = ast::SourceDatabase::new();

        let file1 = parser::parse_inline(&mut sources1, "foo", src.clone()).unwrap();
        let file2 = parser::parse_inline(&mut sources2, "foo", src.clone()).unwrap();
        let file3 = parser::parse_inline(&mut sources3, "foo", src).unwrap();

        let schema1 = backends::intermediate::generate(&file1).unwrap();
        let schema2 = backends::intermediate::generate(&file2).unwrap();
        let schema3 = backends::intermediate::generate(&file3).unwrap();

        let result1 = backends::rust_no_allocation::generate(&file1, &schema1).unwrap();
        let result2 = backends::rust_no_allocation::generate(&file2, &schema2).unwrap();
        let result3 = backends::rust_no_allocation::generate(&file3, &schema3).unwrap();

        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}
