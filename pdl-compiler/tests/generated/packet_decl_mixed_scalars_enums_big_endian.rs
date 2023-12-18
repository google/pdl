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
pub enum Enum7 {
    A = 0x1,
    B = 0x2,
}
impl TryFrom<u8> for Enum7 {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Enum7::A),
            0x2 => Ok(Enum7::B),
            _ => Err(value),
        }
    }
}
impl From<&Enum7> for u8 {
    fn from(value: &Enum7) -> Self {
        match value {
            Enum7::A => 0x1,
            Enum7::B => 0x2,
        }
    }
}
impl From<Enum7> for u8 {
    fn from(value: Enum7) -> Self {
        (&value).into()
    }
}
impl From<Enum7> for i8 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for i16 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for i32 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for i64 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for u16 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for u32 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
impl From<Enum7> for u64 {
    fn from(value: Enum7) -> Self {
        u8::from(value) as Self
    }
}
#[repr(u64)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u16", into = "u16"))]
pub enum Enum9 {
    A = 0x1,
    B = 0x2,
}
impl TryFrom<u16> for Enum9 {
    type Error = u16;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Enum9::A),
            0x2 => Ok(Enum9::B),
            _ => Err(value),
        }
    }
}
impl From<&Enum9> for u16 {
    fn from(value: &Enum9) -> Self {
        match value {
            Enum9::A => 0x1,
            Enum9::B => 0x2,
        }
    }
}
impl From<Enum9> for u16 {
    fn from(value: Enum9) -> Self {
        (&value).into()
    }
}
impl From<Enum9> for i16 {
    fn from(value: Enum9) -> Self {
        u16::from(value) as Self
    }
}
impl From<Enum9> for i32 {
    fn from(value: Enum9) -> Self {
        u16::from(value) as Self
    }
}
impl From<Enum9> for i64 {
    fn from(value: Enum9) -> Self {
        u16::from(value) as Self
    }
}
impl From<Enum9> for u32 {
    fn from(value: Enum9) -> Self {
        u16::from(value) as Self
    }
}
impl From<Enum9> for u64 {
    fn from(value: Enum9) -> Self {
        u16::from(value) as Self
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooData {
    x: Enum7,
    y: u8,
    z: Enum9,
    w: u8,
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
    pub w: u8,
    pub x: Enum7,
    pub y: u8,
    pub z: Enum9,
}
impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 3
    }
    fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        if bytes.get().remaining() < 3 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 3,
                got: bytes.get().remaining(),
            });
        }
        let chunk = bytes.get_mut().get_uint(3) as u32;
        let x = Enum7::try_from((chunk & 0x7f) as u8)
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Foo",
                field: "x",
                value: unknown_val as u64,
                type_: "Enum7",
            })?;
        let y = ((chunk >> 7) & 0x1f) as u8;
        let z = Enum9::try_from(((chunk >> 12) & 0x1ff) as u16)
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Foo",
                field: "z",
                value: unknown_val as u64,
                type_: "Enum9",
            })?;
        let w = ((chunk >> 21) & 0x7) as u8;
        Ok(Self { x, y, z, w })
    }
    fn write_to<T: BufMut>(&self, buffer: &mut T) -> Result<(), EncodeError> {
        if self.y > 0x1f {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "y",
                value: self.y as u64,
                maximum_value: 0x1f,
            });
        }
        if self.w > 0x7 {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "w",
                value: self.w as u64,
                maximum_value: 0x7,
            });
        }
        let value = (u8::from(self.x) as u32) | ((self.y as u32) << 7)
            | ((u16::from(self.z) as u32) << 12) | ((self.w as u32) << 21);
        buffer.put_uint(value as u64, 3);
        Ok(())
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        3
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        self.get_size()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        self.foo.write_to(buf)
    }
}
impl TryFrom<Foo> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: Foo) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<Foo> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: Foo) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl Foo {
    pub fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self, DecodeError> {
        let data = FooData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(foo: FooData) -> Result<Self, DecodeError> {
        Ok(Self { foo })
    }
    pub fn get_w(&self) -> u8 {
        self.foo.w
    }
    pub fn get_x(&self) -> Enum7 {
        self.foo.x
    }
    pub fn get_y(&self) -> u8 {
        self.foo.y
    }
    pub fn get_z(&self) -> Enum9 {
        self.foo.z
    }
    fn write_to(&self, buffer: &mut impl BufMut) -> Result<(), EncodeError> {
        self.foo.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.foo.get_size()
    }
}
impl FooBuilder {
    pub fn build(self) -> Foo {
        let foo = FooData {
            w: self.w,
            x: self.x,
            y: self.y,
            z: self.z,
        };
        Foo::new(foo).unwrap()
    }
}
impl From<FooBuilder> for Foo {
    fn from(builder: FooBuilder) -> Foo {
        builder.build().into()
    }
}
