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

packet Parent {
  a: 8,
  _payload_
}

packet Child1 : Parent (a = 1) {
  x: 8,
}

packet Child2 : Parent (a = 2) {
  x: 16,
}
"#
)]
#[cfg(test)]
mod child_determined_by_constraint {
    #[test]
    fn test_child1() {
        // Success
        let parent = Parent::decode_full(&[1, 42]).unwrap();
        assert_eq!(parent.a, 1);
        assert_eq!(parent.specialize(), Ok(ParentChild::Child1(Child1 { x: 42 })));

        // Payload contains too many bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[1, 2, 3]).unwrap();
        assert_eq!(parent.a, 1);
        assert!(matches!(parent.specialize(), Err(DecodeError::TrailingBytes)));

        // Payload contains too few bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[1]).unwrap();
        assert_eq!(parent.a, 1);
        assert!(matches!(parent.specialize(), Err(DecodeError::InvalidLengthError { .. })));
    }

    #[test]
    fn test_child2() {
        // Success
        let parent = Parent::decode_full(&[2, 42, 43]).unwrap();
        assert_eq!(parent.a, 2);
        assert_eq!(parent.specialize(), Ok(ParentChild::Child2(Child2 { x: 11050 })));

        // Payload contains too many bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[2, 2, 3, 4]).unwrap();
        assert_eq!(parent.a, 2);
        assert!(matches!(parent.specialize(), Err(DecodeError::TrailingBytes)));

        // Payload contains too few bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[2, 0]).unwrap();
        assert_eq!(parent.a, 2);
        assert!(matches!(parent.specialize(), Err(DecodeError::InvalidLengthError { .. })));
    }

    #[test]
    fn test_none() {
        // No child matches the constraint.
        let parent = Parent::decode_full(&[4, 0]).unwrap();
        assert_eq!(parent.specialize(), Ok(ParentChild::None));
    }
}

#[pdl_inline(
    r#"
little_endian_packets

packet Parent {
  a: 8,
  _payload_
}

packet Child : Parent (a = 1) {
  b: 8,
  _payload_,
}

packet Grandchild1 : Child (b = 1) {
  x: 8,
}

packet Grandchild2 : Child (b = 2) {
  x: 16,
}
"#
)]
#[cfg(test)]
mod grandchild_determined_by_constraint {
    #[test]
    fn test_grandchild1() {
        // Success
        let child = Child::decode_full(&[1, 1, 42]).unwrap();
        assert_eq!(child.b, 1);
        assert_eq!(child.specialize(), Ok(ChildChild::Grandchild1(Grandchild1 { x: 42 })));

        // Payload contains too many bytes; specialize fails to produce
        // a valid child.
        let child = Child::decode_full(&[1, 1, 2, 3]).unwrap();
        assert_eq!(child.b, 1);
        assert!(matches!(child.specialize(), Err(DecodeError::TrailingBytes)));

        // Payload contains too few bytes; specialize fails to produce
        // a valid child.
        let child = Child::decode_full(&[1, 1]).unwrap();
        assert_eq!(child.b, 1);
        assert!(matches!(child.specialize(), Err(DecodeError::InvalidLengthError { .. })));
    }

    #[test]
    fn test_grandchild2() {
        // Success
        let child = Child::decode_full(&[1, 2, 42, 43]).unwrap();
        assert_eq!(child.b, 2);
        assert_eq!(child.specialize(), Ok(ChildChild::Grandchild2(Grandchild2 { x: 11050 })));

        // Payload contains too many bytes; specialize fails to produce
        // a valid child.
        let child = Child::decode_full(&[1, 2, 2, 3, 4]).unwrap();
        assert_eq!(child.b, 2);
        assert!(matches!(child.specialize(), Err(DecodeError::TrailingBytes)));

        // Payload contains too few bytes; specialize fails to produce
        // a valid child.
        let child = Child::decode_full(&[1, 2, 0]).unwrap();
        assert_eq!(child.b, 2);
        assert!(matches!(child.specialize(), Err(DecodeError::InvalidLengthError { .. })));
    }

    #[test]
    fn test_none() {
        // No child matches the constraint.
        let parent = Parent::decode_full(&[2, 4, 0]).unwrap();
        assert_eq!(parent.specialize(), Ok(ParentChild::None));

        // No grandchild matches the constraint.
        let child = Child::decode_full(&[1, 4, 0]).unwrap();
        assert_eq!(child.specialize(), Ok(ChildChild::None));
    }
}

#[pdl_inline(
    r#"
little_endian_packets

packet Parent {
  a: 8,
  _payload_
}

packet Child1 : Parent {
  x: 8,
}

