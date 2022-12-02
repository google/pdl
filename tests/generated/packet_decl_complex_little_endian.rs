// @generated rust packets from test

#![allow(warnings, missing_docs)]

use bytes::{Buf, BufMut, Bytes, BytesMut};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
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
    #[error("when parsing {obj} needed length of {wanted} but got {got}")]
    InvalidLengthError { obj: String, wanted: usize, got: usize },
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

#[derive(Debug)]
struct FooData {
    a: u8,
    b: u8,
    c: u8,
    d: u32,
    e: u16,
    f: u8,
}

#[derive(Debug, Clone)]
pub struct FooPacket {
    foo: Arc<FooData>,
}

#[derive(Debug)]
pub struct FooBuilder {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u32,
    pub e: u16,
    pub f: u8,
}

impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 7
    }
    fn parse(mut bytes: &[u8]) -> Result<Self> {
        if bytes.remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 2,
                got: bytes.remaining(),
            });
        }
        let chunk = bytes.get_u16_le();
        let a = (chunk & 0x7) as u8;
        let b = (chunk >> 3) as u8;
        let c = ((chunk >> 11) & 0x1f) as u8;
        if bytes.remaining() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 3,
                got: bytes.remaining(),
            });
        }
        let d = bytes.get_uint_le(3) as u32;
        if bytes.remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 2,
                got: bytes.remaining(),
            });
        }
        let chunk = bytes.get_u16_le();
        let e = (chunk & 0xfff);
        let f = ((chunk >> 12) & 0xf) as u8;
        Ok(Self { a, b, c, d, e, f })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let chunk = 0;
        let chunk = chunk | ((self.a as u16) & 0x7);
        let chunk = chunk | ((self.b as u16) << 3);
        let chunk = chunk | (((self.c as u16) & 0x1f) << 11);
        buffer.put_u16_le(chunk);
        let d = self.d;
        buffer.put_uint_le(d as u64, 3);
        let chunk = 0;
        let chunk = chunk | (self.e & 0xfff);
        let chunk = chunk | (((self.f as u16) & 0xf) << 12);
        buffer.put_u16_le(chunk);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        7
    }
}

impl Packet for FooPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.foo.get_total_size());
        self.foo.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<FooPacket> for Bytes {
    fn from(packet: FooPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<FooPacket> for Vec<u8> {
    fn from(packet: FooPacket) -> Self {
        packet.to_vec()
    }
}

impl FooPacket {
    pub fn parse(mut bytes: &[u8]) -> Result<Self> {
        Ok(Self::new(Arc::new(FooData::parse(bytes)?)).unwrap())
    }
    fn new(root: Arc<FooData>) -> std::result::Result<Self, &'static str> {
        let foo = root;
        Ok(Self { foo })
    }
    pub fn get_a(&self) -> u8 {
        self.foo.as_ref().a
    }
    pub fn get_b(&self) -> u8 {
        self.foo.as_ref().b
    }
    pub fn get_c(&self) -> u8 {
        self.foo.as_ref().c
    }
    pub fn get_d(&self) -> u32 {
        self.foo.as_ref().d
    }
    pub fn get_e(&self) -> u16 {
        self.foo.as_ref().e
    }
    pub fn get_f(&self) -> u8 {
        self.foo.as_ref().f
    }
}

impl FooBuilder {
    pub fn build(self) -> FooPacket {
        let foo =
            Arc::new(FooData { a: self.a, b: self.b, c: self.c, d: self.d, e: self.e, f: self.f });
        FooPacket::new(foo).unwrap()
    }
}
