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
        buf.put_uint(self.a.len() as u64, 5);
        for elem in &self.a {
            buf.put_u16(*elem);
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
        let a_count = buf.get_uint(5) as usize;
        if buf.remaining() < a_count * 2usize {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: a_count * 2usize,
                got: buf.remaining(),
            });
        }
        let a = (0..a_count)
            .map(|_| Ok::<_, DecodeError>(buf.get_u16()))
            .collect::<Result<Vec<_>, DecodeError>>()?;
        Ok((Self { a }, buf))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar {
    pub x: Vec<Foo>,
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
    pub fn x(&self) -> &Vec<Foo> {
        &self.x
    }
}
impl Packet for Bar {
    fn encoded_len(&self) -> usize {
        5 + self.x.iter().map(Packet::encoded_len).sum::<usize>()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        let x_size = self.x.iter().map(Packet::encoded_len).sum::<usize>();
        if x_size > 0xff_ffff_ffff_usize {
            return Err(EncodeError::SizeOverflow {
                packet: "Bar",
                field: "x",
                size: x_size,
                maximum_size: 0xff_ffff_ffff_usize,
            });
        }
        buf.put_uint(x_size as u64, 5);
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
        let x_size = buf.get_uint(5) as usize;
        if buf.remaining() < x_size {
            return Err(DecodeError::InvalidLengthError {
                obj: "Bar",
                wanted: x_size,
                got: buf.remaining(),
            });
        }
        let (mut head, tail) = buf.split_at(x_size);
        buf = tail;
        let mut x = Vec::new();
        while !head.is_empty() {
            x.push(Foo::decode_mut(&mut head)?);
        }
        Ok((Self { x }, buf))
    }
}
