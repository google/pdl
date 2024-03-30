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
    B = 0x1,
    C = 0x2,
}
impl TryFrom<u8> for Enum8 {
    type Error = u8;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
pub struct Parent {
    pub v: Enum8,
    pub payload: Vec<u8>,
}
impl TryFrom<&Parent> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: &Parent) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<&Parent> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: &Parent) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParentChild {
    AliasChild(AliasChild),
    NormalChild(NormalChild),
    None,
}
impl Parent {
    pub fn specialize(&self) -> Result<ParentChild, DecodeError> {
        Ok(
            match (self.v,) {
                (_,) => ParentChild::AliasChild(self.try_into()?),
                (Enum8::A,) => ParentChild::NormalChild(self.try_into()?),
                _ => ParentChild::None,
            },
        )
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn v(&self) -> Enum8 {
        self.v
    }
}
impl Packet for Parent {
    fn encoded_len(&self) -> usize {
        1 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        buf.put_slice(&self.payload);
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
        let payload = buf.to_vec();
        buf.advance(payload.len());
        Ok((Self { payload, v }, buf))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AliasChild {
    pub v: Enum8,
    pub payload: Vec<u8>,
}
impl TryFrom<&Parent> for AliasChild {
    type Error = DecodeError;
    fn try_from(parent: &Parent) -> Result<AliasChild, Self::Error> {
        AliasChild::decode_partial(&parent)
    }
}
impl TryFrom<&AliasChild> for Parent {
    type Error = EncodeError;
    fn try_from(packet: &AliasChild) -> Result<Parent, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(Parent { v: packet.v, payload })
    }
}
impl TryFrom<&AliasChild> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: &AliasChild) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<&AliasChild> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: &AliasChild) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AliasChildChild {
    NormalGrandChild1(NormalGrandChild1),
    NormalGrandChild2(NormalGrandChild2),
    None,
}
impl AliasChild {
    pub fn specialize(&self) -> Result<AliasChildChild, DecodeError> {
        Ok(
            match (self.v,) {
                (Enum8::B,) => AliasChildChild::NormalGrandChild1(self.try_into()?),
                (Enum8::C,) => AliasChildChild::NormalGrandChild2(self.try_into()?),
                _ => AliasChildChild::None,
            },
        )
    }
    fn decode_partial(parent: &Parent) -> Result<Self, DecodeError> {
        let mut buf: &[u8] = &parent.payload;
        let payload = buf.to_vec();
        buf.advance(payload.len());
        if buf.is_empty() {
            Ok(Self { payload, v: parent.v })
        } else {
            Err(DecodeError::TrailingBytes)
        }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_slice(&self.payload);
        Ok(())
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn v(&self) -> Enum8 {
        self.v
    }
}
impl Packet for AliasChild {
    fn encoded_len(&self) -> usize {
        1 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let (parent, trailing_bytes) = Parent::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalChild {}
impl TryFrom<&Parent> for NormalChild {
    type Error = DecodeError;
    fn try_from(parent: &Parent) -> Result<NormalChild, Self::Error> {
        NormalChild::decode_partial(&parent)
    }
}
impl TryFrom<&NormalChild> for Parent {
    type Error = EncodeError;
    fn try_from(packet: &NormalChild) -> Result<Parent, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(Parent { v: Enum8::A, payload })
    }
}
impl TryFrom<&NormalChild> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: &NormalChild) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<&NormalChild> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: &NormalChild) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl NormalChild {
    fn decode_partial(parent: &Parent) -> Result<Self, DecodeError> {
        let mut buf: &[u8] = &parent.payload;
        if buf.is_empty() { Ok(Self {}) } else { Err(DecodeError::TrailingBytes) }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        Ok(())
    }
    pub fn v(&self) -> Enum8 {
        Enum8::A
    }
}
impl Packet for NormalChild {
    fn encoded_len(&self) -> usize {
        1
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let (parent, trailing_bytes) = Parent::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalGrandChild1 {}
impl TryFrom<&AliasChild> for NormalGrandChild1 {
    type Error = DecodeError;
    fn try_from(parent: &AliasChild) -> Result<NormalGrandChild1, Self::Error> {
        NormalGrandChild1::decode_partial(&parent)
    }
}
impl TryFrom<&NormalGrandChild1> for AliasChild {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild1) -> Result<AliasChild, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(AliasChild { v: Enum8::B, payload })
    }
}
impl TryFrom<&NormalGrandChild1> for Parent {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild1) -> Result<Parent, Self::Error> {
        (&AliasChild::try_from(packet)?).try_into()
    }
}
impl TryFrom<&NormalGrandChild1> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild1) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<&NormalGrandChild1> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild1) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl NormalGrandChild1 {
    fn decode_partial(parent: &AliasChild) -> Result<Self, DecodeError> {
        let mut buf: &[u8] = &parent.payload;
        if buf.is_empty() { Ok(Self {}) } else { Err(DecodeError::TrailingBytes) }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        Ok(())
    }
    pub fn v(&self) -> Enum8 {
        Enum8::B
    }
}
impl Packet for NormalGrandChild1 {
    fn encoded_len(&self) -> usize {
        1
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let (parent, trailing_bytes) = AliasChild::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalGrandChild2 {
    pub payload: Vec<u8>,
}
impl TryFrom<&AliasChild> for NormalGrandChild2 {
    type Error = DecodeError;
    fn try_from(parent: &AliasChild) -> Result<NormalGrandChild2, Self::Error> {
        NormalGrandChild2::decode_partial(&parent)
    }
}
impl TryFrom<&NormalGrandChild2> for AliasChild {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild2) -> Result<AliasChild, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(AliasChild { v: Enum8::C, payload })
    }
}
impl TryFrom<&NormalGrandChild2> for Parent {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild2) -> Result<Parent, Self::Error> {
        (&AliasChild::try_from(packet)?).try_into()
    }
}
impl TryFrom<&NormalGrandChild2> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild2) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<&NormalGrandChild2> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild2) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl NormalGrandChild2 {
    fn decode_partial(parent: &AliasChild) -> Result<Self, DecodeError> {
        let mut buf: &[u8] = &parent.payload;
        let payload = buf.to_vec();
        buf.advance(payload.len());
        if buf.is_empty() {
            Ok(Self { payload })
        } else {
            Err(DecodeError::TrailingBytes)
        }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_slice(&self.payload);
        Ok(())
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn v(&self) -> Enum8 {
        Enum8::C
    }
}
impl Packet for NormalGrandChild2 {
    fn encoded_len(&self) -> usize {
        1 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let (parent, trailing_bytes) = AliasChild::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
