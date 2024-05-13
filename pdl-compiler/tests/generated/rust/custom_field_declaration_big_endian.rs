#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use std::result::Result;
use pdl_runtime::{DecodeError, EncodeError, Packet};
/// Private prevents users from creating arbitrary scalar values
/// in situations where the value needs to be validated.
/// Users can freely deref the value, but only the backend
/// may create it.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Private<T>(T);
impl<T> std::ops::Deref for Private<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: std::fmt::Debug> std::fmt::Debug for Private<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        T::fmt(&self.0, f)
    }
}
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
impl Packet for ExactSize {
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.len() < 4 {
            return Err(DecodeError::InvalidLengthError {
                obj: "ExactSize",
                wanted: 4,
                got: buf.len(),
            });
        }
        Ok((buf.get_u32().into(), buf))
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u32(u32::from(self));
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        4
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
impl Packet for TruncatedSize {
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.len() < 3 {
            return Err(DecodeError::InvalidLengthError {
                obj: "TruncatedSize",
                wanted: 3,
                got: buf.len(),
            });
        }
        Ok(((buf.get_uint(3) as u32).try_into().unwrap(), buf))
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_uint(u32::from(self) as u64, 3);
        Ok(())
    }
    fn encoded_len(&self) -> usize {
        3
    }
}
impl TryFrom<u32> for TruncatedSize {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 0xff_ffff { Err(value) } else { Ok(TruncatedSize(value)) }
    }
}
