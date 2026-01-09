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
#[cfg_attr(feature = "serde", serde(try_from = "u16", into = "u16"))]
pub enum Enum16 {
    #[default]
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
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parent {
    pub foo: Enum16,
    pub bar: Enum16,
    pub baz: Enum16,
    pub payload: Vec<u8>,
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
            match (self.bar, self.baz, self.foo) {
                (_, _, Enum16::A)
                | (Enum16::A, _, Enum16::A)
                | (Enum16::A, Enum16::A, Enum16::A) => {
                    ParentChild::Child(self.try_into()?)
                }
                _ => ParentChild::None,
            },
        )
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn foo(&self) -> Enum16 {
        self.foo
    }
    pub fn bar(&self) -> Enum16 {
        self.bar
    }
    pub fn baz(&self) -> Enum16 {
        self.baz
    }
}
impl Packet for Parent {
    fn encoded_len(&self) -> usize {
        7 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u16(u16::from(self.foo()));
        buf.put_u16(u16::from(self.bar()));
        buf.put_u16(u16::from(self.baz()));
        if self.payload.len() > 0xff {
            return Err(EncodeError::SizeOverflow {
                packet: "Parent",
                field: "_payload_",
                size: self.payload.len(),
                maximum_size: 0xff,
            });
        }
        buf.put_u8((self.payload.len()) as u8);
        buf.put_slice(&self.payload);
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 2 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Parent",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let foo = Enum16::try_from(buf.get_u16())
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Parent",
                field: "foo",
                value: unknown_val as u64,
                type_: "Enum16",
            })?;
        if buf.remaining() < 2 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Parent",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let bar = Enum16::try_from(buf.get_u16())
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Parent",
                field: "bar",
                value: unknown_val as u64,
                type_: "Enum16",
            })?;
        if buf.remaining() < 2 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Parent",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let baz = Enum16::try_from(buf.get_u16())
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Parent",
                field: "baz",
                value: unknown_val as u64,
                type_: "Enum16",
            })?;
        if buf.remaining() < 1 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Parent",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let payload_size = buf.get_u8() as usize;
        if buf.remaining() < payload_size {
            return Err(DecodeError::InvalidLengthError {
                obj: "Parent",
                wanted: payload_size,
                got: buf.remaining(),
            });
        }
        let payload = buf[..payload_size].to_vec();
        buf.advance(payload_size);
        let payload = Vec::from(payload);
        Ok((Self { payload, foo, bar, baz }, buf))
    }
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Child {
    pub quux: Enum16,
    pub bar: Enum16,
    pub baz: Enum16,
    pub payload: Vec<u8>,
}
impl TryFrom<&Child> for Parent {
    type Error = EncodeError;
    fn try_from(packet: &Child) -> Result<Parent, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(Parent {
            foo: Enum16::A,
            bar: packet.bar,
            baz: packet.baz,
            payload,
        })
    }
}
impl TryFrom<Child> for Parent {
    type Error = EncodeError;
    fn try_from(packet: Child) -> Result<Parent, Self::Error> {
        (&packet).try_into()
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
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChildChild {
    GrandChild(GrandChild),
    #[default]
    None,
}
impl Child {
    pub fn specialize(&self) -> Result<ChildChild, DecodeError> {
        Ok(
            match (self.bar, self.baz, self.quux) {
                (Enum16::A, _, Enum16::A) | (Enum16::A, Enum16::A, Enum16::A) => {
                    ChildChild::GrandChild(self.try_into()?)
                }
                _ => ChildChild::None,
            },
        )
    }
    fn decode_partial(parent: &Parent) -> Result<Self, DecodeError> {
        let mut buf: &[u8] = &parent.payload;
        if parent.foo() != Enum16::A {
            return Err(DecodeError::InvalidFieldValue {
                packet: "Child",
                field: "foo",
                expected: "Enum16::A",
                actual: format!("{:?}", parent.foo()),
            });
        }
        if buf.remaining() < 2 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Child",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let quux = Enum16::try_from(buf.get_u16())
            .map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                obj: "Child",
                field: "quux",
                value: unknown_val as u64,
                type_: "Enum16",
            })?;
        let payload = buf.to_vec();
        buf.advance(payload.len());
        let payload = Vec::from(payload);
        if buf.is_empty() {
            Ok(Self {
                payload,
                quux,
                bar: parent.bar,
                baz: parent.baz,
            })
        } else {
            Err(DecodeError::TrailingBytes)
        }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u16(u16::from(self.quux()));
        buf.put_slice(&self.payload);
        Ok(())
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn quux(&self) -> Enum16 {
        self.quux
    }
    pub fn bar(&self) -> Enum16 {
        self.bar
    }
    pub fn baz(&self) -> Enum16 {
        self.baz
    }
    pub fn foo(&self) -> Enum16 {
        Enum16::A
    }
}
impl Packet for Child {
    fn encoded_len(&self) -> usize {
        9 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u16(u16::from(self.foo()));
        buf.put_u16(u16::from(self.bar()));
        buf.put_u16(u16::from(self.baz()));
        if 2 + self.payload.len() > 0xff {
            return Err(EncodeError::SizeOverflow {
                packet: "Parent",
                field: "_payload_",
                size: 2 + self.payload.len(),
                maximum_size: 0xff,
            });
        }
        buf.put_u8((2 + self.payload.len()) as u8);
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let (parent, trailing_bytes) = Parent::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrandChild {
    pub baz: Enum16,
    pub payload: Vec<u8>,
}
impl TryFrom<&GrandChild> for Child {
    type Error = EncodeError;
    fn try_from(packet: &GrandChild) -> Result<Child, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(Child {
            quux: Enum16::A,
            bar: Enum16::A,
            baz: packet.baz,
            payload,
        })
    }
}
impl TryFrom<GrandChild> for Child {
    type Error = EncodeError;
    fn try_from(packet: GrandChild) -> Result<Child, Self::Error> {
        (&packet).try_into()
    }
}
impl TryFrom<&Child> for GrandChild {
    type Error = DecodeError;
    fn try_from(parent: &Child) -> Result<GrandChild, Self::Error> {
        GrandChild::decode_partial(&parent)
    }
}
impl TryFrom<Child> for GrandChild {
    type Error = DecodeError;
    fn try_from(parent: Child) -> Result<GrandChild, Self::Error> {
        (&parent).try_into()
    }
}
impl TryFrom<&GrandChild> for Parent {
    type Error = EncodeError;
    fn try_from(packet: &GrandChild) -> Result<Parent, Self::Error> {
        (&Child::try_from(packet)?).try_into()
    }
}
impl TryFrom<GrandChild> for Parent {
    type Error = EncodeError;
    fn try_from(packet: GrandChild) -> Result<Parent, Self::Error> {
        (&packet).try_into()
    }
}
impl TryFrom<&Parent> for GrandChild {
    type Error = DecodeError;
    fn try_from(packet: &Parent) -> Result<GrandChild, Self::Error> {
        (&Child::try_from(packet)?).try_into()
    }
}
impl TryFrom<Parent> for GrandChild {
    type Error = DecodeError;
    fn try_from(packet: Parent) -> Result<GrandChild, Self::Error> {
        (&packet).try_into()
    }
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrandChildChild {
    GrandGrandChild(GrandGrandChild),
    #[default]
    None,
}
impl GrandChild {
    pub fn specialize(&self) -> Result<GrandChildChild, DecodeError> {
        Ok(
            match (self.baz) {
                (Enum16::A) => GrandChildChild::GrandGrandChild(self.try_into()?),
                _ => GrandChildChild::None,
            },
        )
    }
    fn decode_partial(parent: &Child) -> Result<Self, DecodeError> {
        let mut buf: &[u8] = &parent.payload;
        if parent.bar() != Enum16::A {
            return Err(DecodeError::InvalidFieldValue {
                packet: "GrandChild",
                field: "bar",
                expected: "Enum16::A",
                actual: format!("{:?}", parent.bar()),
            });
        }
        if parent.quux() != Enum16::A {
            return Err(DecodeError::InvalidFieldValue {
                packet: "GrandChild",
                field: "quux",
                expected: "Enum16::A",
                actual: format!("{:?}", parent.quux()),
            });
        }
        let payload = buf.to_vec();
        buf.advance(payload.len());
        let payload = Vec::from(payload);
        if buf.is_empty() {
            Ok(Self { payload, baz: parent.baz })
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
    pub fn baz(&self) -> Enum16 {
        self.baz
    }
    pub fn quux(&self) -> Enum16 {
        Enum16::A
    }
    pub fn foo(&self) -> Enum16 {
        Enum16::A
    }
    pub fn bar(&self) -> Enum16 {
        Enum16::A
    }
}
impl Packet for GrandChild {
    fn encoded_len(&self) -> usize {
        9 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u16(u16::from(self.foo()));
        buf.put_u16(u16::from(self.bar()));
        buf.put_u16(u16::from(self.baz()));
        if 2 + self.payload.len() > 0xff {
            return Err(EncodeError::SizeOverflow {
                packet: "Parent",
                field: "_payload_",
                size: 2 + self.payload.len(),
                maximum_size: 0xff,
            });
        }
        buf.put_u8((2 + self.payload.len()) as u8);
        buf.put_u16(u16::from(self.quux()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let (parent, trailing_bytes) = Child::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrandGrandChild {
    pub payload: Vec<u8>,
}
impl TryFrom<&GrandGrandChild> for GrandChild {
    type Error = EncodeError;
    fn try_from(packet: &GrandGrandChild) -> Result<GrandChild, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(GrandChild {
            baz: Enum16::A,
            payload,
        })
    }
}
impl TryFrom<GrandGrandChild> for GrandChild {
    type Error = EncodeError;
    fn try_from(packet: GrandGrandChild) -> Result<GrandChild, Self::Error> {
        (&packet).try_into()
    }
}
impl TryFrom<&GrandChild> for GrandGrandChild {
    type Error = DecodeError;
    fn try_from(parent: &GrandChild) -> Result<GrandGrandChild, Self::Error> {
        GrandGrandChild::decode_partial(&parent)
    }
}
impl TryFrom<GrandChild> for GrandGrandChild {
    type Error = DecodeError;
    fn try_from(parent: GrandChild) -> Result<GrandGrandChild, Self::Error> {
        (&parent).try_into()
    }
}
impl TryFrom<&GrandGrandChild> for Child {
    type Error = EncodeError;
    fn try_from(packet: &GrandGrandChild) -> Result<Child, Self::Error> {
        (&GrandChild::try_from(packet)?).try_into()
    }
}
impl TryFrom<GrandGrandChild> for Child {
    type Error = EncodeError;
    fn try_from(packet: GrandGrandChild) -> Result<Child, Self::Error> {
        (&packet).try_into()
    }
}
impl TryFrom<&GrandGrandChild> for Parent {
    type Error = EncodeError;
    fn try_from(packet: &GrandGrandChild) -> Result<Parent, Self::Error> {
        (&GrandChild::try_from(packet)?).try_into()
    }
}
impl TryFrom<GrandGrandChild> for Parent {
    type Error = EncodeError;
    fn try_from(packet: GrandGrandChild) -> Result<Parent, Self::Error> {
        (&packet).try_into()
    }
}
impl TryFrom<&Child> for GrandGrandChild {
    type Error = DecodeError;
    fn try_from(packet: &Child) -> Result<GrandGrandChild, Self::Error> {
        (&GrandChild::try_from(packet)?).try_into()
    }
}
impl TryFrom<Child> for GrandGrandChild {
    type Error = DecodeError;
    fn try_from(packet: Child) -> Result<GrandGrandChild, Self::Error> {
        (&packet).try_into()
    }
}
impl TryFrom<&Parent> for GrandGrandChild {
    type Error = DecodeError;
    fn try_from(packet: &Parent) -> Result<GrandGrandChild, Self::Error> {
        (&GrandChild::try_from(packet)?).try_into()
    }
}
impl TryFrom<Parent> for GrandGrandChild {
    type Error = DecodeError;
    fn try_from(packet: Parent) -> Result<GrandGrandChild, Self::Error> {
        (&packet).try_into()
    }
}
impl GrandGrandChild {
    fn decode_partial(parent: &GrandChild) -> Result<Self, DecodeError> {
        let mut buf: &[u8] = &parent.payload;
        if parent.baz() != Enum16::A {
            return Err(DecodeError::InvalidFieldValue {
                packet: "GrandGrandChild",
                field: "baz",
                expected: "Enum16::A",
                actual: format!("{:?}", parent.baz()),
            });
        }
        let payload = buf.to_vec();
        buf.advance(payload.len());
        let payload = Vec::from(payload);
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
    pub fn quux(&self) -> Enum16 {
        Enum16::A
    }
    pub fn foo(&self) -> Enum16 {
        Enum16::A
    }
    pub fn bar(&self) -> Enum16 {
        Enum16::A
    }
    pub fn baz(&self) -> Enum16 {
        Enum16::A
    }
}
impl Packet for GrandGrandChild {
    fn encoded_len(&self) -> usize {
        9 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u16(u16::from(self.foo()));
        buf.put_u16(u16::from(self.bar()));
        buf.put_u16(u16::from(self.baz()));
        if 2 + self.payload.len() > 0xff {
            return Err(EncodeError::SizeOverflow {
                packet: "Parent",
                field: "_payload_",
                size: 2 + self.payload.len(),
                maximum_size: 0xff,
            });
        }
        buf.put_u8((2 + self.payload.len()) as u8);
        buf.put_u16(u16::from(self.quux()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let (parent, trailing_bytes) = GrandChild::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
