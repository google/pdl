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

//! Helper definitions used by the generated Rust backend.

#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use bytes::{BufMut, Bytes, BytesMut};
use core::fmt;

/// Type of parsing errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// This error case is generated in places where an unwrap call would be used.
    /// All occurences in the generated code actually correspond to unreachable
    /// statements, but this error is generated instead to keep the code unwrap free.
    UnwrapError,
    FixedValueError {
        expected: u64,
        actual: u64,
    },
    LengthError {
        obj: &'static str,
        wanted: usize,
        got: usize,
    },
    ArraySizeError {
        array: usize,
        element: usize,
    },
    EnumValueError {
        obj: &'static str,
        field: &'static str,
        value: u64,
        type_: &'static str,
    },
    ConstraintValueError {
        packet: &'static str,
        field: &'static str,
        expected: &'static str,
        actual: String,
    },
    TrailingBytesError,
    TrailingBytesInArray {
        obj: &'static str,
        field: &'static str,
    },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnwrapError => write!(f, "unwrap error"),
            Self::FixedValueError { expected, actual } => {
                write!(f, "Got {actual:x}, expected {expected:x}")
            }
            Self::LengthError { obj, wanted, got } => {
                write!(f, "when parsing {obj} needed length of {wanted} but got {got}")
            }
            Self::ArraySizeError { array, element } => {
                write!(
                    f,
                    "array size ({array} bytes) is not a multiple of the element size ({element} bytes)"
                )
            }
            Self::EnumValueError { obj, field, value, type_ } => {
                write!(f, "when parsing field {obj}.{field}, {value} is not a valid {type_} value")
            }
            Self::ConstraintValueError { packet, field, expected, actual } => {
                write!(f, "invalid field {packet}::{field} value, {expected} != {actual}")
            }
            Self::TrailingBytesError => write!(f, "packet has trailing bytes"),
            Self::TrailingBytesInArray { obj, field } => {
                write!(f, "packet has trailing bytes inside {obj}.{field} array")
            }
        }
    }
}

impl core::error::Error for DecodeError {}

/// Type of serialization errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodeError {
    SizeOverflow {
        packet: &'static str,
        field: &'static str,
        size: usize,
        maximum_size: usize,
    },
    CountOverflow {
        packet: &'static str,
        field: &'static str,
        count: usize,
        maximum_count: usize,
    },
    InvalidScalarValue {
        packet: &'static str,
        field: &'static str,
        value: u64,
        maximum_value: u64,
    },
    InvalidArrayElementSize {
        packet: &'static str,
        field: &'static str,
        size: usize,
        expected_size: usize,
        element_index: usize,
    },
    InconsistentConditionValue {
        packet: &'static str,
        field: &'static str,
    },
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SizeOverflow { packet, field, size, maximum_size } => write!(
                f,
                "the size of {packet}::{field} ({size}) is outside the range of valid values 0..{maximum_size}"
            ),
            Self::CountOverflow { packet, field, count, maximum_count } => write!(
                f,
                "the count of {packet}::{field} ({count}) is outside the range of valid values 0..{maximum_count}"
            ),
            Self::InvalidScalarValue { packet, field, value, maximum_value } => write!(
                f,
                "the value of {packet}::{field} ({value}) is outside the range of valid values 0..{maximum_value}"
            ),
            Self::InvalidArrayElementSize { packet, field, size, expected_size, element_index } => {
                write!(
                    f,
                    "{packet}.{field}[{element_index}] size is {size}, but {expected_size} was expected (size of {packet}.{field}[0])"
                )
            }
            Self::InconsistentConditionValue { packet, field } => {
                write!(f, "{packet}.{field} value cannot be uniquely determined")
            }
        }
    }
}

impl core::error::Error for EncodeError {}

/// Trait implemented for all toplevel packet declarations.
pub trait Packet: Sized {
    /// Try parsing an instance of Self from the input slice.
    /// On success, returns the parsed object and the remaining unparsed slice.
    /// On failure, returns an error with the reason for the parsing failure.
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError>;

    /// Try parsing an instance of Packet updating the slice in place
    /// to the remainder of the data. The input buffer is not updated if
    /// parsing fails.
    fn decode_mut(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        let (packet, remaining) = Self::decode(buf)?;
        *buf = remaining;
        Ok(packet)
    }

