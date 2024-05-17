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
#[cfg_attr(feature = "serde", serde(try_from = "u16", into = "u16"))]
pub enum Enum16 {
    A = 0x1,
    B = 0x2,
}
impl TryFrom<u16> for Enum16 {
    type Error = u16;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
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
pub struct Foo {
    pub a: u8,
    pub b: Enum16,
    pub payload: Vec<u8>,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FooChild {
    Bar(Bar),
    Baz(Baz),
    None,
}
impl Foo {
    pub fn specialize(&self) -> Result<FooChild, DecodeError> {
        Ok(
            match (self.a, self.b) {
                (100, _) => FooChild::Bar(self.try_into()?),
                (_, Enum16::B) => FooChild::Baz(self.try_into()?),
                _ => FooChild::None,
            },
        )
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn a(&self) -> u8 {
        self.a
    }
    pub fn b(&self) -> Enum16 {
        self.b
    }
}
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        4 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(self.a());
        buf.put_u16_le(u16::from(self.b()));
        if self.payload.len() > 0xff {
            return Err(EncodeError::SizeOverflow {
                packet: "Foo",
                field: "_payload_",
                size: self.payload.len(),
                maximum_size: 0xff,
            });
        }
        buf.put_u8(self.payload.len() as u8);
        buf.put_slice(&self.payload);
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 1 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let a = buf.get_u8();
        if buf.remaining() < 2 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let b = Enum16::try_from(buf.get_u16_le())
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Foo",
                field: "b",
                value: unknown_val as u64,
                type_: "Enum16",
            })?;
        if buf.remaining() < 1 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let payload_size = buf.get_u8() as usize;
        if buf.remaining() < payload_size {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: payload_size,
                got: buf.remaining(),
            });
        }
        let payload = buf[..payload_size].to_vec();
        buf.advance(payload_size);
        Ok((Self { payload, a, b }, buf))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar {
    pub x: u8,
    pub b: Enum16,
}
impl TryFrom<&Foo> for Bar {
    type Error = DecodeError;
    fn try_from(parent: &Foo) -> Result<Bar, Self::Error> {
        Bar::decode_partial(&parent)
    }
}
impl TryFrom<&Bar> for Foo {
    type Error = EncodeError;
    fn try_from(packet: &Bar) -> Result<Foo, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(Foo {
            a: 100,
            b: packet.b,
            payload,
        })
    }
}
impl TryFrom<&Bar> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: &Bar) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<&Bar> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: &Bar) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl Bar {
    fn decode_partial(parent: &Foo) -> Result<Self, DecodeError> {
        let mut buf: &[u8] = &parent.payload;
        if parent.a() != 100 {
            return Err(DecodeError::InvalidFieldValue {
                packet: "Bar",
                field: "a",
                expected: "100",
                actual: format!("{:?}", parent.a()),
            });
        }
        if buf.remaining() < 1 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Bar",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let x = buf.get_u8();
        if buf.is_empty() {
            Ok(Self { x, b: parent.b })
        } else {
            Err(DecodeError::TrailingBytes)
        }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(self.x());
        Ok(())
    }
    pub fn x(&self) -> u8 {
        self.x
    }
    pub fn b(&self) -> Enum16 {
        self.b
    }
    pub fn a(&self) -> u8 {
        100
    }
}
impl Packet for Bar {
    fn encoded_len(&self) -> usize {
        5
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(self.a());
        buf.put_u16_le(u16::from(self.b()));
        if 1 > 0xff {
            return Err(EncodeError::SizeOverflow {
                packet: "Foo",
                field: "_payload_",
                size: 1,
                maximum_size: 0xff,
            });
        }
        buf.put_u8(1 as u8);
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let (parent, trailing_bytes) = Foo::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Baz {
    pub y: u16,
    pub a: u8,
}
impl TryFrom<&Foo> for Baz {
    type Error = DecodeError;
    fn try_from(parent: &Foo) -> Result<Baz, Self::Error> {
        Baz::decode_partial(&parent)
    }
}
impl TryFrom<&Baz> for Foo {
    type Error = EncodeError;
    fn try_from(packet: &Baz) -> Result<Foo, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(Foo {
            a: packet.a,
            b: Enum16::B,
            payload,
        })
    }
}
impl TryFrom<&Baz> for Bytes {
    type Error = EncodeError;
    fn try_from(packet: &Baz) -> Result<Self, Self::Error> {
        packet.encode_to_bytes()
    }
}
impl TryFrom<&Baz> for Vec<u8> {
    type Error = EncodeError;
    fn try_from(packet: &Baz) -> Result<Self, Self::Error> {
        packet.encode_to_vec()
    }
}
impl Baz {
    fn decode_partial(parent: &Foo) -> Result<Self, DecodeError> {
        let mut buf: &[u8] = &parent.payload;
        if parent.b() != Enum16::B {
            return Err(DecodeError::InvalidFieldValue {
                packet: "Baz",
                field: "b",
                expected: "Enum16::B",
                actual: format!("{:?}", parent.b()),
            });
        }
        if buf.remaining() < 2 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Baz",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let y = buf.get_u16_le();
        if buf.is_empty() {
            Ok(Self { y, a: parent.a })
        } else {
            Err(DecodeError::TrailingBytes)
        }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u16_le(self.y());
        Ok(())
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn a(&self) -> u8 {
        self.a
    }
    pub fn b(&self) -> Enum16 {
        Enum16::B
    }
}
impl Packet for Baz {
    fn encoded_len(&self) -> usize {
        6
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u8(self.a());
        buf.put_u16_le(u16::from(self.b()));
        if 2 > 0xff {
            return Err(EncodeError::SizeOverflow {
                packet: "Foo",
                field: "_payload_",
                size: 2,
                maximum_size: 0xff,
            });
        }
        buf.put_u8(2 as u8);
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let (parent, trailing_bytes) = Foo::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
