#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use thiserror::Error;
type Result<T> = std::result::Result<T, Error>;
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
#[derive(Debug, Error)]
pub enum Error {
    #[error("Packet parsing failed")]
    InvalidPacketError,
    #[error("{field} was {value:x}, which is not known")]
    ConstraintOutOfBounds { field: String, value: u64 },
    #[error("Got {actual:x}, expected {expected:x}")]
    InvalidFixedValue { expected: u64, actual: u64 },
    #[error("when parsing {obj} needed length of {wanted} but got {got}")]
    InvalidLengthError { obj: String, wanted: usize, got: usize },
    #[error(
        "array size ({array} bytes) is not a multiple of the element size ({element} bytes)"
    )]
    InvalidArraySize { array: usize, element: usize },
    #[error("Due to size restrictions a struct could not be parsed.")]
    ImpossibleStructError,
    #[error("when parsing field {obj}.{field}, {value} is not a valid {type_} value")]
    InvalidEnumValueError { obj: String, field: String, value: u64, type_: String },
    #[error("expected child {expected}, got {actual}")]
    InvalidChildError { expected: &'static str, actual: String },
}
pub trait Packet {
    fn to_bytes(self) -> Bytes;
    fn to_vec(self) -> Vec<u8>;
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
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 5,
                got: bytes.get().remaining(),
            });
        }
        let a_count = bytes.get_mut().get_uint_le(5) as usize;
        if bytes.get().remaining() < a_count * 2usize {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: a_count * 2usize,
                got: bytes.get().remaining(),
            });
        }
        let a = (0..a_count)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_u16_le()))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { a })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        if self.a.len() > 0xff_ffff_ffff_usize {
            panic!(
                "Invalid length for {}::{}: {} > {}", "Foo", "a", self.a.len(),
                0xff_ffff_ffff_usize
            );
        }
        buffer.put_uint_le(self.a.len() as u64, 5);
        for elem in &self.a {
            buffer.put_u16_le(*elem);
        }
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
    x: Vec<Foo>,
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
    pub x: Vec<Foo>,
}
impl BarData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 5
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "Bar".to_string(),
                wanted: 5,
                got: bytes.get().remaining(),
            });
        }
        let x_count = bytes.get_mut().get_uint_le(5) as usize;
        let x = (0..x_count)
            .map(|_| Foo::parse_inner(bytes))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { x })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        if self.x.len() > 0xff_ffff_ffff_usize {
            panic!(
                "Invalid length for {}::{}: {} > {}", "Bar", "x", self.x.len(),
                0xff_ffff_ffff_usize
            );
        }
        buffer.put_uint_le(self.x.len() as u64, 5);
        for elem in &self.x {
            elem.write_to(buffer);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        5 + self.x.iter().map(|elem| elem.get_size()).sum::<usize>()
    }
}
impl Packet for Bar {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.bar.get_size());
        self.bar.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Bar> for Bytes {
    fn from(packet: Bar) -> Self {
        packet.to_bytes()
    }
}
impl From<Bar> for Vec<u8> {
    fn from(packet: Bar) -> Self {
        packet.to_vec()
    }
}
impl Bar {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = BarData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(bar: BarData) -> Result<Self> {
        Ok(Self { bar })
    }
    pub fn get_x(&self) -> &Vec<Foo> {
        &self.bar.x
    }
    fn write_to(&self, buffer: &mut BytesMut) {
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
