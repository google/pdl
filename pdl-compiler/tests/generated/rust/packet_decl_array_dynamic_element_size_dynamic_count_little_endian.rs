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
    pub inner: Vec<u8>,
}
impl Foo {
    pub fn inner(&self) -> &Vec<u8> {
        &self.inner
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        self.inner.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        for elem in &self.inner {
            buf.put_u8(*elem);
        }
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let mut inner = Vec::with_capacity(buf.remaining());
        for _ in 0..buf.remaining() {
            inner.push(Ok::<_, DecodeError>(buf.get_u8())?);
        }
        Ok((Self { inner }, buf))
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
        1 + self.x.iter().map(Packet::encoded_len).sum::<usize>()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        if self.x.len() > 0xf {
            return Err(EncodeError::CountOverflow {
                packet: "Bar",
                field: "x",
                count: self.x.len(),
                maximum_count: 0xf,
            });
        }
        let x_element_size = self.x.get(0).map_or(0, Packet::encoded_len);
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
        if x_element_size > 0xf {
            return Err(EncodeError::SizeOverflow {
                packet: "Bar",
                field: "x",
                size: x_element_size,
                maximum_size: 0xf,
            });
        }
        let x_element_size = x_element_size as u8;
        let value = self.x.len() as u8 | (x_element_size << 4);
        buf.put_u8(value);
        for elem in &self.x {
            elem.encode(buf)?;
        }
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 1 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Bar",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let chunk = buf.get_u8();
        let x_count = (chunk & 0xf) as usize;
        let x_element_size = ((chunk >> 4) & 0xf) as usize;
        if buf.remaining() < x_count * x_element_size {
            return Err(DecodeError::InvalidLengthError {
                obj: "Bar",
                wanted: x_count * x_element_size,
                got: buf.remaining(),
            });
        }
        let x = buf
            .chunks(x_element_size)
            .take(x_count)
            .map(|mut chunk| {
                Foo::decode_mut(&mut chunk)
                    .and_then(|value| {
                        if chunk.is_empty() {
                            Ok(value)
                        } else {
                            Err(DecodeError::TrailingBytesInArray {
                                obj: "Bar",
                                field: "x",
                            })
                        }
                    })
            })
            .collect::<Result<Vec<_>, DecodeError>>()?;
        buf = &buf[(x_element_size * x_count)..];
        Ok((Self { x }, buf))
    }
}