    /// Try parsing an instance of Packet from the input slice.
    /// Returns an error if unparsed bytes remain at the end of the input slice.
    fn decode_full(buf: &[u8]) -> Result<Self, DecodeError> {
        let (packet, remaining) = Self::decode(buf)?;
        if remaining.is_empty() { Ok(packet) } else { Err(DecodeError::TrailingBytesError) }
    }

    /// Return the length of the encoded packet.
    fn encoded_len(&self) -> usize;

    /// Write the packet to an output buffer.
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError>;

    /// Encode the packet to a byte vector.
    fn encode_to_vec(&self) -> Result<Vec<u8>, EncodeError> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        Ok(buf)
    }

    /// Encode the packet to a Bytes object.
    fn encode_to_bytes(&self) -> Result<Bytes, EncodeError> {
        let mut buf = BytesMut::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        Ok(buf.freeze())
    }
}

/// Definitions for the `rust_nostd` backend.
pub mod nostd {
    use super::*;

    /// Trait implemented for all toplevel packet declarations in `rust_nostd`.
    pub trait Packet<'a>: Sized + Clone {
        /// Try parsing an instance of Self from the input slice.
        /// On success, returns the parsed object and the remaining unparsed slice.
        /// On failure, returns an error with the reason for the parsing failure.
        fn decode(buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError>;

        /// Try parsing an instance of Packet updating the slice in place
        /// to the remainder of the data. The input buffer is not updated if
        /// parsing fails.
        fn decode_mut(buf: &mut &'a [u8]) -> Result<Self, DecodeError> {
            let (packet, remaining) = Self::decode(buf)?;
            *buf = remaining;
            Ok(packet)
        }

        /// Try parsing an instance of Packet from the input slice.
        /// Returns an error if unparsed bytes remain at the end of the input slice.
        fn decode_full(buf: &'a [u8]) -> Result<Self, DecodeError> {
            let (packet, remaining) = Self::decode(buf)?;
            if remaining.is_empty() { Ok(packet) } else { Err(DecodeError::TrailingBytesError) }
        }

        /// Return the length of the encoded packet.
        fn encoded_len(&self) -> usize;

        /// Write the packet to an output buffer.
        fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError>;

        /// Encode the packet to a byte vector.
        fn encode_to_vec(&self) -> Result<Vec<u8>, EncodeError> {
            let mut buf = Vec::with_capacity(self.encoded_len());
            self.encode(&mut buf)?;
            Ok(buf)
        }

        /// Encode the packet to a Bytes object.
        fn encode_to_bytes(&self) -> Result<Bytes, EncodeError> {
            let mut buf = BytesMut::with_capacity(self.encoded_len());
            self.encode(&mut buf)?;
            Ok(buf.freeze())
        }
    }
}

