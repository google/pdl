#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use pdl_runtime::{Error, Packet, Private};
type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "u32", into = "u32"))]
pub struct ExactSize(u32);
impl From<&ExactSize> for u32 {
    fn from(value: &ExactSize) -> u32 {
        value.0
    }
}
impl From<ExactSize> for u32 {
    fn from(value: ExactSize) -> u32 {
        value.0
    }
}
impl From<u32> for ExactSize {
    fn from(value: u32) -> Self {
        ExactSize(value)
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u32", into = "u32"))]
pub struct TruncatedSize(u32);
impl From<&TruncatedSize> for u32 {
    fn from(value: &TruncatedSize) -> u32 {
        value.0
    }
}
impl From<TruncatedSize> for u32 {
    fn from(value: TruncatedSize) -> u32 {
        value.0
    }
}
impl TryFrom<u32> for TruncatedSize {
    type Error = u32;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        if value > 0xff_ffff { Err(value) } else { Ok(TruncatedSize(value)) }
    }
}
