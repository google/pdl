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
#[cfg_attr(feature = "serde", serde(try_from = "u32", into = "u32"))]
pub struct Bar1(u32);
impl From<&Bar1> for u32 {
    fn from(value: &Bar1) -> u32 {
        value.0
    }
}
impl From<Bar1> for u32 {
    fn from(value: Bar1) -> u32 {
        value.0
    }
}
impl Packet for Bar1 {
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.len() < 3 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Bar1",
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
impl TryFrom<u32> for Bar1 {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 0xff_ffff { Err(value) } else { Ok(Bar1(value)) }
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "u32", into = "u32"))]
pub struct Bar2(u32);
impl From<&Bar2> for u32 {
    fn from(value: &Bar2) -> u32 {
        value.0
    }
}
impl From<Bar2> for u32 {
    fn from(value: Bar2) -> u32 {
        value.0
    }
}
impl Packet for Bar2 {
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.len() < 4 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Bar2",
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
impl From<u32> for Bar2 {
    fn from(value: u32) -> Self {
        Bar2(value)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    pub a: Bar1,
    pub b: Bar2,
}
impl TryFrom<&Foo> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: &Foo) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<&Foo> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: &Foo) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl Foo {
    pub fn a(&self) -> Bar1 {
        self.a
    }
    pub fn b(&self) -> Bar2 {
        self.b
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        56
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_uint(u32::from(self.a) as u64, 3);
        buf.put_u32(u32::from(self.b));
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let a = (buf.get_uint(3) as u32).try_into().unwrap();
        let b = buf.get_u32().into();
        Ok((Self { a, b }, buf))
    }
}
