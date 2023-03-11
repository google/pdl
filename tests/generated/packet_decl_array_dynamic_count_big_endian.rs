// @generated rust packets from test

use bytes::{Buf, BufMut, Bytes, BytesMut};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::cell::Cell;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::sync::Arc;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

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
    #[error("array size ({array} bytes) is not a multiple of the element size ({element} bytes)")]
    InvalidArraySize { array: usize, element: usize },
    #[error("Due to size restrictions a struct could not be parsed.")]
    ImpossibleStructError,
    #[error("when parsing field {obj}.{field}, {value} is not a valid {type_} value")]
    InvalidEnumValueError { obj: String, field: String, value: u64, type_: String },
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct TryFromError(&'static str);

pub trait Packet {
    fn to_bytes(self) -> Bytes;
    fn to_vec(self) -> Vec<u8>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooData {
    padding: u8,
    x: Vec<u32>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    #[cfg_attr(feature = "serde", serde(flatten))]
    foo: Arc<FooData>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooBuilder {
    pub padding: u8,
    pub x: Vec<u32>,
}
impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let chunk = bytes.get_mut().get_u8();
        let x_count = (chunk & 0x1f) as usize;
        let padding = ((chunk >> 5) & 0x7);
        if bytes.get().remaining() < x_count {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: x_count,
                got: bytes.get().remaining(),
            });
        }
        let x = (0..x_count)
            .map(|_| Ok::<_, Error>(bytes.get_mut().get_uint(3) as u32))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { padding, x })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        if self.x.len() > 0x1f {
            panic!("Invalid length for {}::{}: {} > {}", "Foo", "x", self.x.len(), 0x1f);
        }
        if self.padding > 0x7 {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "padding", self.padding, 0x7);
        }
        let value = self.x.len() as u8 | (self.padding << 5);
        buffer.put_u8(value);
        for elem in &self.x {
            buffer.put_uint(*elem as u64, 3);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self.x.len() * 3
    }
}
impl Packet for Foo {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.foo.get_size());
        self.foo.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Foo> for Bytes {
    fn from(packet: Foo) -> Self {
        packet.to_bytes()
    }
}
impl From<Foo> for Vec<u8> {
    fn from(packet: Foo) -> Self {
        packet.to_vec()
    }
}
impl Foo {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        if !cell.get().is_empty() {
            return Err(Error::InvalidPacketError);
        }
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = FooData::parse(&mut bytes)?;
        Ok(Self::new(Arc::new(data)).unwrap())
    }
    fn new(foo: Arc<FooData>) -> std::result::Result<Self, &'static str> {
        Ok(Self { foo })
    }
    pub fn get_padding(&self) -> u8 {
        self.foo.as_ref().padding
    }
    pub fn get_x(&self) -> &Vec<u32> {
        &self.foo.as_ref().x
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.foo.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.foo.get_size()
    }
}
impl FooBuilder {
    pub fn build(self) -> Foo {
        let foo = Arc::new(FooData { padding: self.padding, x: self.x });
        Foo::new(foo).unwrap()
    }
}
impl From<FooBuilder> for Foo {
    fn from(builder: FooBuilder) -> Foo {
        builder.build().into()
    }
}
