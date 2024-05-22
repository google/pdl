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
#[cfg_attr(feature = "serde", serde(try_from = "u32", into = "u32"))]
pub enum Foo {
    A = 0x1,
    B = 0x2,
}
impl TryFrom<u32> for Foo {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Foo::A),
            0x2 => Ok(Foo::B),
            _ => Err(value),
        }
    }
}
impl From<&Foo> for u32 {
    fn from(value: &Foo) -> Self {
        match value {
            Foo::A => 0x1,
            Foo::B => 0x2,
        }
    }
}
impl From<Foo> for u32 {
    fn from(value: Foo) -> Self {
        (&value).into()
    }
}
impl From<Foo> for i32 {
    fn from(value: Foo) -> Self {
        u32::from(value) as Self
    }
}
impl From<Foo> for i64 {
    fn from(value: Foo) -> Self {
        u32::from(value) as Self
    }
}
impl From<Foo> for u64 {
    fn from(value: Foo) -> Self {
        u32::from(value) as Self
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar {
    pub x: Foo,
}
impl Bar {
    pub fn x(&self) -> Foo {
        self.x
    }
}
impl Packet for Bar {
    fn encoded_len(&self) -> usize {
        3
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_uint(u32::from(self.x()) as u64, 3);
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 3 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Bar",
                wanted: 3,
                got: buf.remaining(),
            });
        }
        let x = Foo::try_from(buf.get_uint(3) as u32)
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Bar",
                field: "x",
                value: unknown_val as u64,
                type_: "Foo",
            })?;
        Ok((Self { x }, buf))
    }
}
