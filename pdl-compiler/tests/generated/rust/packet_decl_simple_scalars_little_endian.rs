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
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    pub x: u8,
    pub y: u16,
    pub z: u32,
}
impl Foo {
    pub fn x(&self) -> u8 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn z(&self) -> u32 {
        self.z
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        6
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(self.x());
        buf.put_u16_le(self.y());
        if self.z() > 0xff_ffff {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "z",
                value: self.z() as u64,
                maximum_value: 0xff_ffff as u64,
            });
        }
        buf.put_uint_le(self.z() as u64, 3);
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 1 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let x = buf.get_u8();
        if buf.remaining() < 2 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let y = buf.get_u16_le();
        if buf.remaining() < 3 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 3,
                got: buf.remaining(),
            });
        }
        let z = buf.get_uint_le(3) as u32;
        Ok((Self { x, y, z }, buf))
    }
}
