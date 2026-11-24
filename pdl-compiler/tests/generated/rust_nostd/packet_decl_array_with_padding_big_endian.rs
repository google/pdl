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
    pub a: Array<'a, u16>,
}
impl<'a> Foo<'a> {
    pub fn into_owned<'b>(self) -> Foo<'b> {
        Foo { a: self.a.into_owned() }
    }
    pub fn a(&self) -> &Array<'a, u16> {
        &self.a
    }
}
impl<'a> Default for Foo<'a> {
    fn default() -> Foo<'a> {
        Foo { a: Array::default() }
    }
}
impl<'a> Packet<'a> for Foo<'a> {
    fn encoded_len(&self) -> usize {
        5 + (self.a.len() * 2)
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
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.remaining() < 5 {
            return Err(DecodeError::LengthError {
                obj: "Foo",
                wanted: 5,
                got: buf.remaining(),
            });
        }
        let a_count = buf.get_uint(5) as usize;
        if buf.remaining() < a_count * 2usize {
            return Err(DecodeError::LengthError {
                obj: "Foo",
                wanted: a_count * 2usize,
                got: buf.remaining(),
            });
        }
        let (raw_slice, tail) = buf.split_at(a_count * 2usize);
        buf = tail;
        let a = Array::from_raw_parts(a_count, raw_slice, None, |span| span.get_u16());
        Ok((Self { a }, buf))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar<'a> {
    pub a: Array<'a, Foo<'a>>,
}
impl<'a> Bar<'a> {
    pub fn into_owned<'b>(self) -> Bar<'b> {
        Bar {
            a: self.a.map_owned(Foo::into_owned),
        }
    }
    pub fn a(&self) -> &Array<'a, Foo<'a>> {
        &self.a
    }
}
impl<'a> Default for Bar<'a> {
    fn default() -> Bar<'a> {
        Bar { a: Array::default() }
    }
}
impl<'a> Packet<'a> for Bar<'a> {
    fn encoded_len(&self) -> usize {
        128
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        let array_size = self.a.iter().map(|elem| elem.encoded_len()).sum::<usize>();
        if array_size > 128usize {
            return Err(EncodeError::SizeOverflow {
                packet: "Bar",
                field: "a",
                size: array_size,
                maximum_size: 128usize,
            });
        }
        for elem in &self.a {
            elem.encode(buf)?;
        }
        buf.put_bytes(0, 128usize - array_size);
        Ok(())
    }
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.remaining() < 128usize {
            return Err(DecodeError::LengthError {
                obj: "Bar",
                wanted: 128usize,
                got: buf.remaining(),
            });
        }
        let (mut head, tail) = buf.split_at(128usize);
        buf = tail;
        let raw_slice = head;
        let mut a_count = 0;
        while !head.is_empty() {
            Foo::decode_mut(&mut head)?;
            a_count += 1;
        }
        let a = Array::from_raw_parts(
            a_count,
            raw_slice,
            None,
            |span| Foo::decode_mut(span).unwrap(),
        );
        Ok((Self { a }, buf))
    }
}
