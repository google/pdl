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

packet Test {
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
    fn test_ok_value_0() {
        let value = Test { a: Some(255), b: None };
        let mut encoded_value = vec![];

        // The optional fields provide both the same value 0.
        assert!(value.encode(&mut encoded_value).is_ok());
        assert_eq!(Test::decode_full(&encoded_value), Ok(value));
    }

    #[test]
    fn test_ok_value_1() {
        let value = Test { a: None, b: Some(Enum16::X) };
        let mut encoded_value = vec![];

        // The optional fields provide both the same value 0.
        assert!(value.encode(&mut encoded_value).is_ok());
        assert_eq!(Test::decode_full(&encoded_value), Ok(value));
    }

    #[test]
    fn test_err_inconsistent_condition_value() {
        // The optional fields would provide the value 1 and 0
        // for the condition flag.
        assert!(matches!(
            Test { a: None, b: None }.encode_to_vec(),
            Err(EncodeError::InconsistentConditionValue { .. })
        ));

        // The optional fields would provide the value 0 and 1
        // for the condition flag.
        assert!(matches!(
            Test { a: Some(255), b: Some(Enum16::X) }.encode_to_vec(),
            Err(EncodeError::InconsistentConditionValue { .. })
        ));
    }
}

#[pdl_inline(
    r#"
little_endian_packets

struct Elem {
    tag : 16,
    _size_(value) : 16,
    value : 8[],
}

packet Parent {
    a : 8,
    _size_(_payload_) : 8,
    _payload_,
}

packet Child : Parent(a = 42) {
    b : Elem,
    c : Elem,
}
"#
)]
#[cfg(test)]
mod payload_size_field {
    #[test]
    fn test_ok() {
        assert_eq!(Parent { a: 1, payload: vec![] }.encode_to_vec(), Ok(vec![1, 0]));
    }

    #[test]
    fn test_err_size_overflow() {
        // Attempting to encode a packet with a payload size exceeding
        // the range for the size field must fail.
        assert!(matches!(
            Parent { a: 42, payload: vec![1; 1024] }.encode_to_vec(),
            Err(EncodeError::SizeOverflow { .. })
        ));
        assert!(matches!(
            Child {
                b: Elem { tag: 42, value: vec![1; 42] },
                c: Elem { tag: 42, value: vec![1; 1024] }
            }
            .encode_to_vec(),
            Err(EncodeError::SizeOverflow { .. })
        ));
    }
}
