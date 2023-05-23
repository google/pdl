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
    B = 0x1,
    C = 0x2,
}
impl TryFrom<u8> for Enum8 {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Enum8::A),
            0x1 => Ok(Enum8::B),
            0x2 => Ok(Enum8::C),
            _ => Err(value),
        }
    }
}
impl From<&Enum8> for u8 {
    fn from(value: &Enum8) -> Self {
        match value {
            Enum8::A => 0x0,
            Enum8::B => 0x1,
            Enum8::C => 0x2,
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
    AliasChild(Arc<AliasChildData>),
    NormalChild(Arc<NormalChildData>),
    Payload(Bytes),
    None,
}
impl ParentDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            ParentDataChild::AliasChild(value) => value.get_total_size(),
            ParentDataChild::NormalChild(value) => value.get_total_size(),
            ParentDataChild::Payload(bytes) => bytes.len(),
            ParentDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParentChild {
    AliasChild(AliasChild),
    NormalChild(NormalChild),
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
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (v) {
            (Enum8::B | Enum8::C) if AliasChildData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = AliasChildData::parse_inner(&mut cell, v)?;
                ParentDataChild::AliasChild(Arc::new(child_data))
            }
            (Enum8::A) if NormalChildData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = NormalChildData::parse_inner(&mut cell)?;
                ParentDataChild::NormalChild(Arc::new(child_data))
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
        match &self.child {
            ParentDataChild::AliasChild(child) => child.write_to(buffer),
            ParentDataChild::NormalChild(child) => child.write_to(buffer),
            ParentDataChild::Payload(payload) => buffer.put_slice(payload),
            ParentDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self.child.get_total_size()
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
            ParentDataChild::AliasChild(_) => {
                ParentChild::AliasChild(AliasChild::new(self.parent.clone()).unwrap())
            }
            ParentDataChild::NormalChild(_) => {
                ParentChild::NormalChild(NormalChild::new(self.parent.clone()).unwrap())
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
            child: match self.payload {
                None => ParentDataChild::None,
                Some(bytes) => ParentDataChild::Payload(bytes),
            },
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
pub enum AliasChildDataChild {
    NormalGrandChild1(Arc<NormalGrandChild1Data>),
    NormalGrandChild2(Arc<NormalGrandChild2Data>),
    Payload(Bytes),
    None,
}
impl AliasChildDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            AliasChildDataChild::NormalGrandChild1(value) => value.get_total_size(),
            AliasChildDataChild::NormalGrandChild2(value) => value.get_total_size(),
            AliasChildDataChild::Payload(bytes) => bytes.len(),
            AliasChildDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AliasChildChild {
    NormalGrandChild1(NormalGrandChild1),
    NormalGrandChild2(NormalGrandChild2),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AliasChildData {
    child: AliasChildDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AliasChild {
    #[cfg_attr(feature = "serde", serde(flatten))]
    parent: Arc<ParentData>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    aliaschild: Arc<AliasChildData>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AliasChildBuilder {
    pub v: Enum8,
    pub payload: Option<Bytes>,
}
impl AliasChildData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], v: Enum8) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, v)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, v: Enum8) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (v) {
            (Enum8::B) if NormalGrandChild1Data::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = NormalGrandChild1Data::parse_inner(&mut cell)?;
                AliasChildDataChild::NormalGrandChild1(Arc::new(child_data))
            }
            (Enum8::C) if NormalGrandChild2Data::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = NormalGrandChild2Data::parse_inner(&mut cell)?;
                AliasChildDataChild::NormalGrandChild2(Arc::new(child_data))
            }
            _ if !payload.is_empty() => {
                AliasChildDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => AliasChildDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            AliasChildDataChild::NormalGrandChild1(child) => child.write_to(buffer),
            AliasChildDataChild::NormalGrandChild2(child) => child.write_to(buffer),
            AliasChildDataChild::Payload(payload) => buffer.put_slice(payload),
            AliasChildDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for AliasChild {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.parent.get_size());
        self.parent.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<AliasChild> for Bytes {
    fn from(packet: AliasChild) -> Self {
        packet.to_bytes()
    }
}
impl From<AliasChild> for Vec<u8> {
    fn from(packet: AliasChild) -> Self {
        packet.to_vec()
    }
}
impl From<AliasChild> for Parent {
    fn from(packet: AliasChild) -> Parent {
        Parent::new(packet.parent).unwrap()
    }
}
impl TryFrom<Parent> for AliasChild {
    type Error = Error;
    fn try_from(packet: Parent) -> Result<AliasChild> {
        AliasChild::new(packet.parent)
    }
}
impl AliasChild {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = ParentData::parse_inner(&mut bytes)?;
        Self::new(Arc::new(data))
    }
    pub fn specialize(&self) -> AliasChildChild {
        match &self.aliaschild.child {
            AliasChildDataChild::NormalGrandChild1(_) => {
                AliasChildChild::NormalGrandChild1(
                    NormalGrandChild1::new(self.parent.clone()).unwrap(),
                )
            }
            AliasChildDataChild::NormalGrandChild2(_) => {
                AliasChildChild::NormalGrandChild2(
                    NormalGrandChild2::new(self.parent.clone()).unwrap(),
                )
            }
            AliasChildDataChild::Payload(payload) => {
                AliasChildChild::Payload(payload.clone())
            }
            AliasChildDataChild::None => AliasChildChild::None,
        }
    }
    fn new(parent: Arc<ParentData>) -> Result<Self> {
        let aliaschild = match &parent.child {
            ParentDataChild::AliasChild(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(ParentDataChild::AliasChild),
                    actual: format!("{:?}", & parent.child),
                });
            }
        };
        Ok(Self { parent, aliaschild })
    }
    pub fn get_v(&self) -> Enum8 {
        self.parent.as_ref().v
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.aliaschild.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.parent.get_size()
    }
}
impl AliasChildBuilder {
    pub fn build(self) -> AliasChild {
        let aliaschild = Arc::new(AliasChildData {
            child: match self.payload {
                None => AliasChildDataChild::None,
                Some(bytes) => AliasChildDataChild::Payload(bytes),
            },
        });
        let parent = Arc::new(ParentData {
            v: self.v,
            child: ParentDataChild::AliasChild(aliaschild),
        });
        AliasChild::new(parent).unwrap()
    }
}
impl From<AliasChildBuilder> for Parent {
    fn from(builder: AliasChildBuilder) -> Parent {
        builder.build().into()
    }
}
impl From<AliasChildBuilder> for AliasChild {
    fn from(builder: AliasChildBuilder) -> AliasChild {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalChildData {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalChild {
    #[cfg_attr(feature = "serde", serde(flatten))]
    parent: Arc<ParentData>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    normalchild: Arc<NormalChildData>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalChildBuilder {}
impl NormalChildData {
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
impl Packet for NormalChild {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.parent.get_size());
        self.parent.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<NormalChild> for Bytes {
    fn from(packet: NormalChild) -> Self {
        packet.to_bytes()
    }
}
impl From<NormalChild> for Vec<u8> {
    fn from(packet: NormalChild) -> Self {
        packet.to_vec()
    }
}
impl From<NormalChild> for Parent {
    fn from(packet: NormalChild) -> Parent {
        Parent::new(packet.parent).unwrap()
    }
}
impl TryFrom<Parent> for NormalChild {
    type Error = Error;
    fn try_from(packet: Parent) -> Result<NormalChild> {
        NormalChild::new(packet.parent)
    }
}
impl NormalChild {
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
        let normalchild = match &parent.child {
            ParentDataChild::NormalChild(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(ParentDataChild::NormalChild),
                    actual: format!("{:?}", & parent.child),
                });
            }
        };
        Ok(Self { parent, normalchild })
    }
    pub fn get_v(&self) -> Enum8 {
        self.parent.as_ref().v
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.normalchild.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.parent.get_size()
    }
}
impl NormalChildBuilder {
    pub fn build(self) -> NormalChild {
        let normalchild = Arc::new(NormalChildData {});
        let parent = Arc::new(ParentData {
            v: Enum8::A,
            child: ParentDataChild::NormalChild(normalchild),
        });
        NormalChild::new(parent).unwrap()
    }
}
impl From<NormalChildBuilder> for Parent {
    fn from(builder: NormalChildBuilder) -> Parent {
        builder.build().into()
    }
}
impl From<NormalChildBuilder> for NormalChild {
    fn from(builder: NormalChildBuilder) -> NormalChild {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalGrandChild1Data {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalGrandChild1 {
    #[cfg_attr(feature = "serde", serde(flatten))]
    parent: Arc<ParentData>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    aliaschild: Arc<AliasChildData>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    normalgrandchild1: Arc<NormalGrandChild1Data>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalGrandChild1Builder {}
impl NormalGrandChild1Data {
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
impl Packet for NormalGrandChild1 {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.parent.get_size());
        self.parent.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<NormalGrandChild1> for Bytes {
    fn from(packet: NormalGrandChild1) -> Self {
        packet.to_bytes()
    }
}
impl From<NormalGrandChild1> for Vec<u8> {
    fn from(packet: NormalGrandChild1) -> Self {
        packet.to_vec()
    }
}
impl From<NormalGrandChild1> for Parent {
    fn from(packet: NormalGrandChild1) -> Parent {
        Parent::new(packet.parent).unwrap()
    }
}
impl From<NormalGrandChild1> for AliasChild {
    fn from(packet: NormalGrandChild1) -> AliasChild {
        AliasChild::new(packet.parent).unwrap()
    }
}
impl TryFrom<Parent> for NormalGrandChild1 {
    type Error = Error;
    fn try_from(packet: Parent) -> Result<NormalGrandChild1> {
        NormalGrandChild1::new(packet.parent)
    }
}
impl NormalGrandChild1 {
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
        let aliaschild = match &parent.child {
            ParentDataChild::AliasChild(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(ParentDataChild::AliasChild),
                    actual: format!("{:?}", & parent.child),
                });
            }
        };
        let normalgrandchild1 = match &aliaschild.child {
            AliasChildDataChild::NormalGrandChild1(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(AliasChildDataChild::NormalGrandChild1),
                    actual: format!("{:?}", & aliaschild.child),
                });
            }
        };
        Ok(Self {
            parent,
            aliaschild,
            normalgrandchild1,
        })
    }
    pub fn get_v(&self) -> Enum8 {
        self.parent.as_ref().v
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.normalgrandchild1.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.parent.get_size()
    }
}
impl NormalGrandChild1Builder {
    pub fn build(self) -> NormalGrandChild1 {
        let normalgrandchild1 = Arc::new(NormalGrandChild1Data {});
        let aliaschild = Arc::new(AliasChildData {
            child: AliasChildDataChild::NormalGrandChild1(normalgrandchild1),
        });
        let parent = Arc::new(ParentData {
            v: Enum8::B,
            child: ParentDataChild::AliasChild(aliaschild),
        });
        NormalGrandChild1::new(parent).unwrap()
    }
}
impl From<NormalGrandChild1Builder> for Parent {
    fn from(builder: NormalGrandChild1Builder) -> Parent {
        builder.build().into()
    }
}
impl From<NormalGrandChild1Builder> for AliasChild {
    fn from(builder: NormalGrandChild1Builder) -> AliasChild {
        builder.build().into()
    }
}
impl From<NormalGrandChild1Builder> for NormalGrandChild1 {
    fn from(builder: NormalGrandChild1Builder) -> NormalGrandChild1 {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NormalGrandChild2DataChild {
    Payload(Bytes),
    None,
}
impl NormalGrandChild2DataChild {
    fn get_total_size(&self) -> usize {
        match self {
            NormalGrandChild2DataChild::Payload(bytes) => bytes.len(),
            NormalGrandChild2DataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NormalGrandChild2Child {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalGrandChild2Data {
    child: NormalGrandChild2DataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalGrandChild2 {
    #[cfg_attr(feature = "serde", serde(flatten))]
    parent: Arc<ParentData>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    aliaschild: Arc<AliasChildData>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    normalgrandchild2: Arc<NormalGrandChild2Data>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalGrandChild2Builder {
    pub payload: Option<Bytes>,
}
impl NormalGrandChild2Data {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match () {
            _ if !payload.is_empty() => {
                NormalGrandChild2DataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => NormalGrandChild2DataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            NormalGrandChild2DataChild::Payload(payload) => buffer.put_slice(payload),
            NormalGrandChild2DataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for NormalGrandChild2 {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.parent.get_size());
        self.parent.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<NormalGrandChild2> for Bytes {
    fn from(packet: NormalGrandChild2) -> Self {
        packet.to_bytes()
    }
}
impl From<NormalGrandChild2> for Vec<u8> {
    fn from(packet: NormalGrandChild2) -> Self {
        packet.to_vec()
    }
}
impl From<NormalGrandChild2> for Parent {
    fn from(packet: NormalGrandChild2) -> Parent {
        Parent::new(packet.parent).unwrap()
    }
}
impl From<NormalGrandChild2> for AliasChild {
    fn from(packet: NormalGrandChild2) -> AliasChild {
        AliasChild::new(packet.parent).unwrap()
    }
}
impl TryFrom<Parent> for NormalGrandChild2 {
    type Error = Error;
    fn try_from(packet: Parent) -> Result<NormalGrandChild2> {
        NormalGrandChild2::new(packet.parent)
    }
}
impl NormalGrandChild2 {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = ParentData::parse_inner(&mut bytes)?;
        Self::new(Arc::new(data))
    }
    pub fn specialize(&self) -> NormalGrandChild2Child {
        match &self.normalgrandchild2.child {
            NormalGrandChild2DataChild::Payload(payload) => {
                NormalGrandChild2Child::Payload(payload.clone())
            }
            NormalGrandChild2DataChild::None => NormalGrandChild2Child::None,
        }
    }
    fn new(parent: Arc<ParentData>) -> Result<Self> {
        let aliaschild = match &parent.child {
            ParentDataChild::AliasChild(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(ParentDataChild::AliasChild),
                    actual: format!("{:?}", & parent.child),
                });
            }
        };
        let normalgrandchild2 = match &aliaschild.child {
            AliasChildDataChild::NormalGrandChild2(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(AliasChildDataChild::NormalGrandChild2),
                    actual: format!("{:?}", & aliaschild.child),
                });
            }
        };
        Ok(Self {
            parent,
            aliaschild,
            normalgrandchild2,
        })
    }
    pub fn get_v(&self) -> Enum8 {
        self.parent.as_ref().v
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.normalgrandchild2.child {
            NormalGrandChild2DataChild::Payload(bytes) => &bytes,
            NormalGrandChild2DataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.normalgrandchild2.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.parent.get_size()
    }
}
impl NormalGrandChild2Builder {
    pub fn build(self) -> NormalGrandChild2 {
        let normalgrandchild2 = Arc::new(NormalGrandChild2Data {
            child: match self.payload {
                None => NormalGrandChild2DataChild::None,
                Some(bytes) => NormalGrandChild2DataChild::Payload(bytes),
            },
        });
        let aliaschild = Arc::new(AliasChildData {
            child: AliasChildDataChild::NormalGrandChild2(normalgrandchild2),
        });
        let parent = Arc::new(ParentData {
            v: Enum8::C,
            child: ParentDataChild::AliasChild(aliaschild),
        });
        NormalGrandChild2::new(parent).unwrap()
    }
}
impl From<NormalGrandChild2Builder> for Parent {
    fn from(builder: NormalGrandChild2Builder) -> Parent {
        builder.build().into()
    }
}
impl From<NormalGrandChild2Builder> for AliasChild {
    fn from(builder: NormalGrandChild2Builder) -> AliasChild {
        builder.build().into()
    }
}
impl From<NormalGrandChild2Builder> for NormalGrandChild2 {
    fn from(builder: NormalGrandChild2Builder) -> NormalGrandChild2 {
        builder.build().into()
    }
}