/// Representation of arrays in PDL packets and structures.
#[derive(Clone)]
pub enum Array<'a, T> {
    /// Lazy array created when parsing a packet with a reference to the
    /// original slice.
    Lazy {
        len: usize,
        slice: &'a [u8],
        element_size: Option<usize>,
        parse_elem: fn(&mut &'a [u8]) -> T,
    },
    /// Borrowed array representation used when building a packet
    /// with the array slice.
    Borrowed(&'a [T]),
    /// Owned array representation used when building a packet
    /// with the array value.
    Owned(Box<[T]>),
}

/// Iterator for `Array` elements.
pub enum ArrayIter<'a, 'b, T> {
    Lazy {
        remaining: usize,
        slice: &'a [u8],
        element_size: Option<usize>,
        parse_elem: fn(&mut &'a [u8]) -> T,
    },
    Borrowed(core::slice::Iter<'b, T>),
}

/// Type of items returned iterating over `Array` elements.
#[derive(Clone)]
pub enum ArrayItem<'a, T> {
    Owned(T),
    Borrowed(&'a T),
}

impl<T: Clone> ArrayItem<'_, T> {
    /// Convert the item into an owned value.
    pub fn into_owned(self) -> T {
        match self {
            ArrayItem::Owned(item) => item,
            ArrayItem::Borrowed(item) => item.clone(),
        }
    }
}

impl<'a, T> Array<'a, T> {
    /// Create a lazy array from the raw data bytes and an element parser function.
    pub fn from_raw_parts(
        len: usize,
        slice: &'a [u8],
        element_size: Option<usize>,
        parse_elem: fn(&mut &'a [u8]) -> T,
    ) -> Array<'a, T> {
        Array::Lazy { len, slice, element_size, parse_elem }
    }

    /// Iterate over the elements of the array.
    pub fn iter(&self) -> ArrayIter<'a, '_, T> {
        match self {
            Array::Lazy { len, slice, element_size, parse_elem } => ArrayIter::Lazy {
                remaining: *len,
                slice,
                element_size: *element_size,
                parse_elem: *parse_elem,
            },
            Array::Borrowed(slice) => ArrayIter::Borrowed(slice.iter()),
            Array::Owned(vec) => ArrayIter::Borrowed(vec.iter()),
        }
    }

    /// Return the number of elements in the array.
    /// This is *not* the length in bytes of the serialized array.
    pub fn len(&self) -> usize {
        match self {
            Array::Lazy { len, .. } => *len,
            Array::Borrowed(slice) => slice.len(),
            Array::Owned(vec) => vec.len(),
        }
    }

    /// Return true if the array contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> Array<'a, u8> {
    /// Create a lazy byte array from the raw data bytes.
    pub fn lazy(len: usize, slice: &'a [u8]) -> Array<'a, u8> {
        Array::Borrowed(&slice[..len])
    }

    /// Return the underlying slice with lifetime `'a` if borrowed/lazy,
    /// or with the lifetime of `self` if owned.
    pub fn as_slice(&self) -> Result<&'a [u8], &[u8]> {
        match self {
            Array::Lazy { slice, .. } | Array::Borrowed(slice) => Ok(slice),
            Array::Owned(vec) => Err(vec),
        }
    }
}

impl<T: Clone> Array<'_, T> {
    /// Convert the array into an owned `Array<'b, T>` for any target lifetime `'b`.
    pub fn into_owned<'b>(self) -> Array<'b, T> {
        match self {
            Array::Owned(vec) => Array::Owned(vec),
            Array::Borrowed(slice) => Array::Owned(slice.to_vec().into_boxed_slice()),
            Array::Lazy { .. } => {
                let vec: Vec<T> = self.iter().map(ArrayItem::into_owned).collect();
                Array::Owned(vec.into_boxed_slice())
            }
        }
    }

    /// Convert the array into an owned `Array<'b, U>` by mapping each element.
    pub fn map_owned<'b, U>(self, f: fn(T) -> U) -> Array<'b, U> {
        let vec: Vec<U> = self.iter().map(|item| f(item.into_owned())).collect();
        Array::Owned(vec.into_boxed_slice())
    }
}

impl<T> Default for Array<'_, T> {
    fn default() -> Self {
        Array::Borrowed(&[])
    }
}

impl<'a, 'b, T> Iterator for ArrayIter<'a, 'b, T> {
    type Item = ArrayItem<'b, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ArrayIter::Lazy { remaining, slice, element_size, parse_elem } => {
                if *remaining == 0 {
                    return None;
                }
                *remaining -= 1;
                if let Some(step) = *element_size {
                    let (mut chunk, rest) = slice.split_at(step);
                    *slice = rest;
                    Some(ArrayItem::Owned(parse_elem(&mut chunk)))
                } else {
                    Some(ArrayItem::Owned(parse_elem(slice)))
                }
            }
            ArrayIter::Borrowed(it) => it.next().map(ArrayItem::Borrowed),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match self {
            ArrayIter::Lazy { remaining, .. } => *remaining,
            ArrayIter::Borrowed(it) => it.len(),
        };
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for ArrayIter<'_, '_, T> {}

impl<'a, 'b, T> IntoIterator for &'b Array<'a, T> {
    type Item = ArrayItem<'b, T>;
    type IntoIter = ArrayIter<'a, 'b, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> From<&'a [T]> for Array<'a, T> {
    fn from(slice: &'a [T]) -> Array<'a, T> {
        Array::Borrowed(slice)
    }
}

impl<'a, T, const N: usize> From<&'a [T; N]> for Array<'a, T> {
    fn from(arr: &'a [T; N]) -> Array<'a, T> {
        Array::Borrowed(arr.as_slice())
    }
}

impl<T> From<Box<[T]>> for Array<'_, T> {
    fn from(vec: Box<[T]>) -> Self {
        Array::Owned(vec)
    }
}

impl<T> From<Vec<T>> for Array<'_, T> {
    fn from(vec: Vec<T>) -> Self {
        Array::Owned(vec.into_boxed_slice())
    }
}

