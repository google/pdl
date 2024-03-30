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
    fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        if bytes.get().remaining() < 1 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Test",
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let r#type = bytes.get_mut().get_u8();
        Ok(Self { r#type })
    }
    fn write_to<T: BufMut>(&self, buffer: &mut T) -> Result<(), EncodeError> {
        buffer.put_u8(self.r#type);
        Ok(())
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for Test {
    fn encoded_len(&self) -> usize {
        self.get_size()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        self.test.write_to(buf)
    }
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        unimplemented!("Rust legacy does not implement full packet trait")
    }
}
impl TryFrom<Test> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: Test) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<Test> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: Test) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl Test {
    pub fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        let data = TestData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(test: TestData) -> Result<Self, DecodeError> {
        Ok(Self { test })
    }
    pub fn get_type(&self) -> u8 {
        self.test.r#type
    }
    fn write_to(&self, buffer: &mut impl BufMut) -> Result<(), EncodeError> {
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
