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

use pdl_derive::pdl_inline;

#[pdl_inline(
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
    #[test]
    fn test_try_from_parent() {
        // Success.
        assert_eq!(
            Child1::try_from(Parent { a: 1, payload: vec![42, 0, 0] }),
            Ok(Child1 { x: 42, payload: vec![0, 0] })
        );
        assert_eq!(Child2::try_from(Parent { a: 2, payload: vec![42, 0] }), Ok(Child2 { x: 42 }));
        assert_eq!(
            GrandChild1::try_from(Parent { a: 1, payload: vec![42, 0, 0] }),
            Ok(GrandChild1 { y: 0 })
        );
        assert_eq!(
            GrandChild1::try_from(Child1 { x: 42, payload: vec![0, 0] }),
            Ok(GrandChild1 { y: 0 })
        );

        // Invalid constraint value.
        assert!(matches!(
            Child2::try_from(Parent { a: 1, payload: vec![42, 0, 0] }),
            Err(DecodeError::InvalidFieldValue { .. })
        ));
        assert!(matches!(
            GrandChild1::try_from(Parent { a: 2, payload: vec![42, 0, 0] }),
            Err(DecodeError::InvalidFieldValue { .. })
        ));
        assert!(matches!(
            GrandChild1::try_from(Parent { a: 1, payload: vec![43, 0, 0] }),
            Err(DecodeError::InvalidFieldValue { .. })
        ));

        // Payload contains too many bytes.
        assert!(matches!(
            Child2::try_from(Parent { a: 2, payload: vec![42, 2, 3, 4] }),
            Err(DecodeError::TrailingBytes)
        ));
        assert!(matches!(
            GrandChild1::try_from(Parent { a: 1, payload: vec![42, 0, 1, 2] }),
            Err(DecodeError::TrailingBytes)
        ));
        assert!(matches!(
            GrandChild1::try_from(Child1 { x: 42, payload: vec![0, 1, 2] }),
            Err(DecodeError::TrailingBytes)
        ));

        // Payload contains too few bytes.
        assert!(matches!(
            Child1::try_from(Parent { a: 1, payload: vec![] }),
            Err(DecodeError::InvalidLengthError { .. })
        ));
        assert!(matches!(
            GrandChild1::try_from(Parent { a: 1, payload: vec![42, 0] }),
            Err(DecodeError::InvalidLengthError { .. })
        ));
        assert!(matches!(
            GrandChild1::try_from(Child1 { x: 42, payload: vec![0] }),
            Err(DecodeError::InvalidLengthError { .. })
        ));
    }

    #[test]
    fn test_try_from_child() {
        // Success.
        assert_eq!(
            Parent::try_from(Child1 { x: 42, payload: vec![0, 0] }),
            Ok(Parent { a: 1, payload: vec![42, 0, 0] })
        );
        assert_eq!(Parent::try_from(Child2 { x: 42 }), Ok(Parent { a: 2, payload: vec![42, 0] }));
        assert_eq!(
            Parent::try_from(GrandChild1 { y: 0 }),
            Ok(Parent { a: 1, payload: vec![42, 0, 0] })
        );
        assert_eq!(
            Child1::try_from(GrandChild1 { y: 0 }),
            Ok(Child1 { x: 42, payload: vec![0, 0] })
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
