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
    pub b: u64,
}
impl Foo {
    pub fn b(&self) -> u64 {
        self.b
    }
}
impl Default for Foo {
    fn default() -> Foo {
        Foo { b: 0 }
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        8
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        if self.b() > 0x1ff_ffff_ffff_ffff_u64 {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "b",
                value: self.b() as u64,
                maximum_value: 0x1ff_ffff_ffff_ffff_u64 as u64,
            });
        }
        let value = (7 as u64) | (self.b() << 7);
        buf.put_u64(value);
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 8 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 8,
                got: buf.remaining(),
            });
        }
        let chunk = buf.get_u64();
        let fixed_value = (chunk & 0x7f) as u8;
        if fixed_value != 7 {
            return Err(DecodeError::InvalidFixedValue {
                expected: 7,
                actual: fixed_value as u64,
            });
        }
        let b = ((chunk >> 7) & 0x1ff_ffff_ffff_ffff_u64);
        Ok((Self { b }, buf))
    }
}
