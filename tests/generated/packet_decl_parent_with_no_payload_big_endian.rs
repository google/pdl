#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use std::sync::Arc;
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
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum Enum8 {
    A = 0x0,
}
impl TryFrom<u8> for Enum8 {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Enum8::A),
            _ => Err(value),
        }
    }
}
impl From<&Enum8> for u8 {
    fn from(value: &Enum8) -> Self {
        match value {
            Enum8::A => 0x0,
        }
    }
}
impl From<Enum8> for u8 {
    fn from(value: Enum8) -> Self {
        (&value).into()
    }
}
impl From<Enum8> for i16 {
    fn from(value: Enum8) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum8> for i32 {
    fn from(value: Enum8) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum8> for i64 {
    fn from(value: Enum8) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum8> for u16 {
    fn from(value: Enum8) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum8> for u32 {
    fn from(value: Enum8) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum8> for u64 {
    fn from(value: Enum8) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParentDataChild {
    Child(Arc<ChildData>),
    Payload(Bytes),
    None,
}
impl ParentDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            ParentDataChild::Child(value) => value.get_total_size(),
            ParentDataChild::Payload(bytes) => bytes.len(),
            ParentDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParentChild {
    Child(Child),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParentData {
    v: Enum8,
    child: ParentDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parent {
    #[cfg_attr(feature = "serde", serde(flatten))]
    parent: Arc<ParentData>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParentBuilder {
    pub v: Enum8,
    pub payload: Option<Bytes>,
}
impl ParentData {
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
                obj: "Parent".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let v = Enum8::try_from(bytes.get_mut().get_u8())
            .map_err(|_| Error::InvalidEnumValueError {
                obj: "Parent".to_string(),
                field: "v".to_string(),
                value: bytes.get_mut().get_u8() as u64,
                type_: "Enum8".to_string(),
            })?;
        let payload: &[u8] = &[];
        let child = match (v) {
            (Enum8::A) if ChildData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = ChildData::parse_inner(&mut cell)?;
                ParentDataChild::Child(Arc::new(child_data))
            }
            _ if !payload.is_empty() => {
                ParentDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => ParentDataChild::None,
        };
        Ok(Self { v, child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(u8::from(self.v));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for Parent {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.parent.get_size());
        self.parent.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Parent> for Bytes {
    fn from(packet: Parent) -> Self {
        packet.to_bytes()
    }
}
impl From<Parent> for Vec<u8> {
    fn from(packet: Parent) -> Self {
        packet.to_vec()
    }
}
impl Parent {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = ParentData::parse_inner(&mut bytes)?;
        Self::new(Arc::new(data))
    }
    pub fn specialize(&self) -> ParentChild {
        match &self.parent.child {
            ParentDataChild::Child(_) => {
                ParentChild::Child(Child::new(self.parent.clone()).unwrap())
            }
            ParentDataChild::Payload(payload) => ParentChild::Payload(payload.clone()),
            ParentDataChild::None => ParentChild::None,
        }
    }
    fn new(parent: Arc<ParentData>) -> Result<Self> {
        Ok(Self { parent })
    }
    pub fn get_v(&self) -> Enum8 {
        self.parent.as_ref().v
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.parent.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.parent.get_size()
    }
}
impl ParentBuilder {
    pub fn build(self) -> Parent {
        let parent = Arc::new(ParentData {
            v: self.v,
            child: ParentDataChild::None,
        });
        Parent::new(parent).unwrap()
    }
}
impl From<ParentBuilder> for Parent {
    fn from(builder: ParentBuilder) -> Parent {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChildData {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Child {
    #[cfg_attr(feature = "serde", serde(flatten))]
    parent: Arc<ParentData>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    child: Arc<ChildData>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChildBuilder {}
impl ChildData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        0
    }
}
impl Packet for Child {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.parent.get_size());
        self.parent.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Child> for Bytes {
    fn from(packet: Child) -> Self {
        packet.to_bytes()
    }
}
impl From<Child> for Vec<u8> {
    fn from(packet: Child) -> Self {
        packet.to_vec()
    }
}
impl From<Child> for Parent {
    fn from(packet: Child) -> Parent {
        Parent::new(packet.parent).unwrap()
    }
}
impl TryFrom<Parent> for Child {
    type Error = Error;
    fn try_from(packet: Parent) -> Result<Child> {
        Child::new(packet.parent)
    }
}
impl Child {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = ParentData::parse_inner(&mut bytes)?;
        Self::new(Arc::new(data))
    }
    fn new(parent: Arc<ParentData>) -> Result<Self> {
        let child = match &parent.child {
            ParentDataChild::Child(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(ParentDataChild::Child),
                    actual: format!("{:?}", & parent.child),
                });
            }
        };
        Ok(Self { parent, child })
    }
    pub fn get_v(&self) -> Enum8 {
        self.parent.as_ref().v
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.child.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.parent.get_size()
    }
}
impl ChildBuilder {
    pub fn build(self) -> Child {
        let child = Arc::new(ChildData {});
        let parent = Arc::new(ParentData {
            v: Enum8::A,
            child: ParentDataChild::None,
        });
        Child::new(parent).unwrap()
    }
}
impl From<ChildBuilder> for Parent {
    fn from(builder: ChildBuilder) -> Parent {
        builder.build().into()
    }
}
impl From<ChildBuilder> for Child {
    fn from(builder: ChildBuilder) -> Child {
        builder.build().into()
    }
}
