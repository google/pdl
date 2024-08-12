// Copyright 2024 Google LLC
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

enum Enum16 : 16 {
   X = 0x1234,
   Y = 0x5678,
}

packet CondTest {
  cond: 1,
  _reserved_ : 7,
  a: 8 if cond = 0,
  b: Enum16 if cond = 1,
}
"#
)]
#[cfg(test)]
mod optional_field {
    #[test]
    fn test_value_0() {
      // Success
      let test_value_0 = CondTest {
        a: Some(255),
        b: None,
      };
      let mut buf = vec![];
      assert!(test_value_0.encode(&mut buf).is_ok());

      let decoded_cond = CondTest::decode_full(&buf).unwrap();
      assert_eq!(decoded_cond.a, test_value_0.a);
      assert_eq!(decoded_cond.b, test_value_0.b);
    }

    #[test]
    fn test_value_1() {
      // Success
      let test_value_1 = CondTest {
        a: None,
        b: Some(Enum16::X),
      };
      let mut buf = vec![];
      assert!(test_value_1.encode(&mut buf).is_ok());

      let decoded_cond = CondTest::decode_full(&buf).unwrap();
      assert_eq!(decoded_cond.a, test_value_1.a);
      assert_eq!(decoded_cond.b, test_value_1.b);
    }

    #[test]
    fn test_value_inconsistent() {
      let test_value_none = CondTest {
        a: None,
        b: None,
      };
      assert!(matches!(test_value_none.encode_to_vec(), Err(EncodeError::InconsistentConditionValue { .. })));

      let test_value_both = CondTest {
        a: Some(255),
        b: Some(Enum16::X),
      };
      assert!(matches!(test_value_both.encode_to_vec(), Err(EncodeError::InconsistentConditionValue { .. })));
    }
}
