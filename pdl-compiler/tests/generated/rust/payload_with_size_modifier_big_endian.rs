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
pub struct Test {
    pub payload: Vec<u8>,
}
impl Test {
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}
impl Packet for Test {
    fn encoded_len(&self) -> usize {
        1 + self.payload.len()
    }
    fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError> {
        if (self.payload.len() + 1) > 0xff {
            return Err(EncodeError::SizeOverflow {
                packet: "Test",
                field: "_payload_",
                size: (self.payload.len() + 1),
                maximum_size: 0xff,
            });
        }
        buf.put_u8((self.payload.len() + 1) as u8);
        buf.put_slice(&self.payload);
        Ok(())
    }
    fn decode(mut buf: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        if buf.remaining() < 1 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Test",
                wanted: 1,
                got: buf.remaining(),
            });
        }
        let payload_size = buf.get_u8() as usize;
        if payload_size < 1 {
            return Err(DecodeError::InvalidLengthError {
                obj: "Test",
                wanted: 1,
                got: payload_size,
            });
        }
        let payload_size = payload_size - 1;
        if buf.remaining() < payload_size {
            return Err(DecodeError::InvalidLengthError {
                obj: "Test",
                wanted: payload_size,
                got: buf.remaining(),
            });
        }
        let payload = buf[..payload_size].to_vec();
        buf.advance(payload_size);
        Ok((Self { payload }, buf))
    }
}
