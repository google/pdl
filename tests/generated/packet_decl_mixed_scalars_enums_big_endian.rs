// @generated rust packets from test

use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::cell::Cell;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::sync::Arc;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[doc = r" Private prevents users from creating arbitrary scalar values"]
#[doc = r" in situations where the value needs to be validated."]
#[doc = r" Users can freely deref the value, but only the backend"]
#[doc = r" may create it."]
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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum Enum7 {
    A = 0x1,
    B = 0x2,
}
impl TryFrom<u8> for Enum7 {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Enum7::A),
            0x2 => Ok(Enum7::B),
            _ => Err(value),
        }
    }
}
impl From<&Enum7> for u8 {
    fn from(value: &Enum7) -> Self {
        match value {
            Enum7::A => 0x1,
            Enum7::B => 0x2,
        }
    }
}
impl From<Enum7> for u8 {
    fn from(value: Enum7) -> Self {
        (&value).into()
    }
}
impl From<Enum7> for i8 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for i16 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for i32 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for i64 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for u16 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for u32 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for u64 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u16", into = "u16"))]
pub enum Enum9 {
    A = 0x1,
    B = 0x2,
}
impl TryFrom<u16> for Enum9 {
    type Error = u16;
    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Enum9::A),
            0x2 => Ok(Enum9::B),
            _ => Err(value),
        }
    }
}
impl From<&Enum9> for u16 {
    fn from(value: &Enum9) -> Self {
        match value {
            Enum9::A => 0x1,
            Enum9::B => 0x2,
        }
    }
}
impl From<Enum9> for u16 {
    fn from(value: Enum9) -> Self {
        (&value).into()
    }
}
impl From<Enum9> for i16 {
    fn from(value: Enum9) -> Self {
        u16::from(value) as Self
    }
}
impl From<Enum9> for i32 {
    fn from(value: Enum9) -> Self {
        u16::from(value) as Self
    }
}
impl From<Enum9> for i64 {
    fn from(value: Enum9) -> Self {
        u16::from(value) as Self
    }
}
impl From<Enum9> for u32 {
    fn from(value: Enum9) -> Self {
        u16::from(value) as Self
    }
}
impl From<Enum9> for u64 {
    fn from(value: Enum9) -> Self {
        u16::from(value) as Self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooData {
    x: Enum7,
    y: u8,
    z: Enum9,
    w: u8,
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
    pub w: u8,
    pub x: Enum7,
    pub y: u8,
    pub z: Enum9,
}
impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 3
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 3,
                got: bytes.get().remaining(),
            });
        }
        let chunk = bytes.get_mut().get_uint(3) as u32;
        let x = Enum7::try_from((chunk & 0x7f) as u8).unwrap();
        let y = ((chunk >> 7) & 0x1f) as u8;
        let z = Enum9::try_from(((chunk >> 12) & 0x1ff) as u16).unwrap();
        let w = ((chunk >> 21) & 0x7) as u8;
        Ok(Self { x, y, z, w })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        if self.y > 0x1f {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "y", self.y, 0x1f);
        }
        if self.w > 0x7 {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "w", self.w, 0x7);
        }
        let value = (u8::from(self.x) as u32)
            | ((self.y as u32) << 7)
            | ((u16::from(self.z) as u32) << 12)
            | ((self.w as u32) << 21);
        buffer.put_uint(value as u64, 3);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        3
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
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = FooData::parse_inner(&mut bytes)?;
        Ok(Self::new(Arc::new(data)).unwrap())
    }
    fn new(foo: Arc<FooData>) -> std::result::Result<Self, &'static str> {
        Ok(Self { foo })
    }
    pub fn get_w(&self) -> u8 {
        self.foo.as_ref().w
    }
    pub fn get_x(&self) -> Enum7 {
        self.foo.as_ref().x
    }
    pub fn get_y(&self) -> u8 {
        self.foo.as_ref().y
    }
    pub fn get_z(&self) -> Enum9 {
        self.foo.as_ref().z
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
        let foo = Arc::new(FooData { w: self.w, x: self.x, y: self.y, z: self.z });
        Foo::new(foo).unwrap()
    }
}
impl From<FooBuilder> for Foo {
    fn from(builder: FooBuilder) -> Foo {
        builder.build().into()
    }
}