impl<'a, T: Clone> From<alloc::borrow::Cow<'a, [T]>> for Array<'a, T> {
    fn from(cow: alloc::borrow::Cow<'a, [T]>) -> Self {
        match cow {
            alloc::borrow::Cow::Borrowed(slice) => Array::Borrowed(slice),
            alloc::borrow::Cow::Owned(vec) => Array::Owned(vec.into_boxed_slice()),
        }
    }
}

impl core::ops::Deref for Array<'_, u8> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match self {
            Array::Lazy { slice, .. } | Array::Borrowed(slice) => slice,
            Array::Owned(vec) => vec,
        }
    }
}

impl<T> core::ops::Deref for ArrayItem<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            ArrayItem::Borrowed(item) => item,
            ArrayItem::Owned(item) => item,
        }
    }
}

impl<T: PartialEq> PartialEq<ArrayItem<'_, T>> for ArrayItem<'_, T> {
    fn eq(&self, other: &ArrayItem<'_, T>) -> bool {
        **self == **other
    }
}

impl<T: Eq> Eq for ArrayItem<'_, T> {}

impl<T: PartialEq> PartialEq<Array<'_, T>> for Array<'_, T> {
    fn eq(&self, other: &Array<'_, T>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Eq> Eq for Array<'_, T> {}

impl<T: fmt::Debug> fmt::Debug for ArrayItem<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: fmt::Debug> fmt::Debug for Array<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for ArrayItem<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (**self).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Array<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Array<'_, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec = Vec::<T>::deserialize(deserializer)?;
        Ok(Array::Owned(vec.into_boxed_slice()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::Cow;
    use alloc::format;
    use alloc::vec;
    use bytes::Buf;

    #[test]
    fn test_decode_error_display() {
        assert_eq!(format!("{}", DecodeError::UnwrapError), "unwrap error");
        assert_eq!(
            format!("{}", DecodeError::FixedValueError { expected: 0x10, actual: 0x20 }),
            "Got 20, expected 10"
        );
        assert_eq!(
            format!("{}", DecodeError::LengthError { obj: "Foo", wanted: 4, got: 2 }),
            "when parsing Foo needed length of 4 but got 2"
        );
        assert_eq!(
            format!("{}", DecodeError::ArraySizeError { array: 5, element: 2 }),
            "array size (5 bytes) is not a multiple of the element size (2 bytes)"
        );
        assert_eq!(
            format!(
                "{}",
                DecodeError::EnumValueError { obj: "Foo", field: "bar", value: 7, type_: "Baz" }
            ),
            "when parsing field Foo.bar, 7 is not a valid Baz value"
        );
        assert_eq!(
            format!(
                "{}",
                DecodeError::ConstraintValueError {
                    packet: "Child",
                    field: "tag",
                    expected: "1",
                    actual: String::from("2"),
                }
            ),
            "invalid field Child::tag value, 1 != 2"
        );
        assert_eq!(format!("{}", DecodeError::TrailingBytesError), "packet has trailing bytes");
        assert_eq!(
            format!("{}", DecodeError::TrailingBytesInArray { obj: "Foo", field: "arr" }),
            "packet has trailing bytes inside Foo.arr array"
        );
    }

    #[test]
    fn test_encode_error_display() {
        assert_eq!(
            format!(
                "{}",
                EncodeError::SizeOverflow {
                    packet: "Foo",
                    field: "bar",
                    size: 300,
                    maximum_size: 255
                }
            ),
            "the size of Foo::bar (300) is outside the range of valid values 0..255"
        );
        assert_eq!(
            format!(
                "{}",
                EncodeError::CountOverflow {
                    packet: "Foo",
                    field: "bar",
                    count: 20,
                    maximum_count: 15
                }
            ),
            "the count of Foo::bar (20) is outside the range of valid values 0..15"
        );
        assert_eq!(
            format!(
                "{}",
                EncodeError::InvalidScalarValue {
                    packet: "Foo",
                    field: "bar",
                    value: 9,
                    maximum_value: 7
                }
            ),
            "the value of Foo::bar (9) is outside the range of valid values 0..7"
        );
        assert_eq!(
            format!(
                "{}",
                EncodeError::InvalidArrayElementSize {
                    packet: "Foo",
                    field: "bar",
                    size: 3,
                    expected_size: 4,
                    element_index: 1,
                }
            ),
            "Foo.bar[1] size is 3, but 4 was expected (size of Foo.bar[0])"
        );
        assert_eq!(
            format!("{}", EncodeError::InconsistentConditionValue { packet: "Foo", field: "cond" }),
            "Foo.cond value cannot be uniquely determined"
        );
    }

    #[test]
    fn test_array_representations_and_equality() {
        let raw = [0x01u8, 0x02, 0x03, 0x04];
        let lazy: Array<'_, u16> = Array::from_raw_parts(2, &raw, None, |span| span.get_u16_le());
        let borrowed_slice = [0x0201u16, 0x0403];
        let borrowed: Array<'_, u16> = Array::from(&borrowed_slice);
        let owned: Array<'_, u16> = Array::from(vec![0x0201u16, 0x0403]);
        let from_box: Array<'_, u16> = Array::from(vec![0x0201u16, 0x0403].into_boxed_slice());
        let from_cow_borrowed: Array<'_, u16> = Array::from(Cow::Borrowed(&borrowed_slice[..]));
        let from_cow_owned: Array<'_, u16> = Array::from(Cow::Owned(vec![0x0201u16, 0x0403]));

        assert_eq!(lazy.len(), 2);
        assert!(!lazy.is_empty());
        assert_eq!(borrowed.len(), 2);
        assert_eq!(owned.len(), 2);

        assert_eq!(lazy, borrowed);
        assert_eq!(lazy, owned);
        assert_eq!(borrowed, owned);
        assert_eq!(owned, from_box);
        assert_eq!(borrowed, from_cow_borrowed);
        assert_eq!(owned, from_cow_owned);

        assert_eq!(format!("{lazy:?}"), "[513, 1027]");
        assert_eq!(format!("{borrowed:?}"), "[513, 1027]");
        assert_eq!(format!("{owned:?}"), "[513, 1027]");

        let mut iter = (&lazy).into_iter();
        assert_eq!(iter.len(), 2);
        let item0 = iter.next().unwrap();
        assert_eq!(format!("{item0:?}"), "513");
        assert_eq!(*item0, 0x0201);
        assert_eq!(item0.into_owned(), 0x0201);
        assert_eq!(iter.len(), 1);
        let item1 = iter.next().unwrap();
        assert_eq!(item1, ArrayItem::Borrowed(&0x0403));
        assert!(iter.next().is_none());

        let lazy_into_owned: Array<'static, u16> = lazy.clone().into_owned();
        assert_eq!(lazy_into_owned, borrowed);
        let borrowed_into_owned: Array<'static, u16> = borrowed.clone().into_owned();
        assert_eq!(borrowed_into_owned, owned);
        let owned_into_owned: Array<'static, u16> = owned.clone().into_owned();
        assert_eq!(owned_into_owned, owned);

        let mapped: Array<'static, u32> = lazy.map_owned(|x| x as u32 + 1);
        assert_eq!(mapped, Array::from(&[0x0202u32, 0x0404]));
    }

    #[test]
    fn test_array_chunked_and_u8() {
        let raw = [10u8, 0, 20, 0];
        let chunked: Array<'_, u8> = Array::from_raw_parts(2, &raw, Some(2), |span| span.get_u8());
        assert_eq!(chunked, Array::from(&[10u8, 20]));
        assert_eq!(&*chunked, &raw[..]);
        assert_eq!(chunked.as_slice(), Ok(&raw[..]));

        let lazy_u8 = Array::lazy(2, &raw);
        assert_eq!(&*lazy_u8, &[10u8, 0]);
        assert_eq!(lazy_u8.as_slice(), Ok(&raw[..2]));

        let owned_u8: Array<'_, u8> = Array::from(vec![10u8, 20]);
        assert_eq!(&*owned_u8, &[10u8, 20]);
        assert_eq!(owned_u8.as_slice(), Err(&[10u8, 20][..]));

        let empty: Array<'_, u8> = Array::default();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
    }
}
