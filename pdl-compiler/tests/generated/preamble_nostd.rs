#![rustfmt::skip]
/// @generated rust_nostd packets from foo.pdl.
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
