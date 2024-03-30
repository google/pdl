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
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u64", into = "u64"))]
pub enum Foo {
    A = 0x1,
    B = 0x2,
}
impl TryFrom<u64> for Foo {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Foo::A),
            0x2 => Ok(Foo::B),
            _ => Err(value),
        }
    }
}
impl From<&Foo> for u64 {
    fn from(value: &Foo) -> Self {
        match value {
            Foo::A => 0x1,
            Foo::B => 0x2,
        }
    }
}
impl From<Foo> for u64 {
    fn from(value: Foo) -> Self {
        (&value).into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar {
    pub x: Foo,
}
impl TryFrom<&Bar> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: &Bar) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<&Bar> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: &Bar) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl Bar {
    pub fn x(&self) -> Foo {
        self.x
    }
}
impl Packet for Bar {
    fn encoded_len(&self) -> usize {
        8
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u64(u64::from(self.x()));
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 8 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Bar",
                wanted: 8,
                got: buf.remaining(),
            });
        }
        let x = Foo::try_from(buf.get_u64())
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Bar",
                field: "x",
                value: unknown_val as u64,
                type_: "Foo",
            })?;
        Ok((Self { x }, buf))
    }
}
