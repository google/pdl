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
    pub a: Vec<u16>,
}
impl Foo {
    pub fn a(&self) -> &Vec<u16> {
        &self.a
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        5 + self.a.len() * 2
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        if self.a.len() > 0xff_ffff_ffff_usize {
            return Err(EncodeError::CountOverflow {
                packet: "Foo",
                field: "a",
                count: self.a.len(),
                maximum_count: 0xff_ffff_ffff_usize,
            });
        }
        buf.put_uint_le(self.a.len() as u64, 5);
        for elem in &self.a {
            buf.put_u16_le(*elem);
        }
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 5 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 5,
                got: buf.remaining(),
            });
        }
        let a_count = buf.get_uint_le(5) as usize;
        if buf.remaining() < a_count * 2usize {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: a_count * 2usize,
                got: buf.remaining(),
            });
        }
        let a = (0..a_count)
            .map(|_| Ok::<_, DecodeError>(buf.get_u16_le()))
            .collect::<Result<Vec<_>, DecodeError>>()?;
        Ok((Self { a }, buf))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar {
    pub x: Vec<Foo>,
}
impl Bar {
    pub fn x(&self) -> &Vec<Foo> {
        &self.x
    }
}
impl Packet for Bar {
    fn encoded_len(&self) -> usize {
        5 + self.x.iter().map(Packet::encoded_len).sum::<usize>()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        if self.x.len() > 0xff_ffff_ffff_usize {
            return Err(EncodeError::CountOverflow {
                packet: "Bar",
                field: "x",
                count: self.x.len(),
                maximum_count: 0xff_ffff_ffff_usize,
            });
        }
        buf.put_uint_le(self.x.len() as u64, 5);
        for elem in &self.x {
            elem.encode(buf)?;
        }
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 5 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Bar",
                wanted: 5,
                got: buf.remaining(),
            });
        }
        let x_count = buf.get_uint_le(5) as usize;
        let x = (0..x_count)
            .map(|_| Foo::decode_mut(&mut buf))
            .collect::<Result<Vec<_>, DecodeError>>()?;
        Ok((Self { x }, buf))
    }
}
