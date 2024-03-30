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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    pub b: u64,
}
impl TryFrom<&Foo> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: &Foo) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<&Foo> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: &Foo) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl Foo {
    pub fn b(&self) -> u64 {
        self.b
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        8
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        if self.b() > 0x1ff_ffff_ffff_ffff_u64 {
            return Err(EncodeError::InvalidScalarValue {
                packet: "Foo",
                field: "b",
                value: self.b() as u64,
                maximum_value: 0x1ff_ffff_ffff_ffff_u64 as u64,
            });
        }
        let value = (u8::from(Enum7::A) as u64) | (self.b() << 7);
        buf.put_u64(value);
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 8 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 8,
                got: buf.remaining(),
            });
        }
        let chunk = buf.get_u64();
        let fixed_value = (chunk & 0x7f) as u8;
        if fixed_value != u8::from(Enum7::A) {
            return Err(DecodeError::InvalidFixedValue {
                expected: u8::from(Enum7::A) as u64,
                actual: fixed_value as u64,
            });
        }
        let b = ((chunk >> 7) & 0x1ff_ffff_ffff_ffff_u64);
        Ok((Self { b }, buf))
    }
}
