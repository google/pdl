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
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum Foo {
    FooBar = 0x1,
    Baz = 0x2,
}
impl TryFrom<u8> for Foo {
    type Error = u8;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Foo::FooBar),
            0x2 => Ok(Foo::Baz),
            _ => Err(value),
        }
    }
}
impl From<&Foo> for u8 {
    fn from(value: &Foo) -> Self {
        match value {
            Foo::FooBar => 0x1,
            Foo::Baz => 0x2,
        }
    }
}
impl From<Foo> for u8 {
    fn from(value: Foo) -> Self {
        (&value).into()
    }
}
impl From<Foo> for i16 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
impl From<Foo> for i32 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
impl From<Foo> for i64 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
impl From<Foo> for u16 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
impl From<Foo> for u32 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
impl From<Foo> for u64 {
    fn from(value: Foo) -> Self {
        u8::from(value) as Self
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BarData {
    x: [Foo; 3],
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
    pub x: [Foo; 3],
}
impl BarData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 3
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "Bar".to_string(),
                wanted: 3,
                got: bytes.get().remaining(),
            });
        }
        let x = (0..3)
            .map(|_| {
                Foo::try_from(bytes.get_mut().get_u8())
                    .map_err(|unknown_val| Error::InvalidEnumValueError {
                        obj: "Bar".to_string(),
                        field: String::new(),
                        value: unknown_val as u64,
                        type_: "Foo".to_string(),
                    })
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| Error::InvalidPacketError)?;
        Ok(Self { x })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        for elem in &self.x {
            buffer.put_u8(u8::from(elem));
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        3
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
    pub fn get_x(&self) -> &[Foo; 3] {
        &self.bar.x
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
