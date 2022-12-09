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

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum Foo {
    A = 0x1,
    B = 0x2,
}

#[derive(Debug)]
struct BarData {
    x: Foo,
}
#[derive(Debug, Clone)]
pub struct BarPacket {
    bar: Arc<BarData>,
}
#[derive(Debug)]
pub struct BarBuilder {
    pub x: Foo,
}
impl BarData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 3
    }
    fn parse(mut bytes: &[u8]) -> Result<Self> {
        if bytes.remaining() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "Bar".to_string(),
                wanted: 3,
                got: bytes.remaining(),
            });
        }
        let x = Foo::from_u32(bytes.get_uint(3) as u32).unwrap();
        Ok(Self { x })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_uint(self.x.to_u32().unwrap() as u64, 3);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        3
    }
}
impl Packet for BarPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.bar.get_total_size());
        self.bar.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<BarPacket> for Bytes {
    fn from(packet: BarPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<BarPacket> for Vec<u8> {
    fn from(packet: BarPacket) -> Self {
        packet.to_vec()
    }
}
impl BarPacket {
    pub fn parse(mut bytes: &[u8]) -> Result<Self> {
        Ok(Self::new(Arc::new(BarData::parse(bytes)?)).unwrap())
    }
    fn new(root: Arc<BarData>) -> std::result::Result<Self, &'static str> {
        let bar = root;
        Ok(Self { bar })
    }
    pub fn get_x(&self) -> Foo {
        self.bar.as_ref().x
    }
}
impl BarBuilder {
    pub fn build(self) -> BarPacket {
        let bar = Arc::new(BarData { x: self.x });
        BarPacket::new(bar).unwrap()
    }
}
