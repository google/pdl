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
    pub b: u8,
    pub c: u8,
    pub d: u32,
    pub e: u16,
    pub f: u8,
}
impl Foo {
    pub fn a(&self) -> u8 {
        self.a
    }
    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn c(&self) -> u8 {
        self.c
    }
    pub fn d(&self) -> u32 {
        self.d
    }
    pub fn e(&self) -> u16 {
        self.e
    }
    pub fn f(&self) -> u8 {
        self.f
    }
}
impl Default for Foo {
    fn default() -> Foo {
        Foo {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
        }
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        7
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        if self.a() > 0x7 {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "a",
                value: self.a() as u64,
                maximum_value: 0x7 as u64,
            });
        }
        if self.c() > 0x1f {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "c",
                value: self.c() as u64,
                maximum_value: 0x1f as u64,
            });
        }
        let value = (self.a() as u16) | ((self.b() as u16) << 3)
            | ((self.c() as u16) << 11);
        buf.put_u16(value);
        if self.d() > 0xff_ffff {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "d",
                value: self.d() as u64,
                maximum_value: 0xff_ffff as u64,
            });
        }
        buf.put_uint(self.d() as u64, 3);
        if self.e() > 0xfff {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "e",
                value: self.e() as u64,
                maximum_value: 0xfff as u64,
            });
        }
        if self.f() > 0xf {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "f",
                value: self.f() as u64,
                maximum_value: 0xf as u64,
            });
        }
        let value = self.e() | ((self.f() as u16) << 12);
        buf.put_u16(value);
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 2 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let chunk = buf.get_u16();
        let a = (chunk & 0x7) as u8;
        let b = (chunk >> 3) as u8;
        let c = ((chunk >> 11) & 0x1f) as u8;
        if buf.remaining() < 3 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 3,
                got: buf.remaining(),
            });
        }
        let d = buf.get_uint(3) as u32;
        if buf.remaining() < 2 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let chunk = buf.get_u16();
        let e = (chunk & 0xfff);
        let f = ((chunk >> 12) & 0xf) as u8;
        Ok((Self { a, b, c, d, e, f }, buf))
    }
}
