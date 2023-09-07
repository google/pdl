#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use pdl_runtime::{Error, Packet, Private};
type Result<T> = std::result::Result<T, Error>;
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
