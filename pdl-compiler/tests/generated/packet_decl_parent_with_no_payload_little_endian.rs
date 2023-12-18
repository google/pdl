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
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum Enum8 {
    A = 0x0,
}
impl TryFrom<u8> for Enum8 {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
    Child(ChildData),
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
    parent: ParentData,
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
    fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        if bytes.get().remaining() < 1 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Parent",
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let v = Enum8::try_from(bytes.get_mut().get_u8())
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Parent",
                field: "v",
                value: unknown_val as u64,
                type_: "Enum8",
            })?;
        let payload: &[u8] = &[];
        let child = match (v) {
            (Enum8::A) if ChildData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = ChildData::parse_inner(&mut cell)?;
                ParentDataChild::Child(child_data)
            }
            _ if !payload.is_empty() => {
                ParentDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => ParentDataChild::None,
        };
        Ok(Self { v, child })
    }
    fn write_to<T: BufMut>(&self, buffer: &mut T) -> Result<(), EncodeError> {
        buffer.put_u8(u8::from(self.v));
        Ok(())
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for Parent {
    fn encoded_len(&self) -> usize {
        self.get_size()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        self.parent.write_to(buf)
    }
}
impl TryFrom<Parent> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: Parent) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<Parent> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: Parent) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl Parent {
    pub fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        let data = ParentData::parse_inner(&mut bytes)?;
        Self::new(data)
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
    fn new(parent: ParentData) -> Result<Self, DecodeError> {
        Ok(Self { parent })
    }
    pub fn get_v(&self) -> Enum8 {
        self.parent.v
    }
    fn write_to(&self, buffer: &mut impl BufMut) -> Result<(), EncodeError> {
        self.parent.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.parent.get_size()
    }
}
impl ParentBuilder {
    pub fn build(self) -> Parent {
        let parent = ParentData {
            v: self.v,
            child: ParentDataChild::None,
        };
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
    parent: ParentData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    child: ChildData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChildBuilder {}
impl ChildData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
    fn write_to<T: BufMut>(&self, buffer: &mut T) -> Result<(), EncodeError> {
        Ok(())
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        0
    }
}
impl Packet for Child {
    fn encoded_len(&self) -> usize {
        self.get_size()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        self.parent.write_to(buf)
    }
}
impl TryFrom<Child> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: Child) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<Child> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: Child) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl From<Child> for Parent {
    fn from(packet: Child) -> Parent {
        Parent::new(packet.parent).unwrap()
    }
}
impl TryFrom<Parent> for Child {
    type Error = DecodeError;
    fn try_from(packet: Parent) -> Result<Child, Self::Error> {
        Child::new(packet.parent)
    }
}
impl Child {
    pub fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        let data = ParentData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(parent: ParentData) -> Result<Self, DecodeError> {
        let child = match &parent.child {
            ParentDataChild::Child(value) => value.clone(),
            _ => {
                return Err(DecodeError::InvalidChildError {
                    expected: stringify!(ParentDataChild::Child),
                    actual: format!("{:?}", & parent.child),
                });
            }
        };
        Ok(Self { parent, child })
    }
    pub fn get_v(&self) -> Enum8 {
        self.parent.v
    }
    fn write_to(&self, buffer: &mut impl BufMut) -> Result<(), EncodeError> {
        self.child.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.parent.get_size()
    }
}
impl ChildBuilder {
    pub fn build(self) -> Child {
        let child = ChildData {};
        let parent = ParentData {
            v: Enum8::A,
            child: ParentDataChild::None,
        };
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