packet Child2 : Parent {
  x: 16,
}
"#
)]
#[cfg(test)]
mod child_determined_by_constant_size {
    #[test]
    fn test_child1() {
        // Success
        let parent = Parent::decode_full(&[1, 42]).unwrap();
        assert_eq!(parent.specialize(), Ok(ParentChild::Child1(Child1 { a: 1, x: 42 })));
    }

    #[test]
    fn test_child2() {
        // Success
        let parent = Parent::decode_full(&[2, 42, 43]).unwrap();
        assert_eq!(parent.specialize(), Ok(ParentChild::Child2(Child2 { a: 2, x: 11050 })));
    }

    #[test]
    fn test_none() {
        // Payload contains too many bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[2, 2, 3, 4]).unwrap();
        assert!(matches!(parent.specialize(), Ok(ParentChild::None)));

        // Payload contains too few bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[2]).unwrap();
        assert!(matches!(parent.specialize(), Ok(ParentChild::None)));
    }
}

#[pdl_inline(
    r#"
little_endian_packets

packet Parent {
  a: 8,
  _payload_
}

packet Child1 : Parent (a = 1) {
  x: 8,
}

packet Child2 : Parent (a = 2) {
  x: 16,
}

packet Child3 : Parent (a = 2) {
  x: 16,
  y: 16,
}
"#
)]
#[cfg(test)]
mod child_determined_by_constraint_and_constant_size {
    #[test]
    fn test_child1() {
        // Success
        let parent = Parent::decode_full(&[1, 42]).unwrap();
        assert_eq!(parent.specialize(), Ok(ParentChild::Child1(Child1 { x: 42 })));

        // Payload contains too many bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[1, 2, 3]).unwrap();
        assert_eq!(parent.a, 1);
        assert!(matches!(parent.specialize(), Ok(ParentChild::None)));

        // Payload contains too few bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[1]).unwrap();
        assert_eq!(parent.a, 1);
        assert!(matches!(parent.specialize(), Ok(ParentChild::None)));
    }

    #[test]
    fn test_child2() {
        // Success
        let parent = Parent::decode_full(&[2, 42, 43]).unwrap();
        assert_eq!(parent.specialize(), Ok(ParentChild::Child2(Child2 { x: 11050 })));
    }

    #[test]
    fn test_child3() {
        // Success
        let parent = Parent::decode_full(&[2, 42, 43, 1, 0]).unwrap();
        assert_eq!(parent.specialize(), Ok(ParentChild::Child3(Child3 { x: 11050, y: 1 })));
    }

    #[test]
    fn test_none() {
        // No child matches the constraint.
        let parent = Parent::decode_full(&[4, 0]).unwrap();
        assert_eq!(parent.specialize(), Ok(ParentChild::None));

        // Payload contains too many bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[2, 2, 3, 4, 5, 6]).unwrap();
        assert!(matches!(parent.specialize(), Ok(ParentChild::None)));

        // Payload contains too few bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[2, 1]).unwrap();
        assert!(matches!(parent.specialize(), Ok(ParentChild::None)));
    }
}

#[pdl_inline(
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
        // Success
        let parent = Parent::decode_full(&[1, 42, 43]).unwrap();
        assert_eq!(parent.specialize(), Ok(ParentChild::Child1(Child1 { x: vec![11050] })));

        // Payload contains too many bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[1, 2, 3, 4]).unwrap();
        assert_eq!(parent.a, 1);
        assert!(matches!(parent.specialize(), Err(DecodeError::InvalidArraySize { .. })));

        // Payload contains too few bytes; specialize fails to produce
        // a valid child.
        let parent = Parent::decode_full(&[1, 2]).unwrap();
        assert_eq!(parent.a, 1);
        assert!(matches!(parent.specialize(), Err(DecodeError::InvalidArraySize { .. })));
    }

    #[test]
    fn test_alias() {
        // Success
        let parent = Parent::decode_full(&[2, 42]).unwrap();
        assert_eq!(parent.specialize(), Ok(ParentChild::Alias(Alias { a: 2, payload: vec![42] })));

        // Payload contains an incorrect number of bytes but the specialize
        // function does not validate grand children.
        let parent = Parent::decode_full(&[2, 42, 43]).unwrap();
        assert_eq!(
            parent.specialize(),
            Ok(ParentChild::Alias(Alias { a: 2, payload: vec![42, 43] }))
        );

        // Success
        let parent = Parent::decode_full(&[3, 42, 43]).unwrap();
        assert_eq!(
            parent.specialize(),
            Ok(ParentChild::Alias(Alias { a: 3, payload: vec![42, 43] }))
        );
    }
}
