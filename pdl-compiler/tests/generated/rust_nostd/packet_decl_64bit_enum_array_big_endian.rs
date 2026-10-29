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
#[cfg_attr(feature = "serde", serde(try_from = "u64", into = "u64"))]
pub enum Foo {
    FooBar = 0x1,
    Baz = 0x2,
}
impl Default for Foo {
    fn default() -> Foo {
        Foo::FooBar
    }
}
impl TryFrom<u64> for Foo {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Foo::FooBar),
            0x2 => Ok(Foo::Baz),
            _ => Err(value),
        }
    }
}
impl From<&Foo> for u64 {
    fn from(value: &Foo) -> Self {
        match value {
            Foo::FooBar => 0x1,
            Foo::Baz => 0x2,
        }
    }
}
impl From<Foo> for u64 {
    fn from(value: Foo) -> Self {
        (&value).into()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bar {
    pub x: [Foo; 7],
}
impl Bar {
    pub fn x(&self) -> &[Foo; 7] {
        &self.x
    }
}
impl Default for Bar {
    fn default() -> Bar {
        Bar {
            x: core::array::from_fn(|_| Default::default()),
        }
    }
}
impl<'a> Packet<'a> for Bar {
    fn encoded_len(&self) -> usize {
        (self.x.len() * 8)
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        for elem in &self.x {
            buf.put_u64(u64::from(*elem));
        }
        Ok(())
    }
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.remaining() < 7 * 8 {
            return Err(DecodeError::LengthError {
                obj: "Bar",
                wanted: 7 * 8,
                got: buf.remaining(),
            });
        }
        let mut x = core::array::from_fn::<_, 7, _>(|_| Default::default());
        for elem in &mut x {
            *elem = Foo::try_from(buf.get_u64())
                .map_err(|unknown_val| DecodeError::EnumValueError {
                    obj: "Bar",
                    field: "",
                    value: unknown_val as u64,
                    type_: "Foo",
                })?;
        }
        Ok((Self { x }, buf))
    }
}
