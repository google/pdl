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
    pub padding: u8,
    pub x: Array<'a, u32>,
}
impl<'a> Foo<'a> {
    pub fn into_owned<'b>(self) -> Foo<'b> {
        Foo {
            padding: self.padding,
            x: self.x.into_owned(),
        }
    }
    pub fn padding(&self) -> u8 {
        self.padding
    }
    pub fn x(&self) -> &Array<'a, u32> {
        &self.x
    }
}
impl<'a> Default for Foo<'a> {
    fn default() -> Foo<'a> {
        Foo {
            padding: 0,
            x: Array::default(),
        }
    }
}
impl<'a> Packet<'a> for Foo<'a> {
    fn encoded_len(&self) -> usize {
        1 + (self.x.len() * 3)
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        #[allow(unused_comparisons)]
        if (self.x.len() * 3) > 0x1f {
            return Err(EncodeError::SizeOverflow {
                packet: "Foo",
                field: "x",
                size: (self.x.len() * 3),
                maximum_size: 0x1f,
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
        let value = ((self.x.len() * 3)) as u8 | (self.padding() << 5);
        buf.put_u8(value);
        for elem in &self.x {
            buf.put_uint(*elem as u64, 3);
        }
        Ok(())
    }
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.remaining() < 1 {
            return Err(DecodeError::LengthError {
                obj: "Foo",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let chunk = buf.get_u8();
        let x_size = (chunk & 0x1f) as usize;
        let padding = ((chunk >> 5) & 0x7);
        if buf.remaining() < x_size {
            return Err(DecodeError::LengthError {
                obj: "Foo",
                wanted: x_size,
                got: buf.remaining(),
            });
        }
        if x_size % 3 != 0 {
            return Err(DecodeError::ArraySizeError {
                array: x_size,
                element: 3,
            });
        }
        let x_count = x_size / 3;
        let (raw_slice, tail) = buf.split_at(x_size);
        buf = tail;
        let x = Array::from_raw_parts(
            x_count,
            raw_slice,
            None,
            |span| span.get_uint(3) as u32,
        );
        Ok((Self { padding, x }, buf))
    }
}
