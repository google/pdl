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
pub struct Test<'a> {
    pub payload: Array<'a, u8>,
}
impl<'a> Test<'a> {
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn into_owned<'b>(self) -> Test<'b> {
        Test {
            payload: self.payload.into_owned(),
        }
    }
}
impl<'a> Default for Test<'a> {
    fn default() -> Test<'a> {
        Test { payload: Array::default() }
    }
}
impl<'a> Packet<'a> for Test<'a> {
    fn encoded_len(&self) -> usize {
        1 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        #[allow(unused_comparisons)]
        if (self.payload.len() + 1) > 0xff {
            return Err(EncodeError::SizeOverflow {
                packet: "Test",
                field: "_payload_",
                size: (self.payload.len() + 1),
                maximum_size: 0xff,
            });
        }
        buf.put_u8(((self.payload.len() + 1)) as u8);
        buf.put_slice(&self.payload);
        Ok(())
    }
    fn decode(mut buf: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        if buf.remaining() < 1 {
            return Err(DecodeError::LengthError {
                obj: "Test",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let payload_size = buf.get_u8() as usize;
        if payload_size < 1 {
            return Err(DecodeError::LengthError {
                obj: "Test",
                wanted: 1,
                got: payload_size,
            });
        }
        let payload_size = payload_size - 1;
        if buf.remaining() < payload_size {
            return Err(DecodeError::LengthError {
                obj: "Test",
                wanted: payload_size,
                got: buf.remaining(),
            });
        }
        let (payload, tail) = buf.split_at(payload_size);
        buf = tail;
        let payload = Array::Borrowed(payload);
        Ok((Self { payload }, buf))
    }
}
