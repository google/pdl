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
pub enum FooDataChild {
    Payload(Bytes),
    None,
}
impl FooDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            FooDataChild::Payload(bytes) => bytes.len(),
            FooDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FooChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooData {
    a: u8,
    b: u16,
    child: FooDataChild,
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
    pub a: u8,
    pub b: u16,
    pub payload: Option<Bytes>,
}
impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 4
    }
    fn parse(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let a = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let payload_size = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < payload_size {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: payload_size,
                got: bytes.get().remaining(),
            });
        }
        let payload = &bytes.get()[..payload_size];
        bytes.get_mut().advance(payload_size);
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let b = bytes.get_mut().get_u16();
        let child = match () {
            _ if !payload.is_empty() => FooDataChild::Payload(Bytes::copy_from_slice(payload)),
            _ => FooDataChild::None,
        };
        Ok(Self { a, b, child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.a);
        if self.child.get_total_size() > 0xff {
            panic!(
                "Invalid length for {}::{}: {} > {}",
                "Foo",
                "_payload_",
                self.child.get_total_size(),
                0xff
            );
        }
        buffer.put_u8(self.child.get_total_size() as u8);
        match &self.child {
            FooDataChild::Payload(payload) => buffer.put_slice(payload),
            FooDataChild::None => {}
        }
        buffer.put_u16(self.b);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        4 + self.child.get_total_size()
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
    pub fn specialize(&self) -> FooChild {
        match &self.foo.child {
            FooDataChild::Payload(payload) => FooChild::Payload(payload.clone()),
            FooDataChild::None => FooChild::None,
        }
    }
    fn new(foo: Arc<FooData>) -> std::result::Result<Self, &'static str> {
        Ok(Self { foo })
    }
    pub fn get_a(&self) -> u8 {
        self.foo.as_ref().a
    }
    pub fn get_b(&self) -> u16 {
        self.foo.as_ref().b
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.foo.child {
            FooDataChild::Payload(bytes) => &bytes,
            FooDataChild::None => &[],
        }
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
        let foo = Arc::new(FooData {
            a: self.a,
            b: self.b,
            child: match self.payload {
                None => FooDataChild::None,
                Some(bytes) => FooDataChild::Payload(bytes),
            },
        });
        Foo::new(foo).unwrap()
    }
}
impl From<FooBuilder> for Foo {
    fn from(builder: FooBuilder) -> Foo {
        builder.build().into()
    }
}
