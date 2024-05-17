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

//! Helper definitions used used by the generated Rust backend.

use bytes::{BufMut, Bytes, BytesMut};

/// Type of parsing errors.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DecodeError {
    #[error("packet parsing failed")]
    InvalidPacketError,
    #[error("{field} was {value:x}, which is not known")]
    ConstraintOutOfBounds { field: &'static str, value: u64 },
    #[error("Got {actual:x}, expected {expected:x}")]
    InvalidFixedValue { expected: u64, actual: u64 },
    #[error("when parsing {obj} needed length of {wanted} but got {got}")]
    InvalidLengthError { obj: &'static str, wanted: usize, got: usize },
    #[error("array size ({array} bytes) is not a multiple of the element size ({element} bytes)")]
    InvalidArraySize { array: usize, element: usize },
    #[error("Due to size restrictions a struct could not be parsed.")]
    ImpossibleStructError,
    #[error("when parsing field {obj}.{field}, {value} is not a valid {type_} value")]
    InvalidEnumValueError {
        obj: &'static str,
        field: &'static str,
        value: u64,
        type_: &'static str,
    },
    #[error("invalid field {packet}::{field} value, {expected} != {actual}")]
    InvalidFieldValue {
        packet: &'static str,
        field: &'static str,
        expected: &'static str,
        actual: String,
    },
    #[error("expected child {expected}, got {actual}")]
    InvalidChildError { expected: &'static str, actual: String },
    #[error("packet has trailing bytes")]
    TrailingBytes,
    #[error("packet has trailing bytes inside {obj}.{field} array")]
    TrailingBytesInArray { obj: &'static str, field: &'static str },
}

/// Type of serialization errors.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum EncodeError {
    #[error("the size of {packet}::{field} ({size}) is outside the range of valid values 0..{maximum_size}")]
    SizeOverflow { packet: &'static str, field: &'static str, size: usize, maximum_size: usize },
    #[error(
        "the count of {packet}::{field} ({count}) is outside the range of valid values 0..{maximum_count}"
    )]
    CountOverflow { packet: &'static str, field: &'static str, count: usize, maximum_count: usize },
    #[error(
        "the value of {packet}::{field} ({value}) is outside the range of valid values 0..{maximum_value}"
    )]
    InvalidScalarValue { packet: &'static str, field: &'static str, value: u64, maximum_value: u64 },
    #[error(
        "{packet}.{field}[{element_index}] size is {size}, but {expected_size} was expected (size of {packet}.{field}[0])"
    )]
    InvalidArrayElementSize {
        packet: &'static str,
        field: &'static str,
        size: usize,
        expected_size: usize,
        element_index: usize,
    },
}

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
        if remaining.is_empty() {
            Ok(packet)
        } else {
            Err(DecodeError::TrailingBytes)
        }
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
