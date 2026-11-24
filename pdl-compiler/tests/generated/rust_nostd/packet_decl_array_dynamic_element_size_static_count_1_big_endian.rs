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
    pub inner: Array<'a, u8>,
}
impl<'a> Foo<'a> {
    pub fn into_owned<'b>(self) -> Foo<'b> {
        Foo {
            inner: self.inner.into_owned(),
        }
    }
    pub fn inner(&self) -> &Array<'a, u8> {
        &self.inner
    }
}
impl<'a> Default for Foo<'a> {
    fn default() -> Foo<'a> {
        Foo { inner: Array::default() }
    }
}
impl<'a> Packet<'a> for Foo<'a> {
    fn encoded_len(&self) -> usize {
        self.inner.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        for elem in &self.inner {
            buf.put_u8(*elem);
        }
        Ok(())
    }
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        let (raw_slice, tail) = buf.split_at(buf.remaining());
        buf = tail;
        let inner = Array::Borrowed(raw_slice);
        Ok((Self { inner }, buf))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar<'a> {
    pub padding: u8,
    pub x: [Foo<'a>; 1],
}
impl<'a> Bar<'a> {
    pub fn into_owned<'b>(self) -> Bar<'b> {
        Bar {
            padding: self.padding,
            x: self.x.map(Foo::into_owned),
        }
    }
    pub fn padding(&self) -> u8 {
        self.padding
    }
    pub fn x(&self) -> &[Foo<'a>; 1] {
        &self.x
    }
}
impl<'a> Default for Bar<'a> {
    fn default() -> Bar<'a> {
        Bar {
            padding: 0,
            x: core::array::from_fn(|_| Default::default()),
        }
    }
}
impl<'a> Packet<'a> for Bar<'a> {
    fn encoded_len(&self) -> usize {
        1 + self.x.iter().map(|elem| elem.encoded_len()).sum::<usize>()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        let x_element_size = self.x.iter().next().map_or(0, |elem| elem.encoded_len());
        for (element_index, element) in self.x.iter().enumerate() {
            if element.encoded_len() != x_element_size {
                return Err(EncodeError::InvalidArrayElementSize {
                    packet: "Bar",
                    field: "x",
                    size: element.encoded_len(),
                    expected_size: x_element_size,
                    element_index,
                });
            }
        }
        if x_element_size > 0x1f {
            return Err(EncodeError::SizeOverflow {
                packet: "Bar",
                field: "x",
                size: x_element_size,
                maximum_size: 0x1f,
            });
        }
        let x_element_size = x_element_size as u8;
        if self.padding() > 0x7 {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Bar",
                field: "padding",
                value: self.padding() as u64,
                maximum_value: 0x7 as u64,
            });
        }
        let value = x_element_size | (self.padding() << 5);
        buf.put_u8(value);
        for elem in &self.x {
            elem.encode(buf)?;
        }
        Ok(())
    }
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.remaining() < 1 {
            return Err(DecodeError::LengthError {
                obj: "Bar",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let chunk = buf.get_u8();
        let x_element_size = (chunk & 0x1f) as usize;
        let padding = ((chunk >> 5) & 0x7);
        if x_element_size == 0 {
            return Err(DecodeError::LengthError {
                obj: "Bar",
                wanted: 1,
                got: 0,
            });
        }
        if buf.remaining() < x_element_size {
            return Err(DecodeError::LengthError {
                obj: "Bar",
                wanted: x_element_size,
                got: buf.remaining(),
            });
        }
        let mut x = core::array::from_fn::<_, 1, _>(|_| Default::default());
        for elem in &mut x {
            let (mut chunk, tail) = buf.split_at(x_element_size);
            buf = tail;
            let value = Foo::decode_mut(&mut chunk)?;
            if !chunk.is_empty() {
                return Err(DecodeError::TrailingBytesInArray {
                    obj: "Bar",
                    field: "x",
                });
            }
            *elem = value;
        }
        Ok((Self { padding, x }, buf))
    }
}
