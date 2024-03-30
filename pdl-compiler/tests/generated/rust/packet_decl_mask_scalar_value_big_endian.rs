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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    pub a: u8,
    pub b: u32,
    pub c: u8,
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
    pub fn a(&self) -> u8 {
        self.a
    }
    pub fn b(&self) -> u32 {
        self.b
    }
    pub fn c(&self) -> u8 {
        self.c
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        4
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        if self.a() > 0x3 {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "a",
                value: self.a() as u64,
                maximum_value: 0x3 as u64,
            });
        }
        if self.b() > 0xff_ffff {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "b",
                value: self.b() as u64,
                maximum_value: 0xff_ffff as u64,
            });
        }
        if self.c() > 0x3f {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "c",
                value: self.c() as u64,
                maximum_value: 0x3f as u64,
            });
        }
        let value = (self.a() as u32) | (self.b() << 2) | ((self.c() as u32) << 26);
        buf.put_u32(value);
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 4 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 4,
                got: buf.remaining(),
            });
        }
        let chunk = buf.get_u32();
        let a = (chunk & 0x3) as u8;
        let b = ((chunk >> 2) & 0xff_ffff);
        let c = ((chunk >> 26) & 0x3f) as u8;
        Ok((Self { a, b, c }, buf))
    }
}
