// @generated rust packets from test

use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::cell::Cell;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::sync::Arc;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[doc = r" Private prevents users from creating arbitrary scalar values"]
#[doc = r" in situations where the value needs to be validated."]
#[doc = r" Users can freely deref the value, but only the backend"]
#[doc = r" may create it."]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Private<T>(T);
impl<T> std::ops::Deref for Private<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Packet parsing failed")]
    InvalidPacketError,
    #[error("{field} was {value:x}, which is not known")]
    ConstraintOutOfBounds { field: String, value: u64 },
    #[error("Got {actual:x}, expected {expected:x}")]
    InvalidFixedValue { expected: u64, actual: u64 },
    #[error("when parsing {obj} needed length of {wanted} but got {got}")]
    InvalidLengthError { obj: String, wanted: usize, got: usize },
    #[error("array size ({array} bytes) is not a multiple of the element size ({element} bytes)")]
    InvalidArraySize { array: usize, element: usize },
    #[error("Due to size restrictions a struct could not be parsed.")]
    ImpossibleStructError,
    #[error("when parsing field {obj}.{field}, {value} is not a valid {type_} value")]
    InvalidEnumValueError { obj: String, field: String, value: u64, type_: String },
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct TryFromError(&'static str);

pub trait Packet {
    fn to_bytes(self) -> Bytes;
    fn to_vec(self) -> Vec<u8>;
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum IncompleteTruncated {
    A = 0x0,
    B = 0x1,
}
impl TryFrom<u8> for IncompleteTruncated {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(IncompleteTruncated::A),
            0x1 => Ok(IncompleteTruncated::B),
            _ => Err(value),
        }
    }
}
impl From<&IncompleteTruncated> for u8 {
    fn from(value: &IncompleteTruncated) -> Self {
        match value {
            IncompleteTruncated::A => 0x0,
            IncompleteTruncated::B => 0x1,
        }
    }
}
impl From<IncompleteTruncated> for u8 {
    fn from(value: IncompleteTruncated) -> Self {
        (&value).into()
    }
}
impl From<IncompleteTruncated> for i8 {
    fn from(value: IncompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncated> for i16 {
    fn from(value: IncompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncated> for i32 {
    fn from(value: IncompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncated> for i64 {
    fn from(value: IncompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncated> for u16 {
    fn from(value: IncompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncated> for u32 {
    fn from(value: IncompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncated> for u64 {
    fn from(value: IncompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum IncompleteTruncatedWithRange {
    A,
    X,
    Y,
    B(Private<u8>),
}
impl TryFrom<u8> for IncompleteTruncatedWithRange {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(IncompleteTruncatedWithRange::A),
            0x1 => Ok(IncompleteTruncatedWithRange::X),
            0x2 => Ok(IncompleteTruncatedWithRange::Y),
            0x1..=0x6 => Ok(IncompleteTruncatedWithRange::B(Private(value))),
            _ => Err(value),
        }
    }
}
impl From<&IncompleteTruncatedWithRange> for u8 {
    fn from(value: &IncompleteTruncatedWithRange) -> Self {
        match value {
            IncompleteTruncatedWithRange::A => 0x0,
            IncompleteTruncatedWithRange::X => 0x1,
            IncompleteTruncatedWithRange::Y => 0x2,
            IncompleteTruncatedWithRange::B(Private(value)) => *value,
        }
    }
}
impl From<IncompleteTruncatedWithRange> for u8 {
    fn from(value: IncompleteTruncatedWithRange) -> Self {
        (&value).into()
    }
}
impl From<IncompleteTruncatedWithRange> for i8 {
    fn from(value: IncompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedWithRange> for i16 {
    fn from(value: IncompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedWithRange> for i32 {
    fn from(value: IncompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedWithRange> for i64 {
    fn from(value: IncompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedWithRange> for u16 {
    fn from(value: IncompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedWithRange> for u32 {
    fn from(value: IncompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedWithRange> for u64 {
    fn from(value: IncompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum CompleteTruncated {
    A = 0x0,
    B = 0x1,
    C = 0x2,
    D = 0x3,
    E = 0x4,
    F = 0x5,
    G = 0x6,
    H = 0x7,
}
impl TryFrom<u8> for CompleteTruncated {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(CompleteTruncated::A),
            0x1 => Ok(CompleteTruncated::B),
            0x2 => Ok(CompleteTruncated::C),
            0x3 => Ok(CompleteTruncated::D),
            0x4 => Ok(CompleteTruncated::E),
            0x5 => Ok(CompleteTruncated::F),
            0x6 => Ok(CompleteTruncated::G),
            0x7 => Ok(CompleteTruncated::H),
            _ => Err(value),
        }
    }
}
impl From<&CompleteTruncated> for u8 {
    fn from(value: &CompleteTruncated) -> Self {
        match value {
            CompleteTruncated::A => 0x0,
            CompleteTruncated::B => 0x1,
            CompleteTruncated::C => 0x2,
            CompleteTruncated::D => 0x3,
            CompleteTruncated::E => 0x4,
            CompleteTruncated::F => 0x5,
            CompleteTruncated::G => 0x6,
            CompleteTruncated::H => 0x7,
        }
    }
}
impl From<CompleteTruncated> for u8 {
    fn from(value: CompleteTruncated) -> Self {
        (&value).into()
    }
}
impl From<CompleteTruncated> for i8 {
    fn from(value: CompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncated> for i16 {
    fn from(value: CompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncated> for i32 {
    fn from(value: CompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncated> for i64 {
    fn from(value: CompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncated> for u16 {
    fn from(value: CompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncated> for u32 {
    fn from(value: CompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncated> for u64 {
    fn from(value: CompleteTruncated) -> Self {
        u8::from(value) as Self
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum CompleteTruncatedWithRange {
    A,
    X,
    Y,
    B(Private<u8>),
}
impl TryFrom<u8> for CompleteTruncatedWithRange {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(CompleteTruncatedWithRange::A),
            0x1 => Ok(CompleteTruncatedWithRange::X),
            0x2 => Ok(CompleteTruncatedWithRange::Y),
            0x1..=0x7 => Ok(CompleteTruncatedWithRange::B(Private(value))),
            _ => Err(value),
        }
    }
}
impl From<&CompleteTruncatedWithRange> for u8 {
    fn from(value: &CompleteTruncatedWithRange) -> Self {
        match value {
            CompleteTruncatedWithRange::A => 0x0,
            CompleteTruncatedWithRange::X => 0x1,
            CompleteTruncatedWithRange::Y => 0x2,
            CompleteTruncatedWithRange::B(Private(value)) => *value,
        }
    }
}
impl From<CompleteTruncatedWithRange> for u8 {
    fn from(value: CompleteTruncatedWithRange) -> Self {
        (&value).into()
    }
}
impl From<CompleteTruncatedWithRange> for i8 {
    fn from(value: CompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncatedWithRange> for i16 {
    fn from(value: CompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncatedWithRange> for i32 {
    fn from(value: CompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncatedWithRange> for i64 {
    fn from(value: CompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncatedWithRange> for u16 {
    fn from(value: CompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncatedWithRange> for u32 {
    fn from(value: CompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteTruncatedWithRange> for u64 {
    fn from(value: CompleteTruncatedWithRange) -> Self {
        u8::from(value) as Self
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum CompleteWithRange {
    A,
    B,
    C(Private<u8>),
}
impl TryFrom<u8> for CompleteWithRange {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(CompleteWithRange::A),
            0x1 => Ok(CompleteWithRange::B),
            0x2..=0xff => Ok(CompleteWithRange::C(Private(value))),
        }
    }
}
impl From<&CompleteWithRange> for u8 {
    fn from(value: &CompleteWithRange) -> Self {
        match value {
            CompleteWithRange::A => 0x0,
            CompleteWithRange::B => 0x1,
            CompleteWithRange::C(Private(value)) => *value,
        }
    }
}
impl From<CompleteWithRange> for u8 {
    fn from(value: CompleteWithRange) -> Self {
        (&value).into()
    }
}
impl From<CompleteWithRange> for i16 {
    fn from(value: CompleteWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteWithRange> for i32 {
    fn from(value: CompleteWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteWithRange> for i64 {
    fn from(value: CompleteWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteWithRange> for u16 {
    fn from(value: CompleteWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteWithRange> for u32 {
    fn from(value: CompleteWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<CompleteWithRange> for u64 {
    fn from(value: CompleteWithRange) -> Self {
        u8::from(value) as Self
    }
}
