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
#[cfg_attr(feature = "serde", serde(try_from = "u16", into = "u16"))]
pub enum Enum16 {
    A = 0x1,
    B = 0x2,
}
impl Default for Enum16 {
    fn default() -> Enum16 {
        Enum16::A
    }
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
pub struct Parent<'a> {
    pub foo: Enum16,
    pub bar: Enum16,
    pub baz: Enum16,
    pub payload: Array<'a, u8>,
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParentChild<'a> {
    Child(Child<'a>),
    #[default]
    None,
}
impl<'a> Parent<'a> {
    pub fn specialize(&self) -> Result<ParentChild<'a>, DecodeError> {
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
    pub fn into_owned<'b>(self) -> Parent<'b> {
        Parent {
            foo: self.foo,
            bar: self.bar,
            baz: self.baz,
            payload: self.payload.into_owned(),
        }
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
impl<'a> Default for Parent<'a> {
    fn default() -> Parent<'a> {
        Parent {
            foo: Default::default(),
            bar: Default::default(),
            baz: Default::default(),
            payload: Array::default(),
        }
    }
}
impl<'a> Packet<'a> for Parent<'a> {
    fn encoded_len(&self) -> usize {
        7 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u16_le(u16::from(self.foo()));
        buf.put_u16_le(u16::from(self.bar()));
        buf.put_u16_le(u16::from(self.baz()));
        #[allow(unused_comparisons)]
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
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.remaining() < 2 {
            return Err(DecodeError::LengthError {
                obj: "Parent",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let foo = Enum16::try_from(buf.get_u16_le())
            .map_err(|unknown_val| DecodeError::EnumValueError {
                obj: "Parent",
                field: "foo",
                value: unknown_val as u64,
                type_: "Enum16",
            })?;
        if buf.remaining() < 2 {
            return Err(DecodeError::LengthError {
                obj: "Parent",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let bar = Enum16::try_from(buf.get_u16_le())
            .map_err(|unknown_val| DecodeError::EnumValueError {
                obj: "Parent",
                field: "bar",
                value: unknown_val as u64,
                type_: "Enum16",
            })?;
        if buf.remaining() < 2 {
            return Err(DecodeError::LengthError {
                obj: "Parent",
                wanted: 2,
                got: buf.remaining(),
            });
        }
        let baz = Enum16::try_from(buf.get_u16_le())
            .map_err(|unknown_val| DecodeError::EnumValueError {
                obj: "Parent",
                field: "baz",
                value: unknown_val as u64,
                type_: "Enum16",
            })?;
        if buf.remaining() < 1 {
            return Err(DecodeError::LengthError {
                obj: "Parent",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let payload_size = buf.get_u8() as usize;
        if buf.remaining() < payload_size {
            return Err(DecodeError::LengthError {
                obj: "Parent",
                wanted: payload_size,
                got: buf.remaining(),
            });
        }
        let (payload, tail) = buf.split_at(payload_size);
        buf = tail;
        let payload = Array::Borrowed(payload);
        Ok((Self { payload, foo, bar, baz }, buf))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Child<'a> {
    pub quux: Enum16,
    pub bar: Enum16,
    pub baz: Enum16,
    pub payload: Array<'a, u8>,
}
impl<'a> TryFrom<&Child<'a>> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: &Child<'a>) -> Result<Parent<'a>, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(Parent {
            foo: Enum16::A,
            bar: packet.bar,
            baz: packet.baz,
            payload: Array::from(payload),
        })
    }
}
impl<'a> TryFrom<Child<'a>> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: Child<'a>) -> Result<Parent<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&Parent<'a>> for Child<'a> {
    type Error = DecodeError;
    fn try_from(parent: &Parent<'a>) -> Result<Child<'a>, Self::Error> {
        Child::decode_partial(&parent)
    }
}
impl<'a> TryFrom<Parent<'a>> for Child<'a> {
    type Error = DecodeError;
    fn try_from(parent: Parent<'a>) -> Result<Child<'a>, Self::Error> {
        (&parent).try_into()
    }
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChildChild<'a> {
    GrandChild(GrandChild<'a>),
    #[default]
    None,
}
impl<'a> Child<'a> {
    pub fn specialize(&self) -> Result<ChildChild<'a>, DecodeError> {
        Ok(
            match (self.bar, self.baz, self.quux) {
                (Enum16::A, _, Enum16::A) | (Enum16::A, Enum16::A, Enum16::A) => {
                    ChildChild::GrandChild(self.try_into()?)
                }
                _ => ChildChild::None,
            },
        )
    }
    fn decode_partial(parent: &Parent<'a>) -> Result<Self, DecodeError> {
        if parent.foo() != Enum16::A {
            return Err(DecodeError::ConstraintValueError {
                packet: "Child",
                field: "foo",
                expected: "Enum16::A",
                actual: alloc::format!("{:?}", parent.foo()),
            });
        }
        match parent.payload.as_slice() {
            Ok(mut buf) => {
                if buf.remaining() < 2 {
                    return Err(DecodeError::LengthError {
                        obj: "Child",
                        wanted: 2,
                        got: buf.remaining(),
                    });
                }
                let quux = Enum16::try_from(buf.get_u16_le())
                    .map_err(|unknown_val| DecodeError::EnumValueError {
                        obj: "Child",
                        field: "quux",
                        value: unknown_val as u64,
                        type_: "Enum16",
                    })?;
                let (payload, tail) = buf.split_at(buf.len());
                buf = tail;
                let payload = Array::Borrowed(payload);
                if buf.is_empty() {
                    Ok(Self {
                        payload,
                        quux,
                        bar: parent.bar,
                        baz: parent.baz,
                    })
                } else {
                    Err(DecodeError::TrailingBytesError)
                }
            }
            Err(mut buf) => {
                if buf.remaining() < 2 {
                    return Err(DecodeError::LengthError {
                        obj: "Child",
                        wanted: 2,
                        got: buf.remaining(),
                    });
                }
                let quux = Enum16::try_from(buf.get_u16_le())
                    .map_err(|unknown_val| DecodeError::EnumValueError {
                        obj: "Child",
                        field: "quux",
                        value: unknown_val as u64,
                        type_: "Enum16",
                    })?;
                let (payload, tail) = buf.split_at(buf.len());
                buf = tail;
                let payload = Array::Borrowed(payload);
                if buf.is_empty() {
                    Ok(Self {
                        payload: payload.into_owned(),
                        quux: quux,
                        bar: parent.bar,
                        baz: parent.baz,
                    })
                } else {
                    Err(DecodeError::TrailingBytesError)
                }
            }
        }
    }
    pub fn encode_partial(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u16_le(u16::from(self.quux()));
        buf.put_slice(&self.payload);
        Ok(())
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn into_owned<'b>(self) -> Child<'b> {
        Child {
            quux: self.quux,
            bar: self.bar,
            baz: self.baz,
            payload: self.payload.into_owned(),
        }
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
impl<'a> Default for Child<'a> {
    fn default() -> Child<'a> {
        Child {
            quux: Default::default(),
            bar: Default::default(),
            baz: Default::default(),
            payload: Array::default(),
        }
    }
}
impl<'a> Packet<'a> for Child<'a> {
    fn encoded_len(&self) -> usize {
        9 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u16_le(u16::from(self.foo()));
        buf.put_u16_le(u16::from(self.bar()));
        buf.put_u16_le(u16::from(self.baz()));
        #[allow(unused_comparisons)]
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
    fn decode(buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        let (parent, trailing_bytes) = Parent::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrandChild<'a> {
    pub baz: Enum16,
    pub payload: Array<'a, u8>,
}
impl<'a> TryFrom<&GrandChild<'a>> for Child<'a> {
    type Error = EncodeError;
    fn try_from(packet: &GrandChild<'a>) -> Result<Child<'a>, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(Child {
            quux: Enum16::A,
            bar: Enum16::A,
            baz: packet.baz,
            payload: Array::from(payload),
        })
    }
}
impl<'a> TryFrom<GrandChild<'a>> for Child<'a> {
    type Error = EncodeError;
    fn try_from(packet: GrandChild<'a>) -> Result<Child<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&Child<'a>> for GrandChild<'a> {
    type Error = DecodeError;
    fn try_from(parent: &Child<'a>) -> Result<GrandChild<'a>, Self::Error> {
        GrandChild::decode_partial(&parent)
    }
}
impl<'a> TryFrom<Child<'a>> for GrandChild<'a> {
    type Error = DecodeError;
    fn try_from(parent: Child<'a>) -> Result<GrandChild<'a>, Self::Error> {
        (&parent).try_into()
    }
}
impl<'a> TryFrom<&GrandChild<'a>> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: &GrandChild<'a>) -> Result<Parent<'a>, Self::Error> {
        (&Child::try_from(packet)?).try_into()
    }
}
impl<'a> TryFrom<GrandChild<'a>> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: GrandChild<'a>) -> Result<Parent<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&Parent<'a>> for GrandChild<'a> {
    type Error = DecodeError;
    fn try_from(packet: &Parent<'a>) -> Result<GrandChild<'a>, Self::Error> {
        (&Child::try_from(packet)?).try_into()
    }
}
impl<'a> TryFrom<Parent<'a>> for GrandChild<'a> {
    type Error = DecodeError;
    fn try_from(packet: Parent<'a>) -> Result<GrandChild<'a>, Self::Error> {
        (&packet).try_into()
    }
}
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrandChildChild<'a> {
    GrandGrandChild(GrandGrandChild<'a>),
    #[default]
    None,
}
impl<'a> GrandChild<'a> {
    pub fn specialize(&self) -> Result<GrandChildChild<'a>, DecodeError> {
        Ok(
            match (self.baz) {
                (Enum16::A) => GrandChildChild::GrandGrandChild(self.try_into()?),
                _ => GrandChildChild::None,
            },
        )
    }
    fn decode_partial(parent: &Child<'a>) -> Result<Self, DecodeError> {
        if parent.bar() != Enum16::A {
            return Err(DecodeError::ConstraintValueError {
                packet: "GrandChild",
                field: "bar",
                expected: "Enum16::A",
                actual: alloc::format!("{:?}", parent.bar()),
            });
        }
        if parent.quux() != Enum16::A {
            return Err(DecodeError::ConstraintValueError {
                packet: "GrandChild",
                field: "quux",
                expected: "Enum16::A",
                actual: alloc::format!("{:?}", parent.quux()),
            });
        }
        match parent.payload.as_slice() {
            Ok(mut buf) => {
                let (payload, tail) = buf.split_at(buf.len());
                buf = tail;
                let payload = Array::Borrowed(payload);
                if buf.is_empty() {
                    Ok(Self { payload, baz: parent.baz })
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
                        baz: parent.baz,
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
    pub fn into_owned<'b>(self) -> GrandChild<'b> {
        GrandChild {
            baz: self.baz,
            payload: self.payload.into_owned(),
        }
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
impl<'a> Default for GrandChild<'a> {
    fn default() -> GrandChild<'a> {
        GrandChild {
            baz: Default::default(),
            payload: Array::default(),
        }
    }
}
impl<'a> Packet<'a> for GrandChild<'a> {
    fn encoded_len(&self) -> usize {
        9 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u16_le(u16::from(self.foo()));
        buf.put_u16_le(u16::from(self.bar()));
        buf.put_u16_le(u16::from(self.baz()));
        #[allow(unused_comparisons)]
        if 2 + self.payload.len() > 0xff {
            return Err(EncodeError::SizeOverflow {
                packet: "Parent",
                field: "_payload_",
                size: 2 + self.payload.len(),
                maximum_size: 0xff,
            });
        }
        buf.put_u8((2 + self.payload.len()) as u8);
        buf.put_u16_le(u16::from(self.quux()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        let (parent, trailing_bytes) = Child::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrandGrandChild<'a> {
    pub payload: Array<'a, u8>,
}
impl<'a> TryFrom<&GrandGrandChild<'a>> for GrandChild<'a> {
    type Error = EncodeError;
    fn try_from(packet: &GrandGrandChild<'a>) -> Result<GrandChild<'a>, Self::Error> {
        let mut payload = Vec::new();
        packet.encode_partial(&mut payload)?;
        Ok(GrandChild {
            baz: Enum16::A,
            payload: Array::from(payload),
        })
    }
}
impl<'a> TryFrom<GrandGrandChild<'a>> for GrandChild<'a> {
    type Error = EncodeError;
    fn try_from(packet: GrandGrandChild<'a>) -> Result<GrandChild<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&GrandChild<'a>> for GrandGrandChild<'a> {
    type Error = DecodeError;
    fn try_from(parent: &GrandChild<'a>) -> Result<GrandGrandChild<'a>, Self::Error> {
        GrandGrandChild::decode_partial(&parent)
    }
}
impl<'a> TryFrom<GrandChild<'a>> for GrandGrandChild<'a> {
    type Error = DecodeError;
    fn try_from(parent: GrandChild<'a>) -> Result<GrandGrandChild<'a>, Self::Error> {
        (&parent).try_into()
    }
}
impl<'a> TryFrom<&GrandGrandChild<'a>> for Child<'a> {
    type Error = EncodeError;
    fn try_from(packet: &GrandGrandChild<'a>) -> Result<Child<'a>, Self::Error> {
        (&GrandChild::try_from(packet)?).try_into()
    }
}
impl<'a> TryFrom<GrandGrandChild<'a>> for Child<'a> {
    type Error = EncodeError;
    fn try_from(packet: GrandGrandChild<'a>) -> Result<Child<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&GrandGrandChild<'a>> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: &GrandGrandChild<'a>) -> Result<Parent<'a>, Self::Error> {
        (&GrandChild::try_from(packet)?).try_into()
    }
}
impl<'a> TryFrom<GrandGrandChild<'a>> for Parent<'a> {
    type Error = EncodeError;
    fn try_from(packet: GrandGrandChild<'a>) -> Result<Parent<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&Child<'a>> for GrandGrandChild<'a> {
    type Error = DecodeError;
    fn try_from(packet: &Child<'a>) -> Result<GrandGrandChild<'a>, Self::Error> {
        (&GrandChild::try_from(packet)?).try_into()
    }
}
impl<'a> TryFrom<Child<'a>> for GrandGrandChild<'a> {
    type Error = DecodeError;
    fn try_from(packet: Child<'a>) -> Result<GrandGrandChild<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> TryFrom<&Parent<'a>> for GrandGrandChild<'a> {
    type Error = DecodeError;
    fn try_from(packet: &Parent<'a>) -> Result<GrandGrandChild<'a>, Self::Error> {
        (&GrandChild::try_from(packet)?).try_into()
    }
}
impl<'a> TryFrom<Parent<'a>> for GrandGrandChild<'a> {
    type Error = DecodeError;
    fn try_from(packet: Parent<'a>) -> Result<GrandGrandChild<'a>, Self::Error> {
        (&packet).try_into()
    }
}
impl<'a> GrandGrandChild<'a> {
    fn decode_partial(parent: &GrandChild<'a>) -> Result<Self, DecodeError> {
        if parent.baz() != Enum16::A {
            return Err(DecodeError::ConstraintValueError {
                packet: "GrandGrandChild",
                field: "baz",
                expected: "Enum16::A",
                actual: alloc::format!("{:?}", parent.baz()),
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
    pub fn into_owned<'b>(self) -> GrandGrandChild<'b> {
        GrandGrandChild {
            payload: self.payload.into_owned(),
        }
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
impl<'a> Default for GrandGrandChild<'a> {
    fn default() -> GrandGrandChild<'a> {
        GrandGrandChild {
            payload: Array::default(),
        }
    }
}
impl<'a> Packet<'a> for GrandGrandChild<'a> {
    fn encoded_len(&self) -> usize {
        9 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        buf.put_u16_le(u16::from(self.foo()));
        buf.put_u16_le(u16::from(self.bar()));
        buf.put_u16_le(u16::from(self.baz()));
        #[allow(unused_comparisons)]
        if 2 + self.payload.len() > 0xff {
            return Err(EncodeError::SizeOverflow {
                packet: "Parent",
                field: "_payload_",
                size: 2 + self.payload.len(),
                maximum_size: 0xff,
            });
        }
        buf.put_u8((2 + self.payload.len()) as u8);
        buf.put_u16_le(u16::from(self.quux()));
        self.encode_partial(buf)?;
        Ok(())
    }
    fn decode(buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        let (parent, trailing_bytes) = GrandChild::decode(buf)?;
        let packet = Self::decode_partial(&parent)?;
        Ok((packet, trailing_bytes))
    }
}
