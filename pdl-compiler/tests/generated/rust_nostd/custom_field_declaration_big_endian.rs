#![rustfmt::skip]
/// @generated rust_nostd packets from test.
extern crate alloc;
use alloc::vec::Vec;
use bytes::{Buf, BufMut};
use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::result::Result;
use pdl_runtime::nostd::Packet;
use pdl_runtime::{Array, DecodeError, EncodeError};
/// Private prevents users from creating arbitrary scalar values
/// in situations where the value needs to be validated.
/// Users can freely deref the value, but only the backend
/// may create it.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Private<T>(T);
impl<T> core::ops::Deref for Private<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: core::fmt::Debug> core::fmt::Debug for Private<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        T::fmt(&self.0, f)
    }
}
#[derive(Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
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
impl<'a> Packet<'a> for ExactSize {
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.len() < 4 {
            return Err(DecodeError::LengthError {
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
#[derive(Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
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
impl<'a> Packet<'a> for TruncatedSize {
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.len() < 3 {
            return Err(DecodeError::LengthError {
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
