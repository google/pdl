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
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u16", into = "u16"))]
pub enum Enum16 {
    A = 0x1,
    B = 0x2,
}
impl TryFrom<u16> for Enum16 {
    type Error = u16;
    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Enum16::A),
            0x2 => Ok(Enum16::B),
            _ => Err(value),
        }
    }
}
impl From<&Enum16> for u16 {
    fn from(value: &Enum16) -> Self {
        match value {
            Enum16::A => 0x1,
            Enum16::B => 0x2,
        }
    }
}
impl From<Enum16> for u16 {
    fn from(value: Enum16) -> Self {
        (&value).into()
    }
}
impl From<Enum16> for i32 {
    fn from(value: Enum16) -> Self {
        u16::from(value) as Self
    }
}
impl From<Enum16> for i64 {
    fn from(value: Enum16) -> Self {
        u16::from(value) as Self
    }
}
impl From<Enum16> for u32 {
    fn from(value: Enum16) -> Self {
        u16::from(value) as Self
    }
}
impl From<Enum16> for u64 {
    fn from(value: Enum16) -> Self {
        u16::from(value) as Self
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
    foo: Enum16,
    bar: Enum16,
    baz: Enum16,
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
    pub bar: Enum16,
    pub baz: Enum16,
    pub foo: Enum16,
    pub payload: Option<Bytes>,
}
impl ParentData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 7
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Parent".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let foo = Enum16::try_from(bytes.get_mut().get_u16_le())
            .map_err(|unknown_val| Error::InvalidEnumValueError {
                obj: "Parent".to_string(),
                field: "foo".to_string(),
                value: unknown_val as u64,
                type_: "Enum16".to_string(),
            })?;
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Parent".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let bar = Enum16::try_from(bytes.get_mut().get_u16_le())
            .map_err(|unknown_val| Error::InvalidEnumValueError {
                obj: "Parent".to_string(),
                field: "bar".to_string(),
                value: unknown_val as u64,
                type_: "Enum16".to_string(),
            })?;
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Parent".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let baz = Enum16::try_from(bytes.get_mut().get_u16_le())
            .map_err(|unknown_val| Error::InvalidEnumValueError {
                obj: "Parent".to_string(),
                field: "baz".to_string(),
                value: unknown_val as u64,
                type_: "Enum16".to_string(),
            })?;
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "Parent".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let payload_size = bytes.get_mut().get_u8() as usize;
        if bytes.get().remaining() < payload_size {
            return Err(Error::InvalidLengthError {
                obj: "Parent".to_string(),
                wanted: payload_size,
                got: bytes.get().remaining(),
            });
        }
        let payload = &bytes.get()[..payload_size];
        bytes.get_mut().advance(payload_size);
        let child = match (foo) {
            (Enum16::A) if ChildData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = ChildData::parse_inner(&mut cell, bar, baz)?;
                ParentDataChild::Child(child_data)
            }
            _ if !payload.is_empty() => {
                ParentDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => ParentDataChild::None,
        };
        Ok(Self { foo, bar, baz, child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u16_le(u16::from(self.foo));
        buffer.put_u16_le(u16::from(self.bar));
        buffer.put_u16_le(u16::from(self.baz));
        if self.child.get_total_size() > 0xff {
            panic!(
                "Invalid length for {}::{}: {} > {}", "Parent", "_payload_", self.child
                .get_total_size(), 0xff
            );
        }
        buffer.put_u8(self.child.get_total_size() as u8);
        match &self.child {
            ParentDataChild::Child(child) => child.write_to(buffer),
            ParentDataChild::Payload(payload) => buffer.put_slice(payload),
            ParentDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        7 + self.child.get_total_size()
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
    fn new(parent: ParentData) -> Result<Self> {
        Ok(Self { parent })
    }
    pub fn get_bar(&self) -> Enum16 {
        self.parent.bar
    }
    pub fn get_baz(&self) -> Enum16 {
        self.parent.baz
    }
    pub fn get_foo(&self) -> Enum16 {
        self.parent.foo
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
        let parent = ParentData {
            bar: self.bar,
            baz: self.baz,
            foo: self.foo,
            child: match self.payload {
                None => ParentDataChild::None,
                Some(bytes) => ParentDataChild::Payload(bytes),
            },
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
pub enum ChildDataChild {
    GrandChild(GrandChildData),
    Payload(Bytes),
    None,
}
impl ChildDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            ChildDataChild::GrandChild(value) => value.get_total_size(),
            ChildDataChild::Payload(bytes) => bytes.len(),
            ChildDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChildChild {
    GrandChild(GrandChild),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChildData {
    quux: Enum16,
    child: ChildDataChild,
}
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
pub struct ChildBuilder {
    pub bar: Enum16,
    pub baz: Enum16,
    pub quux: Enum16,
    pub payload: Option<Bytes>,
}
impl ChildData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    fn parse(bytes: &[u8], bar: Enum16, baz: Enum16) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, bar, baz)?;
        Ok(packet)
    }
    fn parse_inner(
        mut bytes: &mut Cell<&[u8]>,
        bar: Enum16,
        baz: Enum16,
    ) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Child".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let quux = Enum16::try_from(bytes.get_mut().get_u16_le())
            .map_err(|unknown_val| Error::InvalidEnumValueError {
                obj: "Child".to_string(),
                field: "quux".to_string(),
                value: unknown_val as u64,
                type_: "Enum16".to_string(),
            })?;
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (bar, quux) {
            (Enum16::A, Enum16::A) if GrandChildData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = GrandChildData::parse_inner(&mut cell, baz)?;
                ChildDataChild::GrandChild(child_data)
            }
            _ if !payload.is_empty() => {
                ChildDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => ChildDataChild::None,
        };
        Ok(Self { quux, child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u16_le(u16::from(self.quux));
        match &self.child {
            ChildDataChild::GrandChild(child) => child.write_to(buffer),
            ChildDataChild::Payload(payload) => buffer.put_slice(payload),
            ChildDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2 + self.child.get_total_size()
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
        Self::new(data)
    }
    pub fn specialize(&self) -> ChildChild {
        match &self.child.child {
            ChildDataChild::GrandChild(_) => {
                ChildChild::GrandChild(GrandChild::new(self.parent.clone()).unwrap())
            }
            ChildDataChild::Payload(payload) => ChildChild::Payload(payload.clone()),
            ChildDataChild::None => ChildChild::None,
        }
    }
    fn new(parent: ParentData) -> Result<Self> {
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
    pub fn get_bar(&self) -> Enum16 {
        self.parent.bar
    }
    pub fn get_baz(&self) -> Enum16 {
        self.parent.baz
    }
    pub fn get_foo(&self) -> Enum16 {
        self.parent.foo
    }
    pub fn get_quux(&self) -> Enum16 {
        self.child.quux
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
        let child = ChildData {
            quux: self.quux,
            child: match self.payload {
                None => ChildDataChild::None,
                Some(bytes) => ChildDataChild::Payload(bytes),
            },
        };
        let parent = ParentData {
            bar: self.bar,
            baz: self.baz,
            foo: Enum16::A,
            child: ParentDataChild::Child(child),
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrandChildDataChild {
    GrandGrandChild(GrandGrandChildData),
    Payload(Bytes),
    None,
}
impl GrandChildDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            GrandChildDataChild::GrandGrandChild(value) => value.get_total_size(),
            GrandChildDataChild::Payload(bytes) => bytes.len(),
            GrandChildDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrandChildChild {
    GrandGrandChild(GrandGrandChild),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrandChildData {
    child: GrandChildDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrandChild {
    #[cfg_attr(feature = "serde", serde(flatten))]
    parent: ParentData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    child: ChildData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    grandchild: GrandChildData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrandChildBuilder {
    pub baz: Enum16,
    pub payload: Option<Bytes>,
}
impl GrandChildData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8], baz: Enum16) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell, baz)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>, baz: Enum16) -> Result<Self> {
        let payload = bytes.get();
        bytes.get_mut().advance(payload.len());
        let child = match (baz) {
            (Enum16::A) if GrandGrandChildData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = GrandGrandChildData::parse_inner(&mut cell)?;
                GrandChildDataChild::GrandGrandChild(child_data)
            }
            _ if !payload.is_empty() => {
                GrandChildDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => GrandChildDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            GrandChildDataChild::GrandGrandChild(child) => child.write_to(buffer),
            GrandChildDataChild::Payload(payload) => buffer.put_slice(payload),
            GrandChildDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for GrandChild {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.parent.get_size());
        self.parent.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GrandChild> for Bytes {
    fn from(packet: GrandChild) -> Self {
        packet.to_bytes()
    }
}
impl From<GrandChild> for Vec<u8> {
    fn from(packet: GrandChild) -> Self {
        packet.to_vec()
    }
}
impl From<GrandChild> for Parent {
    fn from(packet: GrandChild) -> Parent {
        Parent::new(packet.parent).unwrap()
    }
}
impl From<GrandChild> for Child {
    fn from(packet: GrandChild) -> Child {
        Child::new(packet.parent).unwrap()
    }
}
impl TryFrom<Parent> for GrandChild {
    type Error = Error;
    fn try_from(packet: Parent) -> Result<GrandChild> {
        GrandChild::new(packet.parent)
    }
}
impl GrandChild {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = ParentData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> GrandChildChild {
        match &self.grandchild.child {
            GrandChildDataChild::GrandGrandChild(_) => {
                GrandChildChild::GrandGrandChild(
                    GrandGrandChild::new(self.parent.clone()).unwrap(),
                )
            }
            GrandChildDataChild::Payload(payload) => {
                GrandChildChild::Payload(payload.clone())
            }
            GrandChildDataChild::None => GrandChildChild::None,
        }
    }
    fn new(parent: ParentData) -> Result<Self> {
        let child = match &parent.child {
            ParentDataChild::Child(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(ParentDataChild::Child),
                    actual: format!("{:?}", & parent.child),
                });
            }
        };
        let grandchild = match &child.child {
            ChildDataChild::GrandChild(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(ChildDataChild::GrandChild),
                    actual: format!("{:?}", & child.child),
                });
            }
        };
        Ok(Self { parent, child, grandchild })
    }
    pub fn get_bar(&self) -> Enum16 {
        self.parent.bar
    }
    pub fn get_baz(&self) -> Enum16 {
        self.parent.baz
    }
    pub fn get_foo(&self) -> Enum16 {
        self.parent.foo
    }
    pub fn get_quux(&self) -> Enum16 {
        self.child.quux
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.grandchild.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.parent.get_size()
    }
}
impl GrandChildBuilder {
    pub fn build(self) -> GrandChild {
        let grandchild = GrandChildData {
            child: match self.payload {
                None => GrandChildDataChild::None,
                Some(bytes) => GrandChildDataChild::Payload(bytes),
            },
        };
        let child = ChildData {
            quux: Enum16::A,
            child: ChildDataChild::GrandChild(grandchild),
        };
        let parent = ParentData {
            bar: Enum16::A,
            baz: self.baz,
            foo: Enum16::A,
            child: ParentDataChild::Child(child),
        };
        GrandChild::new(parent).unwrap()
    }
}
impl From<GrandChildBuilder> for Parent {
    fn from(builder: GrandChildBuilder) -> Parent {
        builder.build().into()
    }
}
impl From<GrandChildBuilder> for Child {
    fn from(builder: GrandChildBuilder) -> Child {
        builder.build().into()
    }
}
impl From<GrandChildBuilder> for GrandChild {
    fn from(builder: GrandChildBuilder) -> GrandChild {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrandGrandChildDataChild {
    Payload(Bytes),
    None,
}
impl GrandGrandChildDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            GrandGrandChildDataChild::Payload(bytes) => bytes.len(),
            GrandGrandChildDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrandGrandChildChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrandGrandChildData {
    child: GrandGrandChildDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrandGrandChild {
    #[cfg_attr(feature = "serde", serde(flatten))]
    parent: ParentData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    child: ChildData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    grandchild: GrandChildData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    grandgrandchild: GrandGrandChildData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrandGrandChildBuilder {
    pub payload: Option<Bytes>,
}
impl GrandGrandChildData {
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
                GrandGrandChildDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => GrandGrandChildDataChild::None,
        };
        Ok(Self { child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            GrandGrandChildDataChild::Payload(payload) => buffer.put_slice(payload),
            GrandGrandChildDataChild::None => {}
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        self.child.get_total_size()
    }
}
impl Packet for GrandGrandChild {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.parent.get_size());
        self.parent.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<GrandGrandChild> for Bytes {
    fn from(packet: GrandGrandChild) -> Self {
        packet.to_bytes()
    }
}
impl From<GrandGrandChild> for Vec<u8> {
    fn from(packet: GrandGrandChild) -> Self {
        packet.to_vec()
    }
}
impl From<GrandGrandChild> for Parent {
    fn from(packet: GrandGrandChild) -> Parent {
        Parent::new(packet.parent).unwrap()
    }
}
impl From<GrandGrandChild> for Child {
    fn from(packet: GrandGrandChild) -> Child {
        Child::new(packet.parent).unwrap()
    }
}
impl From<GrandGrandChild> for GrandChild {
    fn from(packet: GrandGrandChild) -> GrandChild {
        GrandChild::new(packet.parent).unwrap()
    }
}
impl TryFrom<Parent> for GrandGrandChild {
    type Error = Error;
    fn try_from(packet: Parent) -> Result<GrandGrandChild> {
        GrandGrandChild::new(packet.parent)
    }
}
impl GrandGrandChild {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = ParentData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> GrandGrandChildChild {
        match &self.grandgrandchild.child {
            GrandGrandChildDataChild::Payload(payload) => {
                GrandGrandChildChild::Payload(payload.clone())
            }
            GrandGrandChildDataChild::None => GrandGrandChildChild::None,
        }
    }
    fn new(parent: ParentData) -> Result<Self> {
        let child = match &parent.child {
            ParentDataChild::Child(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(ParentDataChild::Child),
                    actual: format!("{:?}", & parent.child),
                });
            }
        };
        let grandchild = match &child.child {
            ChildDataChild::GrandChild(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(ChildDataChild::GrandChild),
                    actual: format!("{:?}", & child.child),
                });
            }
        };
        let grandgrandchild = match &grandchild.child {
            GrandChildDataChild::GrandGrandChild(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(GrandChildDataChild::GrandGrandChild),
                    actual: format!("{:?}", & grandchild.child),
                });
            }
        };
        Ok(Self {
            parent,
            child,
            grandchild,
            grandgrandchild,
        })
    }
    pub fn get_bar(&self) -> Enum16 {
        self.parent.bar
    }
    pub fn get_baz(&self) -> Enum16 {
        self.parent.baz
    }
    pub fn get_foo(&self) -> Enum16 {
        self.parent.foo
    }
    pub fn get_quux(&self) -> Enum16 {
        self.child.quux
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.grandgrandchild.child {
            GrandGrandChildDataChild::Payload(bytes) => &bytes,
            GrandGrandChildDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.grandgrandchild.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.parent.get_size()
    }
}
impl GrandGrandChildBuilder {
    pub fn build(self) -> GrandGrandChild {
        let grandgrandchild = GrandGrandChildData {
            child: match self.payload {
                None => GrandGrandChildDataChild::None,
                Some(bytes) => GrandGrandChildDataChild::Payload(bytes),
            },
        };
        let grandchild = GrandChildData {
            child: GrandChildDataChild::GrandGrandChild(grandgrandchild),
        };
        let child = ChildData {
            quux: Enum16::A,
            child: ChildDataChild::GrandChild(grandchild),
        };
        let parent = ParentData {
            bar: Enum16::A,
            baz: Enum16::A,
            foo: Enum16::A,
            child: ParentDataChild::Child(child),
        };
        GrandGrandChild::new(parent).unwrap()
    }
}
impl From<GrandGrandChildBuilder> for Parent {
    fn from(builder: GrandGrandChildBuilder) -> Parent {
        builder.build().into()
    }
}
impl From<GrandGrandChildBuilder> for Child {
    fn from(builder: GrandGrandChildBuilder) -> Child {
        builder.build().into()
    }
}
impl From<GrandGrandChildBuilder> for GrandChild {
    fn from(builder: GrandGrandChildBuilder) -> GrandChild {
        builder.build().into()
    }
}
impl From<GrandGrandChildBuilder> for GrandGrandChild {
    fn from(builder: GrandGrandChildBuilder) -> GrandGrandChild {
        builder.build().into()
    }
}
