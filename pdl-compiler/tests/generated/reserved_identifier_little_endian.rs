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
pub struct TestData {
    r#type: u8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Test {
    #[cfg_attr(feature = "serde", serde(flatten))]
    test: TestData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TestBuilder {
    pub r#type: u8,
}
impl TestData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "Test".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let r#type = bytes.get_mut().get_u8();
        Ok(Self { r#type })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.r#type);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for Test {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.test.get_size());
        self.test.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Test> for Bytes {
    fn from(packet: Test) -> Self {
        packet.to_bytes()
    }
}
impl From<Test> for Vec<u8> {
    fn from(packet: Test) -> Self {
        packet.to_vec()
    }
}
impl Test {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = TestData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(test: TestData) -> Result<Self> {
        Ok(Self { test })
    }
    pub fn get_type(&self) -> u8 {
        self.test.r#type
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.test.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.test.get_size()
    }
}
impl TestBuilder {
    pub fn build(self) -> Test {
        let test = TestData { r#type: self.r#type };
        Test::new(test).unwrap()
    }
}
impl From<TestBuilder> for Test {
    fn from(builder: TestBuilder) -> Test {
        builder.build().into()
    }
}
