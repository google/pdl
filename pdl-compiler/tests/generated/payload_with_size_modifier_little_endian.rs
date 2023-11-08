#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use pdl_runtime::{Error, Packet};
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TestDataChild {
    Payload(Bytes),
    None,
}
impl TestDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            TestDataChild::Payload(bytes) => bytes.len(),
            TestDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TestChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TestData {
    child: TestDataChild,
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
    pub payload: Option<Bytes>,
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
        let payload_size = bytes.get_mut().get_u8() as usize;
        if payload_size < 1 {
            return Err(Error::InvalidLengthError {
                obj: "Test".to_string(),
                wanted: 1,
                got: payload_size,
            });
        }
        let payload_size = payload_size - 1;
        if bytes.get().remaining() < payload_size {
            return Err(Error::InvalidLengthError {
                obj: "Test".to_string(),
                wanted: payload_size,
                got: bytes.get().remaining(),
            });
        }
        let payload = &bytes.get()[..payload_size];
        bytes.get_mut().advance(payload_size);
        let child = match () {
            _ if !payload.is_empty() => {
                TestDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => TestDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        if (self.child.get_total_size() + 1) > 0xff {
            panic!(
                "Invalid length for {}::{}: {} > {}", "Test", "_payload_", (self.child
                .get_total_size() + 1), 0xff
            );
        }
        buffer.put_u8((self.child.get_total_size() + 1) as u8);
        match &self.child {
            TestDataChild::Payload(payload) => buffer.put_slice(payload),
            TestDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self.child.get_total_size()
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
    pub fn specialize(&self) -> TestChild {
        match &self.test.child {
            TestDataChild::Payload(payload) => TestChild::Payload(payload.clone()),
            TestDataChild::None => TestChild::None,
        }
    }
    fn new(test: TestData) -> Result<Self> {
        Ok(Self { test })
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.test.child {
            TestDataChild::Payload(bytes) => &bytes,
            TestDataChild::None => &[],
        }
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
        let test = TestData {
            child: match self.payload {
                None => TestDataChild::None,
                Some(bytes) => TestDataChild::Payload(bytes),
            },
        };
        Test::new(test).unwrap()
    }
}
impl From<TestBuilder> for Test {
    fn from(builder: TestBuilder) -> Test {
        builder.build().into()
    }
}
