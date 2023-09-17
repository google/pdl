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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Private<T>(T);
impl<T> std::ops::Deref for Private<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u32", into = "u32"))]
pub enum Foo {
    A = 0x1,
    B = 0x2,
}
impl TryFrom<u32> for Foo {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Foo::A),
            0x2 => Ok(Foo::B),
            _ => Err(value),
        }
    }
}
impl From<&Foo> for u32 {
    fn from(value: &Foo) -> Self {
        match value {
            Foo::A => 0x1,
            Foo::B => 0x2,
        }
    }
}
impl From<Foo> for u32 {
    fn from(value: Foo) -> Self {
        (&value).into()
    }
}
impl From<Foo> for i32 {
    fn from(value: Foo) -> Self {
        u32::from(value) as Self
    }
}
impl From<Foo> for i64 {
    fn from(value: Foo) -> Self {
        u32::from(value) as Self
    }
}
impl From<Foo> for u64 {
    fn from(value: Foo) -> Self {
        u32::from(value) as Self
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarData {
    x: Foo,
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
    pub x: Foo,
}
impl BarData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 3
    }
    fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        if bytes.get().remaining() < 3 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Bar",
                wanted: 3,
                got: bytes.get().remaining(),
            });
        }
        let x = Foo::try_from(bytes.get_mut().get_uint_le(3) as u32)
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Bar",
                field: "x",
                value: unknown_val as u64,
                type_: "Foo",
            })?;
        Ok(Self { x })
    }
    fn write_to<T: BufMut>(&self, buffer: &mut T) -> Result<(), EncodeError> {
        buffer.put_uint_le(u32::from(self.x) as u64, 3);
        Ok(())
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        3
    }
}
impl Packet for Bar {
    fn encoded_len(&self) -> usize {
        self.get_size()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        self.bar.write_to(buf)
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
    pub fn get_x(&self) -> Foo {
        self.bar.x
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
        let bar = BarData { x: self.x };
        Bar::new(bar).unwrap()
    }
}
impl From<BarBuilder> for Bar {
    fn from(builder: BarBuilder) -> Bar {
        builder.build().into()
    }
}
