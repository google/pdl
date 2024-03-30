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
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum Foo {
    FooBar = 0x1,
    Baz = 0x2,
}
impl TryFrom<u8> for Foo {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Foo::FooBar),
            0x2 => Ok(Foo::Baz),
            _ => Err(value),
        }
    }
}
impl From<&Foo> for u8 {
    fn from(value: &Foo) -> Self {
        match value {
            Foo::FooBar => 0x1,
            Foo::Baz => 0x2,
        }
    }
}
impl From<Foo> for u8 {
    fn from(value: Foo) -> Self {
        (&value).into()
    }
}
impl From<Foo> for i16 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
impl From<Foo> for i32 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
impl From<Foo> for i64 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
impl From<Foo> for u16 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
impl From<Foo> for u32 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
impl From<Foo> for u64 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar {
    pub x: [Foo; 3],
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
    pub fn x(&self) -> &[Foo; 3] {
        &self.x
    }
}
impl Packet for Bar {
    fn encoded_len(&self) -> usize {
        self.x.len() * 1
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        for elem in &self.x {
            buf.put_u8(u8::from(elem));
        }
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
        let mut x = Vec::with_capacity(3);
        for _ in 0..3 {
            x.push(
                Foo::try_from(buf.get_u8())
                    .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                        obj: "Bar",
                        field: "",
                        value: unknown_val as u64,
                        type_: "Foo",
                    })?,
            )
        }
        let x = x.try_into().map_err(|_| DecodeError::InvalidPacketError)?;
        Ok((Self { x }, buf))
    }
}
