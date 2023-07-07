pub use bytes::{BufMut, Buf};
use thiserror::Error;

mod array;
pub use array::Array;

mod scalar;
pub use scalar::{u16le, u16be, u32le, u32be, u64le, u64be, u128le, u128be};

#[derive(Debug, Error)]
pub enum Error {
  #[error("insufficient bytes to read from")]
  InsufficientBytesReadError,
  #[error("insufficient bytes to write into")]
  InsufficientBytesWriteError,
}

pub trait Packetable<'a>: Sized + Clone {
  type Error: From<Error>;

  fn bytes(&self) -> usize;
  unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut);

  fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error>;

  fn write_into(&self, buf: &mut impl BufMut) -> Result<(), Self::Error> {
    if buf.remaining_mut() < self.bytes() {
      return Err(Self::Error::from(Error::InsufficientBytesWriteError));
    }
    // SAFETY: destination buffer has been checked.
    unsafe { Ok(Self::write_into_unchecked(self, buf)) }
  }

  fn from_bytes(mut bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
    Self::read_from(&mut bytes).map(|res| (res, bytes))
  }

  fn to_bytes(&self) -> Vec<u8> {
    let mut res = vec![];
    let _ = self.write_into(&mut res);
    res
  }
}
