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
    pub padding: u8,
    pub x: Vec<u32>,
}
impl Foo {
    pub fn padding(&self) -> u8 {
        self.padding
    }
    pub fn x(&self) -> &Vec<u32> {
        &self.x
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        1 + self.x.len() * 3
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        if self.x.len() > 0x1f {
            return Err(EncodeError::CountOverflow {
                packet: "Foo",
                field: "x",
                count: self.x.len(),
                maximum_count: 0x1f,
            });
        }
        if self.padding() > 0x7 {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "padding",
                value: self.padding() as u64,
                maximum_value: 0x7 as u64,
            });
        }
        let value = self.x.len() as u8 | (self.padding() << 5);
        buf.put_u8(value);
        for elem in &self.x {
            buf.put_uint_le(*elem as u64, 3);
        }
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
        let chunk = buf.get_u8();
        let x_count = (chunk & 0x1f) as usize;
        let padding = ((chunk >> 5) & 0x7);
        if buf.remaining() < x_count * 3usize {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: x_count * 3usize,
                got: buf.remaining(),
            });
        }
        let x = (0..x_count)
            .map(|_| Ok::<_, DecodeError>(buf.get_uint_le(3) as u32))
            .collect::<Result<Vec<_>, DecodeError>>()?;
        Ok((Self { padding, x }, buf))
    }
}
