// Copyright 2026 Google LLC
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

use pdl_derive::pdl_inline;

#[pdl_inline(
    r#"
little_endian_packets

enum TestEnum : 8 {
    A = 1,
    B = 2,
    OTHER = ..,
}

enum TestEnumRanges : 8 {
    A = 1 .. 10,
    B = 11 .. 20,
    OTHER = ..,
}

struct TestStruct {
    _count_(a) : 8,
    a: TestEnum[],
}

packet TestPacket {
  a: 8,
  b: 1,
  _reserved_ : 7,
  c: TestEnum,
  d: 8[40],
  e: TestStruct,
  f: 16 if b = 0,
  _payload_
}
"#
)]
#[cfg(test)]
mod default_trait {
    #[test]
    fn test() {
        assert_eq!(TestEnum::default(), TestEnum::A);
        assert_eq!(TestEnumRanges::default(), TestEnumRanges::try_from(1).unwrap());

        let default_struct = TestStruct::default();
        assert_eq!(default_struct.a.len(), 0);

        let default_packet = TestPacket::default();
        assert_eq!(default_packet.a, 0);
        assert_eq!(default_packet.c, TestEnum::A);
        assert_eq!(default_packet.d, [0; 40]);
        assert_eq!(default_packet.e, default_struct);
        assert_eq!(default_packet.f, None);
    }
}
