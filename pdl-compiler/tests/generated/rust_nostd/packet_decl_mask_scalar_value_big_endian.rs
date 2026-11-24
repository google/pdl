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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    pub a: u8,
    pub b: u32,
    pub c: u8,
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
impl Default for Foo {
    fn default() -> Foo {
        Foo { a: 0, b: 0, c: 0 }
    }
}
impl<'a> Packet<'a> for Foo {
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
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.remaining() < 4 {
            return Err(DecodeError::LengthError {
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
