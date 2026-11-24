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

#![no_std]

extern crate alloc;

use pdl_derive::pdl_inline_nostd;

#[pdl_inline_nostd(
    r#"
little_endian_packets

packet Parent {
  a: 8,
  _payload_
}

packet Child1 : Parent (a = 1) {
  x: 8,
  _payload_
}

packet Child2 : Parent (a = 2) {
  x: 16,
}

packet GrandChild1 : Child1 (x = 42) {
  y: 12,
  _reserved_: 4,
}
"#
)]
#[cfg(test)]
mod convert {
    use alloc::vec;

    #[test]
    fn test_try_from_parent() {
        // Success with borrowed slices.
        assert_eq!(
            Child1::try_from(Parent { a: 1, payload: (&[42, 0, 0]).into() }),
            Ok(Child1 { x: 42, payload: (&[0, 0]).into() })
        );
        // Success with owned vectors (temporary rvalue).
        assert_eq!(
            Child1::try_from(Parent { a: 1, payload: vec![42, 0, 0].into() }),
            Ok(Child1 { x: 42, payload: (&[0, 0]).into() })
        );
        assert_eq!(
            Child2::try_from(Parent { a: 2, payload: (&[42, 0]).into() }),
            Ok(Child2 { x: 42 })
        );
        assert_eq!(
            GrandChild1::try_from(Parent { a: 1, payload: (&[42, 0, 0]).into() }),
            Ok(GrandChild1 { y: 0 })
        );
        assert_eq!(
            GrandChild1::try_from(Child1 { x: 42, payload: (&[0, 0]).into() }),
            Ok(GrandChild1 { y: 0 })
        );

        // Invalid constraint value.
        assert!(matches!(
            Child2::try_from(Parent { a: 1, payload: (&[42, 0, 0]).into() }),
            Err(DecodeError::ConstraintValueError { .. })
        ));
        assert!(matches!(
            GrandChild1::try_from(Parent { a: 2, payload: (&[42, 0, 0]).into() }),
            Err(DecodeError::ConstraintValueError { .. })
        ));
        assert!(matches!(
            GrandChild1::try_from(Parent { a: 1, payload: (&[43, 0, 0]).into() }),
            Err(DecodeError::ConstraintValueError { .. })
        ));

        // Payload contains too many bytes.
        assert!(matches!(
            Child2::try_from(Parent { a: 2, payload: (&[42, 2, 3, 4]).into() }),
            Err(DecodeError::TrailingBytesError)
        ));
        assert!(matches!(
            GrandChild1::try_from(Parent { a: 1, payload: (&[42, 0, 1, 2]).into() }),
            Err(DecodeError::TrailingBytesError)
        ));
        assert!(matches!(
            GrandChild1::try_from(Child1 { x: 42, payload: (&[0, 1, 2]).into() }),
            Err(DecodeError::TrailingBytesError)
        ));

        // Payload contains too few bytes.
        assert!(matches!(
            Child1::try_from(Parent { a: 1, payload: (&[]).into() }),
            Err(DecodeError::LengthError { .. })
        ));
        assert!(matches!(
            GrandChild1::try_from(Parent { a: 1, payload: (&[42, 0]).into() }),
            Err(DecodeError::LengthError { .. })
        ));
        assert!(matches!(
            GrandChild1::try_from(Child1 { x: 42, payload: (&[0]).into() }),
            Err(DecodeError::LengthError { .. })
        ));
    }

    #[test]
    fn test_try_from_child() {
        assert_eq!(
            Parent::try_from(Child1 { x: 42, payload: (&[0, 0]).into() }),
            Ok(Parent { a: 1, payload: (&[42, 0, 0]).into() })
        );
        assert_eq!(
            Parent::try_from(Child2 { x: 42 }),
            Ok(Parent { a: 2, payload: (&[42, 0]).into() })
        );
        assert_eq!(
            Parent::try_from(GrandChild1 { y: 0 }),
            Ok(Parent { a: 1, payload: (&[42, 0, 0]).into() })
        );
        assert_eq!(
            Child1::try_from(GrandChild1 { y: 0 }),
            Ok(Child1 { x: 42, payload: (&[0, 0]).into() })
        );

        // Invalid scalar value.
        assert!(matches!(
            Parent::try_from(GrandChild1 { y: 0xffff }),
            Err(EncodeError::InvalidScalarValue { .. })
        ));
        assert!(matches!(
            Child1::try_from(GrandChild1 { y: 0xffff }),
            Err(EncodeError::InvalidScalarValue { .. })
        ));
    }
}

#[pdl_inline_nostd(
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
    use alloc::vec;

    #[test]
    fn test_ok_value_0() {
        let value = Test { a: Some(255), b: None };
        let mut encoded_value = vec![];

        assert!(value.encode(&mut encoded_value).is_ok());
        assert_eq!(Test::decode_full(&encoded_value), Ok(value));
    }

    #[test]
    fn test_ok_value_1() {
        let value = Test { a: None, b: Some(Enum16::X) };
        let mut encoded_value = vec![];

        assert!(value.encode(&mut encoded_value).is_ok());
        assert_eq!(Test::decode_full(&encoded_value), Ok(value));
    }

    #[test]
    fn test_err_inconsistent_condition_value() {
        assert!(matches!(
            Test { a: None, b: None }.encode_to_vec(),
            Err(EncodeError::InconsistentConditionValue { .. })
        ));

        assert!(matches!(
            Test { a: Some(255), b: Some(Enum16::X) }.encode_to_vec(),
            Err(EncodeError::InconsistentConditionValue { .. })
        ));
    }
}

#[pdl_inline_nostd(
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
    use alloc::vec;

    #[test]
    fn test_ok() {
        assert_eq!(Parent { a: 1, payload: (&[]).into() }.encode_to_vec(), Ok(vec![1, 0]));
    }

    #[test]
    fn test_err_size_overflow() {
        assert!(matches!(
            Parent { a: 42, payload: (&[1u8; 1024]).into() }.encode_to_vec(),
            Err(EncodeError::SizeOverflow { .. })
        ));
        assert!(matches!(
            Child {
                b: Elem { tag: 42, value: (&[1u8; 42]).into() },
                c: Elem { tag: 42, value: (&[1u8; 1024]).into() }
            }
            .encode_to_vec(),
            Err(EncodeError::SizeOverflow { .. })
        ));
    }
}

#[pdl_inline_nostd(
    r#"
little_endian_packets

packet Parent {
  a: 8,
  _payload_
}

packet Alias : Parent {
  _payload_
}

packet Child1 : Parent (a = 1) {
  x: 16[],
}

packet Child2 : Alias (a = 2) {
  x: 8,
}

packet Child3 : Alias (a = 3) {
  x: 16,
}
"#
)]
#[cfg(test)]
mod child_determined_by_child_constraint {
    #[test]
    fn test_child1() {
        let parent = Parent::decode_full(&[1, 42, 43]).unwrap();
        assert_eq!(
            parent.specialize(),
            Ok(ParentChild::Child1(Child1 { x: (&[11050u16]).into() }))
        );

        let parent = Parent::decode_full(&[1, 2, 3, 4]).unwrap();
        assert_eq!(parent.a, 1);
        assert!(matches!(parent.specialize(), Err(DecodeError::ArraySizeError { .. })));
    }

    #[test]
    fn test_alias() {
        let parent = Parent::decode_full(&[2, 42]).unwrap();
        assert_eq!(
            parent.specialize(),
            Ok(ParentChild::Alias(Alias { a: 2, payload: (&[42u8]).into() }))
        );
    }
}

#[pdl_inline_nostd(
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
