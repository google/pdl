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
pub struct Foo<'a> {
    pub a: u32,
    pub payload: Array<'a, u8>,
}
impl<'a> Foo<'a> {
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn into_owned<'b>(self) -> Foo<'b> {
        Foo {
            a: self.a,
            payload: self.payload.into_owned(),
        }
    }
    pub fn a(&self) -> u32 {
        self.a
    }
}
impl<'a> Default for Foo<'a> {
    fn default() -> Foo<'a> {
        Foo {
            a: 0,
            payload: Array::default(),
        }
    }
}
impl<'a> Packet<'a> for Foo<'a> {
    fn encoded_len(&self) -> usize {
        3 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_slice(&self.payload);
        if self.a() > 0xff_ffff {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "a",
                value: self.a() as u64,
                maximum_value: 0xff_ffff as u64,
            });
        }
        buf.put_uint_le(self.a() as u64, 3);
        Ok(())
    }
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.remaining() < 3 {
            return Err(DecodeError::LengthError {
                obj: "Foo",
                wanted: 3,
                got: buf.remaining(),
            });
        }
        let (payload, tail) = buf.split_at(buf.len() - 3);
        buf = tail;
        let payload = Array::Borrowed(payload);
        if buf.remaining() < 3 {
            return Err(DecodeError::LengthError {
                obj: "Foo",
                wanted: 3,
                got: buf.remaining(),
            });
        }
        let a = buf.get_uint_le(3) as u32;
        Ok((Self { payload, a }, buf))
    }
}
