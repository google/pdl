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
impl Packet for Foo {
    fn encoded_len(&self) -> usize {
        self.x.len() * 8
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        for elem in &self.x {
            buf.put_u64_le(*elem);
        }
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 7 * 8 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Foo",
                wanted: 7 * 8,
                got: buf.remaining(),
            });
        }
        let mut x = Vec::with_capacity(7);
        for _ in 0..7 {
            x.push(Ok::<_, DecodeError>(buf.get_u64_le())?)
        }
        let x = x.try_into().map_err(|_| DecodeError::InvalidPacketError)?;
        Ok((Self { x }, buf))
    }
}
