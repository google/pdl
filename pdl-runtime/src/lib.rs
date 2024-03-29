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
    InvalidEnumValueError { obj: &'static str, field: &'static str, value: u64, type_: &'static str },
    #[error("expected child {expected}, got {actual}")]
    InvalidChildError { expected: &'static str, actual: String },
    #[error("packet has trailing bytes")]
    TrailingBytes,
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
}

/// Trait implemented for all toplevel packet declarations.
pub trait Packet: Sized {
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
