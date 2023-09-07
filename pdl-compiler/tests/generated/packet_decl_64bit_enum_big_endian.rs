#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use pdl_runtime::{Error, Packet, Private};
type Result<T> = std::result::Result<T, Error>;
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u64", into = "u64"))]
pub enum Foo {
    A = 0x1,
    B = 0x2,
}
impl TryFrom<u64> for Foo {
    type Error = u64;
    fn try_from(value: u64) -> std::result::Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Foo::A),
            0x2 => Ok(Foo::B),
            _ => Err(value),
        }
    }
}
impl From<&Foo> for u64 {
    fn from(value: &Foo) -> Self {
        match value {
            Foo::A => 0x1,
            Foo::B => 0x2,
        }
    }
}
impl From<Foo> for u64 {
    fn from(value: Foo) -> Self {
        (&value).into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarData {
    x: Foo,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar {
    #[cfg_attr(feature = "serde", serde(flatten))]
    bar: BarData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarBuilder {
    pub x: Foo,
}
impl BarData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 8
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 8 {
            return Err(Error::InvalidLengthError {
                obj: "Bar".to_string(),
                wanted: 8,
                got: bytes.get().remaining(),
            });
        }
        let x = Foo::try_from(bytes.get_mut().get_u64())
            .map_err(|unknown_val| Error::InvalidEnumValueError {
                obj: "Bar".to_string(),
                field: "x".to_string(),
                value: unknown_val as u64,
                type_: "Foo".to_string(),
            })?;
        Ok(Self { x })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        buffer.put_u64(u64::from(self.x));
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        8
    }
}
impl Packet for Bar {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.bar.get_size());
        self.bar.write_to(&mut buffer);
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
impl Bar {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = BarData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(bar: BarData) -> Result<Self> {
        Ok(Self { bar })
    }
    pub fn get_x(&self) -> Foo {
        self.bar.x
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.bar.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.bar.get_size()
    }
}
impl BarBuilder {
    pub fn build(self) -> Bar {
        let bar = BarData { x: self.x };
        Bar::new(bar).unwrap()
    }
}
impl From<BarBuilder> for Bar {
    fn from(builder: BarBuilder) -> Bar {
        builder.build().into()
    }
}
