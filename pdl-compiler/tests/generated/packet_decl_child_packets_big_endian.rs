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
pub enum FooDataChild {
    Bar(BarData),
    Baz(BazData),
    Payload(Bytes),
    None,
}
impl FooDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            FooDataChild::Bar(value) => value.get_total_size(),
            FooDataChild::Baz(value) => value.get_total_size(),
            FooDataChild::Payload(bytes) => bytes.len(),
            FooDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FooChild {
    Bar(Bar),
    Baz(Baz),
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooData {
    a: u8,
    b: Enum16,
    child: FooDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    #[cfg_attr(feature = "serde", serde(flatten))]
    foo: FooData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooBuilder {
    pub a: u8,
    pub b: Enum16,
    pub payload: Option<Bytes>,
}
impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 4
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let a = bytes.get_mut().get_u8();
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let b = Enum16::try_from(bytes.get_mut().get_u16())
            .map_err(|unknown_val| Error::InvalidEnumValueError {
                obj: "Foo".to_string(),
                field: "b".to_string(),
                value: unknown_val as u64,
                type_: "Enum16".to_string(),
            })?;
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
        let child = match (a, b) {
            (100, _) if BarData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = BarData::parse_inner(&mut cell)?;
                FooDataChild::Bar(child_data)
            }
            (_, Enum16::B) if BazData::conforms(&payload) => {
                let mut cell = Cell::new(payload);
                let child_data = BazData::parse_inner(&mut cell)?;
                FooDataChild::Baz(child_data)
            }
            _ if !payload.is_empty() => {
                FooDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => FooDataChild::None,
        };
        Ok(Self { a, b, child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.a);
        buffer.put_u16(u16::from(self.b));
        if self.child.get_total_size() > 0xff {
            panic!(
                "Invalid length for {}::{}: {} > {}", "Foo", "_payload_", self.child
                .get_total_size(), 0xff
            );
        }
        buffer.put_u8(self.child.get_total_size() as u8);
        match &self.child {
            FooDataChild::Bar(child) => child.write_to(buffer),
            FooDataChild::Baz(child) => child.write_to(buffer),
            FooDataChild::Payload(payload) => buffer.put_slice(payload),
            FooDataChild::None => {}
        }
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
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = FooData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> FooChild {
        match &self.foo.child {
            FooDataChild::Bar(_) => FooChild::Bar(Bar::new(self.foo.clone()).unwrap()),
            FooDataChild::Baz(_) => FooChild::Baz(Baz::new(self.foo.clone()).unwrap()),
            FooDataChild::Payload(payload) => FooChild::Payload(payload.clone()),
            FooDataChild::None => FooChild::None,
        }
    }
    fn new(foo: FooData) -> Result<Self> {
        Ok(Self { foo })
    }
    pub fn get_a(&self) -> u8 {
        self.foo.a
    }
    pub fn get_b(&self) -> Enum16 {
        self.foo.b
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
        let foo = FooData {
            a: self.a,
            b: self.b,
            child: match self.payload {
                None => FooDataChild::None,
                Some(bytes) => FooDataChild::Payload(bytes),
            },
        };
        Foo::new(foo).unwrap()
    }
}
impl From<FooBuilder> for Foo {
    fn from(builder: FooBuilder) -> Foo {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarData {
    x: u8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar {
    #[cfg_attr(feature = "serde", serde(flatten))]
    foo: FooData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    bar: BarData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarBuilder {
    pub b: Enum16,
    pub x: u8,
}
impl BarData {
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
                obj: "Bar".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let x = bytes.get_mut().get_u8();
        Ok(Self { x })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u8(self.x);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1
    }
}
impl Packet for Bar {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.foo.get_size());
        self.foo.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Bar> for Bytes {
    fn from(packet: Bar) -> Self {
        packet.to_bytes()
    }
}
impl From<Bar> for Vec<u8> {
    fn from(packet: Bar) -> Self {
        packet.to_vec()
    }
}
impl From<Bar> for Foo {
    fn from(packet: Bar) -> Foo {
        Foo::new(packet.foo).unwrap()
    }
}
impl TryFrom<Foo> for Bar {
    type Error = Error;
    fn try_from(packet: Foo) -> Result<Bar> {
        Bar::new(packet.foo)
    }
}
impl Bar {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = FooData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(foo: FooData) -> Result<Self> {
        let bar = match &foo.child {
            FooDataChild::Bar(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(FooDataChild::Bar),
                    actual: format!("{:?}", & foo.child),
                });
            }
        };
        Ok(Self { foo, bar })
    }
    pub fn get_a(&self) -> u8 {
        self.foo.a
    }
    pub fn get_b(&self) -> Enum16 {
        self.foo.b
    }
    pub fn get_x(&self) -> u8 {
        self.bar.x
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.bar.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.foo.get_size()
    }
}
impl BarBuilder {
    pub fn build(self) -> Bar {
        let bar = BarData { x: self.x };
        let foo = FooData {
            a: 100,
            b: self.b,
            child: FooDataChild::Bar(bar),
        };
        Bar::new(foo).unwrap()
    }
}
impl From<BarBuilder> for Foo {
    fn from(builder: BarBuilder) -> Foo {
        builder.build().into()
    }
}
impl From<BarBuilder> for Bar {
    fn from(builder: BarBuilder) -> Bar {
        builder.build().into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BazData {
    y: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Baz {
    #[cfg_attr(feature = "serde", serde(flatten))]
    foo: FooData,
    #[cfg_attr(feature = "serde", serde(flatten))]
    baz: BazData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BazBuilder {
    pub a: u8,
    pub y: u16,
}
impl BazData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 2
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Baz".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let y = bytes.get_mut().get_u16();
        Ok(Self { y })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u16(self.y);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        2
    }
}
impl Packet for Baz {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.foo.get_size());
        self.foo.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Baz> for Bytes {
    fn from(packet: Baz) -> Self {
        packet.to_bytes()
    }
}
impl From<Baz> for Vec<u8> {
    fn from(packet: Baz) -> Self {
        packet.to_vec()
    }
}
impl From<Baz> for Foo {
    fn from(packet: Baz) -> Foo {
        Foo::new(packet.foo).unwrap()
    }
}
impl TryFrom<Foo> for Baz {
    type Error = Error;
    fn try_from(packet: Foo) -> Result<Baz> {
        Baz::new(packet.foo)
    }
}
impl Baz {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = FooData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(foo: FooData) -> Result<Self> {
        let baz = match &foo.child {
            FooDataChild::Baz(value) => value.clone(),
            _ => {
                return Err(Error::InvalidChildError {
                    expected: stringify!(FooDataChild::Baz),
                    actual: format!("{:?}", & foo.child),
                });
            }
        };
        Ok(Self { foo, baz })
    }
    pub fn get_a(&self) -> u8 {
        self.foo.a
    }
    pub fn get_b(&self) -> Enum16 {
        self.foo.b
    }
    pub fn get_y(&self) -> u16 {
        self.baz.y
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.baz.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.foo.get_size()
    }
}
impl BazBuilder {
    pub fn build(self) -> Baz {
        let baz = BazData { y: self.y };
        let foo = FooData {
            a: self.a,
            b: Enum16::B,
            child: FooDataChild::Baz(baz),
        };
        Baz::new(foo).unwrap()
    }
}
impl From<BazBuilder> for Foo {
    fn from(builder: BazBuilder) -> Foo {
        builder.build().into()
    }
}
impl From<BazBuilder> for Baz {
    fn from(builder: BazBuilder) -> Baz {
        builder.build().into()
    }
}
