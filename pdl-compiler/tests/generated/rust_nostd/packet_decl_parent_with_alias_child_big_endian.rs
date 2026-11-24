#![rustfmt::skip]
/// @generated rust_nostd packets from test.
extern crate alloc;
use alloc::vec::Vec;
use bytes::{Buf, BufMut};
use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::result::Result;
use pdl_runtime::nostd::Packet;
use pdl_runtime::{Array, DecodeError, EncodeError};
/// Private prevents users from creating arbitrary scalar values
/// in situations where the value needs to be validated.
/// Users can freely deref the value, but only the backend
/// may create it.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Private<T>(T);
impl<T> core::ops::Deref for Private<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: core::fmt::Debug> core::fmt::Debug for Private<T> {
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
impl Default for Enum8 {
    fn default() -> Enum8 {
        Enum8::A
    }
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
pub struct Parent<'a> {
    pub v: Enum8,
    pub payload: Array<'a, u8>,
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParentChild<'a> {
    AliasChild(AliasChild<'a>),
    NormalChild(NormalChild),
    #[default]
    None,
}
impl<'a> Parent<'a> {
    pub fn specialize(&self) -> Result<ParentChild<'a>, DecodeError> {
        Ok(
            match (self.v) {
                (Enum8::B) | (Enum8::C) => ParentChild::AliasChild(self.try_into()?),
                (Enum8::A) => ParentChild::NormalChild(self.try_into()?),
                _ => ParentChild::None,
            },
        )
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn into_owned<'b>(self) -> Parent<'b> {
        Parent {
            v: self.v,
            payload: self.payload.into_owned(),
        }
    }
    pub fn v(&self) -> Enum8 {
        self.v
    }
}
impl<'a> Default for Parent<'a> {
    fn default() -> Parent<'a> {
        Parent {
            v: Default::default(),
            payload: Array::default(),
        }
    }
}
impl<'a> Packet<'a> for Parent<'a> {
    fn encoded_len(&self) -> usize {
        1 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        buf.put_slice(&self.payload);
        Ok(())
    }
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.remaining() < 1 {
            return Err(DecodeError::LengthError {
                obj: "Parent",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let v = Enum8::try_from(buf.get_u8())
            .map_err(|unknown_val| DecodeError::EnumValueError {
                obj: "Parent",
                field: "v",
                value: unknown_val as u64,
                type_: "Enum8",
            })?;
        let (payload, tail) = buf.split_at(buf.len());
        buf = tail;
        let payload = Array::Borrowed(payload);
        Ok((Self { payload, v }, buf))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AliasChild<'a> {
    pub v: Enum8,
    pub payload: Array<'a, u8>,
}
impl<'a> TryFrom<&AliasChild<'a>> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: &AliasChild<'a>) -> Result<Parent<'a>, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(Parent {
            v: packet.v,
            payload: Array::from(payload),
        })
    }
}
impl<'a> TryFrom<AliasChild<'a>> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: AliasChild<'a>) -> Result<Parent<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&Parent<'a>> for AliasChild<'a> {
    type Error = DecodeError;
    fn try_from(parent: &Parent<'a>) -> Result<AliasChild<'a>, Self::Error> {
        AliasChild::decode_partial(&parent)
    }
}
impl<'a> TryFrom<Parent<'a>> for AliasChild<'a> {
    type Error = DecodeError;
    fn try_from(parent: Parent<'a>) -> Result<AliasChild<'a>, Self::Error> {
        (&parent).try_into()
    }
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AliasChildChild<'a> {
    NormalGrandChild1(NormalGrandChild1),
    NormalGrandChild2(NormalGrandChild2<'a>),
    #[default]
    None,
}
impl<'a> AliasChild<'a> {
    pub fn specialize(&self) -> Result<AliasChildChild<'a>, DecodeError> {
        Ok(
            match (self.v) {
                (Enum8::B) => AliasChildChild::NormalGrandChild1(self.try_into()?),
                (Enum8::C) => AliasChildChild::NormalGrandChild2(self.try_into()?),
                _ => AliasChildChild::None,
            },
        )
    }
    fn decode_partial(parent: &Parent<'a>) -> Result<Self, DecodeError> {
        match parent.payload.as_slice() {
            Ok(mut buf) => {
                let (payload, tail) = buf.split_at(buf.len());
                buf = tail;
                let payload = Array::Borrowed(payload);
                if buf.is_empty() {
                    Ok(Self { payload, v: parent.v })
                } else {
                    Err(DecodeError::TrailingBytesError)
                }
            }
            Err(mut buf) => {
                let (payload, tail) = buf.split_at(buf.len());
                buf = tail;
                let payload = Array::Borrowed(payload);
                if buf.is_empty() {
                    Ok(Self {
                        payload: payload.into_owned(),
                        v: parent.v,
                    })
                } else {
                    Err(DecodeError::TrailingBytesError)
                }
            }
        }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_slice(&self.payload);
        Ok(())
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn into_owned<'b>(self) -> AliasChild<'b> {
        AliasChild {
            v: self.v,
            payload: self.payload.into_owned(),
        }
    }
    pub fn v(&self) -> Enum8 {
        self.v
    }
}
impl<'a> Default for AliasChild<'a> {
    fn default() -> AliasChild<'a> {
        AliasChild {
            v: Default::default(),
            payload: Array::default(),
        }
    }
}
impl<'a> Packet<'a> for AliasChild<'a> {
    fn encoded_len(&self) -> usize {
        1 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        let (parent, trailing_bytes) = Parent::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalChild {}
impl<'a> TryFrom<&NormalChild> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: &NormalChild) -> Result<Parent<'a>, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(Parent {
            v: Enum8::A,
            payload: Array::from(payload),
        })
    }
}
impl<'a> TryFrom<NormalChild> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: NormalChild) -> Result<Parent<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&Parent<'a>> for NormalChild {
    type Error = DecodeError;
    fn try_from(parent: &Parent<'a>) -> Result<NormalChild, Self::Error> {
        NormalChild::decode_partial(&parent)
    }
}
impl<'a> TryFrom<Parent<'a>> for NormalChild {
    type Error = DecodeError;
    fn try_from(parent: Parent<'a>) -> Result<NormalChild, Self::Error> {
        (&parent).try_into()
    }
}
impl NormalChild {
    fn decode_partial(parent: &Parent<'_>) -> Result<Self, DecodeError> {
        let mut buf: &[u8] = &parent.payload;
        if parent.v() != Enum8::A {
            return Err(DecodeError::ConstraintValueError {
                packet: "NormalChild",
                field: "v",
                expected: "Enum8::A",
                actual: alloc::format!("{:?}", parent.v()),
            });
        }
        if buf.is_empty() { Ok(Self {}) } else { Err(DecodeError::TrailingBytesError) }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        Ok(())
    }
    pub fn v(&self) -> Enum8 {
        Enum8::A
    }
}
impl Default for NormalChild {
    fn default() -> NormalChild {
        NormalChild {}
    }
}
impl<'a> Packet<'a> for NormalChild {
    fn encoded_len(&self) -> usize {
        1
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        let (parent, trailing_bytes) = Parent::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalGrandChild1 {}
impl<'a> TryFrom<&NormalGrandChild1> for AliasChild<'a> {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild1) -> Result<AliasChild<'a>, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(AliasChild {
            v: Enum8::B,
            payload: Array::from(payload),
        })
    }
}
impl<'a> TryFrom<NormalGrandChild1> for AliasChild<'a> {
    type Error = EncodeError;
    fn try_from(packet: NormalGrandChild1) -> Result<AliasChild<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&AliasChild<'a>> for NormalGrandChild1 {
    type Error = DecodeError;
    fn try_from(parent: &AliasChild<'a>) -> Result<NormalGrandChild1, Self::Error> {
        NormalGrandChild1::decode_partial(&parent)
    }
}
impl<'a> TryFrom<AliasChild<'a>> for NormalGrandChild1 {
    type Error = DecodeError;
    fn try_from(parent: AliasChild<'a>) -> Result<NormalGrandChild1, Self::Error> {
        (&parent).try_into()
    }
}
impl<'a> TryFrom<&NormalGrandChild1> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild1) -> Result<Parent<'a>, Self::Error> {
        (&AliasChild::try_from(packet)?).try_into()
    }
}
impl<'a> TryFrom<NormalGrandChild1> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: NormalGrandChild1) -> Result<Parent<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&Parent<'a>> for NormalGrandChild1 {
    type Error = DecodeError;
    fn try_from(packet: &Parent<'a>) -> Result<NormalGrandChild1, Self::Error> {
        (&AliasChild::try_from(packet)?).try_into()
    }
}
impl<'a> TryFrom<Parent<'a>> for NormalGrandChild1 {
    type Error = DecodeError;
    fn try_from(packet: Parent<'a>) -> Result<NormalGrandChild1, Self::Error> {
        (&packet).try_into()
    }
}
impl NormalGrandChild1 {
    fn decode_partial(parent: &AliasChild<'_>) -> Result<Self, DecodeError> {
        let mut buf: &[u8] = &parent.payload;
        if parent.v() != Enum8::B {
            return Err(DecodeError::ConstraintValueError {
                packet: "NormalGrandChild1",
                field: "v",
                expected: "Enum8::B",
                actual: alloc::format!("{:?}", parent.v()),
            });
        }
        if buf.is_empty() { Ok(Self {}) } else { Err(DecodeError::TrailingBytesError) }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        Ok(())
    }
    pub fn v(&self) -> Enum8 {
        Enum8::B
    }
}
impl Default for NormalGrandChild1 {
    fn default() -> NormalGrandChild1 {
        NormalGrandChild1 {}
    }
}
impl<'a> Packet<'a> for NormalGrandChild1 {
    fn encoded_len(&self) -> usize {
        1
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        let (parent, trailing_bytes) = AliasChild::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalGrandChild2<'a> {
    pub payload: Array<'a, u8>,
}
impl<'a> TryFrom<&NormalGrandChild2<'a>> for AliasChild<'a> {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild2<'a>) -> Result<AliasChild<'a>, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(AliasChild {
            v: Enum8::C,
            payload: Array::from(payload),
        })
    }
}
impl<'a> TryFrom<NormalGrandChild2<'a>> for AliasChild<'a> {
    type Error = EncodeError;
    fn try_from(packet: NormalGrandChild2<'a>) -> Result<AliasChild<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&AliasChild<'a>> for NormalGrandChild2<'a> {
    type Error = DecodeError;
    fn try_from(parent: &AliasChild<'a>) -> Result<NormalGrandChild2<'a>, Self::Error> {
        NormalGrandChild2::decode_partial(&parent)
    }
}
impl<'a> TryFrom<AliasChild<'a>> for NormalGrandChild2<'a> {
    type Error = DecodeError;
    fn try_from(parent: AliasChild<'a>) -> Result<NormalGrandChild2<'a>, Self::Error> {
        (&parent).try_into()
    }
}
impl<'a> TryFrom<&NormalGrandChild2<'a>> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: &NormalGrandChild2<'a>) -> Result<Parent<'a>, Self::Error> {
        (&AliasChild::try_from(packet)?).try_into()
    }
}
impl<'a> TryFrom<NormalGrandChild2<'a>> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: NormalGrandChild2<'a>) -> Result<Parent<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&Parent<'a>> for NormalGrandChild2<'a> {
    type Error = DecodeError;
    fn try_from(packet: &Parent<'a>) -> Result<NormalGrandChild2<'a>, Self::Error> {
        (&AliasChild::try_from(packet)?).try_into()
    }
}
impl<'a> TryFrom<Parent<'a>> for NormalGrandChild2<'a> {
    type Error = DecodeError;
    fn try_from(packet: Parent<'a>) -> Result<NormalGrandChild2<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> NormalGrandChild2<'a> {
    fn decode_partial(parent: &AliasChild<'a>) -> Result<Self, DecodeError> {
        if parent.v() != Enum8::C {
            return Err(DecodeError::ConstraintValueError {
                packet: "NormalGrandChild2",
                field: "v",
                expected: "Enum8::C",
                actual: alloc::format!("{:?}", parent.v()),
            });
        }
        match parent.payload.as_slice() {
            Ok(mut buf) => {
                let (payload, tail) = buf.split_at(buf.len());
                buf = tail;
                let payload = Array::Borrowed(payload);
                if buf.is_empty() {
                    Ok(Self { payload })
                } else {
                    Err(DecodeError::TrailingBytesError)
                }
            }
            Err(mut buf) => {
                let (payload, tail) = buf.split_at(buf.len());
                buf = tail;
                let payload = Array::Borrowed(payload);
                if buf.is_empty() {
                    Ok(Self {
                        payload: payload.into_owned(),
                    })
                } else {
                    Err(DecodeError::TrailingBytesError)
                }
            }
        }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_slice(&self.payload);
        Ok(())
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn into_owned<'b>(self) -> NormalGrandChild2<'b> {
        NormalGrandChild2 {
            payload: self.payload.into_owned(),
        }
    }
    pub fn v(&self) -> Enum8 {
        Enum8::C
    }
}
impl<'a> Default for NormalGrandChild2<'a> {
    fn default() -> NormalGrandChild2<'a> {
        NormalGrandChild2 {
            payload: Array::default(),
        }
    }
}
impl<'a> Packet<'a> for NormalGrandChild2<'a> {
    fn encoded_len(&self) -> usize {
        1 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(u8::from(self.v()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        let (parent, trailing_bytes) = AliasChild::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
