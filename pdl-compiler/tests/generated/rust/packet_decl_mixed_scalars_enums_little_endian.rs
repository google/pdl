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
pub struct Foo {
    pub x: Enum7,
    pub y: u8,
    pub z: Enum9,
    pub w: u8,
}
impl Foo {
    pub fn x(&self) -> Enum7 {
        self.x
    }
    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn z(&self) -> Enum9 {
        self.z
    }
    pub fn w(&self) -> u8 {
        self.w
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        3
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        if self.y() > 0x1f {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "y",
                value: self.y() as u64,
                maximum_value: 0x1f as u64,
            });
        }
        if self.w() > 0x7 {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "w",
                value: self.w() as u64,
                maximum_value: 0x7 as u64,
            });
        }
        let value = (u8::from(self.x()) as u32) | ((self.y() as u32) << 7)
            | ((u16::from(self.z()) as u32) << 12) | ((self.w() as u32) << 21);
        buf.put_uint_le(value as u64, 3);
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 3 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 3,
                got: buf.remaining(),
            });
        }
        let chunk = buf.get_uint_le(3) as u32;
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
        Ok((Self { x, y, z, w }, buf))
    }
}
