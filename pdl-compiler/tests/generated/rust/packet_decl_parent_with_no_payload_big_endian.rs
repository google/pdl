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
#[derive(Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "u8", into = "u8"))]
pub enum Enum8 {
    #[default]
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
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parent {
    pub v: Enum8,
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParentChild {
    Child(Child),
    #[default]
    None,
}
impl Parent {
    pub fn specialize(&self) -> Result<ParentChild, DecodeError> {
        Ok(
            match (self.v) {
                (Enum8::A) => ParentChild::Child(self.try_into()?),
                _ => ParentChild::None,
            },
        )
    }
    pub fn v(&self) -> Enum8 {
        self.v
    }
}
impl Packet for Parent {
    fn encoded_len(&self) -> usize {
        1
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 1 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Parent",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let v = Enum8::try_from(buf.get_u8())
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Parent",
                field: "v",
                value: unknown_val as u64,
                type_: "Enum8",
            })?;
        Ok((Self { v }, buf))
    }
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Child {}
impl From<&Child> for Parent {
    fn from(packet: &Child) -> Parent {
        Parent { v: Enum8::A }
    }
}
impl From<Child> for Parent {
    fn from(packet: Child) -> Parent {
        (&packet).into()
    }
}
impl TryFrom<&Parent> for Child {
    type Error = DecodeError;
    fn try_from(parent: &Parent) -> Result<Child, Self::Error> {
        Child::decode_partial(&parent)
    }
}
impl TryFrom<Parent> for Child {
    type Error = DecodeError;
    fn try_from(parent: Parent) -> Result<Child, Self::Error> {
        (&parent).try_into()
    }
}
impl Child {
    fn decode_partial(parent: &Parent) -> Result<Self, DecodeError> {
        if parent.v() != Enum8::A {
            return Err(DecodeError::InvalidFieldValue {
                packet: "Child",
                field: "v",
                expected: "Enum8::A",
                actual: format!("{:?}", parent.v()),
            });
        }
        Ok(Self {})
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        Ok(())
    }
    pub fn v(&self) -> Enum8 {
        Enum8::A
    }
}
impl Packet for Child {
    fn encoded_len(&self) -> usize {
        1
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        Ok(())
    }
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let (parent, trailing_bytes) = Parent::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
