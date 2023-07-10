#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use thiserror::Error;
type Result<T> = std::result::Result<T, Error>;
/// Private prevents users from creating arbitrary scalar values
/// in situations where the value needs to be validated.
/// Users can freely deref the value, but only the backend
/// may create it.
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
    #[error(
        "array size ({array} bytes) is not a multiple of the element size ({element} bytes)"
    )]
    InvalidArraySize { array: usize, element: usize },
    #[error("Due to size restrictions a struct could not be parsed.")]
    ImpossibleStructError,
    #[error("when parsing field {obj}.{field}, {value} is not a valid {type_} value")]
    InvalidEnumValueError { obj: String, field: String, value: u64, type_: String },
    #[error("expected child {expected}, got {actual}")]
    InvalidChildError { expected: &'static str, actual: String },
}
pub trait Packet {
    fn to_bytes(self) -> Bytes;
    fn to_vec(self) -> Vec<u8>;
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum IncompleteTruncatedClosed {
    A = 0x0,
    B = 0x1,
}
impl TryFrom<u8> for IncompleteTruncatedClosed {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(IncompleteTruncatedClosed::A),
            0x1 => Ok(IncompleteTruncatedClosed::B),
            _ => Err(value),
        }
    }
}
impl From<&IncompleteTruncatedClosed> for u8 {
    fn from(value: &IncompleteTruncatedClosed) -> Self {
        match value {
            IncompleteTruncatedClosed::A => 0x0,
            IncompleteTruncatedClosed::B => 0x1,
        }
    }
}
impl From<IncompleteTruncatedClosed> for u8 {
    fn from(value: IncompleteTruncatedClosed) -> Self {
        (&value).into()
    }
}
impl From<IncompleteTruncatedClosed> for i8 {
    fn from(value: IncompleteTruncatedClosed) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosed> for i16 {
    fn from(value: IncompleteTruncatedClosed) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosed> for i32 {
    fn from(value: IncompleteTruncatedClosed) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosed> for i64 {
    fn from(value: IncompleteTruncatedClosed) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosed> for u16 {
    fn from(value: IncompleteTruncatedClosed) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosed> for u32 {
    fn from(value: IncompleteTruncatedClosed) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosed> for u64 {
    fn from(value: IncompleteTruncatedClosed) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum IncompleteTruncatedOpen {
    A,
    B,
    Unknown(Private<u8>),
}
impl TryFrom<u8> for IncompleteTruncatedOpen {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(IncompleteTruncatedOpen::A),
            0x1 => Ok(IncompleteTruncatedOpen::B),
            0..=0x7 => Ok(IncompleteTruncatedOpen::Unknown(Private(value))),
            _ => Err(value),
        }
    }
}
impl From<&IncompleteTruncatedOpen> for u8 {
    fn from(value: &IncompleteTruncatedOpen) -> Self {
        match value {
            IncompleteTruncatedOpen::A => 0x0,
            IncompleteTruncatedOpen::B => 0x1,
            IncompleteTruncatedOpen::Unknown(Private(value)) => *value,
        }
    }
}
impl From<IncompleteTruncatedOpen> for u8 {
    fn from(value: IncompleteTruncatedOpen) -> Self {
        (&value).into()
    }
}
impl From<IncompleteTruncatedOpen> for i8 {
    fn from(value: IncompleteTruncatedOpen) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpen> for i16 {
    fn from(value: IncompleteTruncatedOpen) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpen> for i32 {
    fn from(value: IncompleteTruncatedOpen) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpen> for i64 {
    fn from(value: IncompleteTruncatedOpen) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpen> for u16 {
    fn from(value: IncompleteTruncatedOpen) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpen> for u32 {
    fn from(value: IncompleteTruncatedOpen) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpen> for u64 {
    fn from(value: IncompleteTruncatedOpen) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum IncompleteTruncatedClosedWithRange {
    A,
    X,
    Y,
    B(Private<u8>),
}
impl TryFrom<u8> for IncompleteTruncatedClosedWithRange {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(IncompleteTruncatedClosedWithRange::A),
            0x1 => Ok(IncompleteTruncatedClosedWithRange::X),
            0x2 => Ok(IncompleteTruncatedClosedWithRange::Y),
            0x1..=0x6 => Ok(IncompleteTruncatedClosedWithRange::B(Private(value))),
            _ => Err(value),
        }
    }
}
impl From<&IncompleteTruncatedClosedWithRange> for u8 {
    fn from(value: &IncompleteTruncatedClosedWithRange) -> Self {
        match value {
            IncompleteTruncatedClosedWithRange::A => 0x0,
            IncompleteTruncatedClosedWithRange::X => 0x1,
            IncompleteTruncatedClosedWithRange::Y => 0x2,
            IncompleteTruncatedClosedWithRange::B(Private(value)) => *value,
        }
    }
}
impl From<IncompleteTruncatedClosedWithRange> for u8 {
    fn from(value: IncompleteTruncatedClosedWithRange) -> Self {
        (&value).into()
    }
}
impl From<IncompleteTruncatedClosedWithRange> for i8 {
    fn from(value: IncompleteTruncatedClosedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosedWithRange> for i16 {
    fn from(value: IncompleteTruncatedClosedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosedWithRange> for i32 {
    fn from(value: IncompleteTruncatedClosedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosedWithRange> for i64 {
    fn from(value: IncompleteTruncatedClosedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosedWithRange> for u16 {
    fn from(value: IncompleteTruncatedClosedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosedWithRange> for u32 {
    fn from(value: IncompleteTruncatedClosedWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedClosedWithRange> for u64 {
    fn from(value: IncompleteTruncatedClosedWithRange) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum IncompleteTruncatedOpenWithRange {
    A,
    X,
    Y,
    B(Private<u8>),
    Unknown(Private<u8>),
}
impl TryFrom<u8> for IncompleteTruncatedOpenWithRange {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(IncompleteTruncatedOpenWithRange::A),
            0x1 => Ok(IncompleteTruncatedOpenWithRange::X),
            0x2 => Ok(IncompleteTruncatedOpenWithRange::Y),
            0x1..=0x6 => Ok(IncompleteTruncatedOpenWithRange::B(Private(value))),
            0..=0x7 => Ok(IncompleteTruncatedOpenWithRange::Unknown(Private(value))),
            _ => Err(value),
        }
    }
}
impl From<&IncompleteTruncatedOpenWithRange> for u8 {
    fn from(value: &IncompleteTruncatedOpenWithRange) -> Self {
        match value {
            IncompleteTruncatedOpenWithRange::A => 0x0,
            IncompleteTruncatedOpenWithRange::X => 0x1,
            IncompleteTruncatedOpenWithRange::Y => 0x2,
            IncompleteTruncatedOpenWithRange::B(Private(value)) => *value,
            IncompleteTruncatedOpenWithRange::Unknown(Private(value)) => *value,
        }
    }
}
impl From<IncompleteTruncatedOpenWithRange> for u8 {
    fn from(value: IncompleteTruncatedOpenWithRange) -> Self {
        (&value).into()
    }
}
impl From<IncompleteTruncatedOpenWithRange> for i8 {
    fn from(value: IncompleteTruncatedOpenWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpenWithRange> for i16 {
    fn from(value: IncompleteTruncatedOpenWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpenWithRange> for i32 {
    fn from(value: IncompleteTruncatedOpenWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpenWithRange> for i64 {
    fn from(value: IncompleteTruncatedOpenWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpenWithRange> for u16 {
    fn from(value: IncompleteTruncatedOpenWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpenWithRange> for u32 {
    fn from(value: IncompleteTruncatedOpenWithRange) -> Self {
        u8::from(value) as Self
    }
}
impl From<IncompleteTruncatedOpenWithRange> for u64 {
    fn from(value: IncompleteTruncatedOpenWithRange) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
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
