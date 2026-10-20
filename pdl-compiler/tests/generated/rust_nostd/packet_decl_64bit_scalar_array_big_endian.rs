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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    pub x: [u64; 7],
}
impl Foo {
    pub fn x(&self) -> &[u64; 7] {
        &self.x
    }
}
impl Default for Foo {
    fn default() -> Foo {
        Foo { x: [0; 7usize] }
    }
}
impl<'a> Packet<'a> for Foo {
    fn encoded_len(&self) -> usize {
        (self.x.len() * 8)
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        for elem in &self.x {
            buf.put_u64(*elem);
        }
        Ok(())
    }
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.remaining() < 7 * 8 {
            return Err(DecodeError::LengthError {
                obj: "Foo",
                wanted: 7 * 8,
                got: buf.remaining(),
            });
        }
        let mut x = core::array::from_fn::<_, 7, _>(|_| Default::default());
        for elem in &mut x {
            *elem = buf.get_u64();
        }
        Ok((Self { x }, buf))
    }
}
