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
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 5
    }
    pub fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        if bytes.get().remaining() < 5 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 5,
                got: bytes.get().remaining(),
            });
        }
        let a_count = bytes.get_mut().get_uint(5) as usize;
        if bytes.get().remaining() < a_count * 2usize {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: a_count * 2usize,
                got: bytes.get().remaining(),
            });
        }
        let a = (0..a_count)
            .map(|_| Ok::<_, DecodeError>(bytes.get_mut().get_u16()))
            .collect::<Result<Vec<_>, DecodeError>>()?;
        Ok(Self { a })
    }
    fn write_to<T: BufMut>(&self, buffer: &mut T) -> Result<(), EncodeError> {
        if self.a.len() > 0xff_ffff_ffff_usize {
            return Err(EncodeError::CountOverflow {
                packet: "Foo",
                field: "a",
                count: self.a.len(),
                maximum_count: 0xff_ffff_ffff_usize,
            });
        }
        buffer.put_uint(self.a.len() as u64, 5);
        for elem in &self.a {
            buffer.put_u16(*elem);
        }
        Ok(())
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        5 + self.a.len() * 2
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarData {
    a: Vec<Foo>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar {
    #[cfg_attr(feature = "serde", serde(flatten))]
    bar: BarData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarBuilder {
    pub a: Vec<Foo>,
}
impl BarData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 128
    }
    fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        if bytes.get().remaining() < 128usize {
            return Err(DecodeError::InvalidLengthError {
                obj: "Bar",
                wanted: 128usize,
                got: bytes.get().remaining(),
            });
        }
        let (head, tail) = bytes.get().split_at(128usize);
        let mut head = &mut Cell::new(head);
        bytes.replace(tail);
        let mut a = Vec::new();
        while !head.get().is_empty() {
            a.push(Foo::parse_inner(head)?);
        }
        Ok(Self { a })
    }
    fn write_to<T: BufMut>(&self, buffer: &mut T) -> Result<(), EncodeError> {
        let array_size = self.a.iter().fold(0, |size, elem| size + elem.get_size());
        if array_size > 128usize {
            return Err(EncodeError::SizeOverflow {
                packet: "Bar",
                field: "a",
                size: array_size,
                maximum_size: 128usize,
            });
        }
        for elem in &self.a {
            elem.write_to(buffer)?;
        }
        buffer.put_bytes(0, 128usize - array_size);
        Ok(())
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        128
    }
}
impl Packet for Bar {
    fn encoded_len(&self) -> usize {
        self.get_size()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        self.bar.write_to(buf)
    }
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        unimplemented!("Rust legacy does not implement full packet trait")
    }
}
impl TryFrom<Bar> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: Bar) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<Bar> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: Bar) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl Bar {
    pub fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        let data = BarData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(bar: BarData) -> Result<Self, DecodeError> {
        Ok(Self { bar })
    }
    pub fn get_a(&self) -> &Vec<Foo> {
        &self.bar.a
    }
    fn write_to(&self, buffer: &mut impl BufMut) -> Result<(), EncodeError> {
        self.bar.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.bar.get_size()
    }
}
impl BarBuilder {
    pub fn build(self) -> Bar {
        let bar = BarData { a: self.a };
        Bar::new(bar).unwrap()
    }
}
impl From<BarBuilder> for Bar {
    fn from(builder: BarBuilder) -> Bar {
        builder.build().into()
    }
}
