/// This provides implementation for a variety of scalar types.
/// Each of these types can be packed into bytes and unpacked from bytes.
/// They are essentially wrapper types that implement the `Packetable` trait.

use crate::{Packetable, Error, Buf, BufMut};

use derive_more::{Display, From, Into, Not, Add, Mul};

macro_rules! impl_scalar {
  ($typ:ident, $bytes:expr, $get:ident, $put:ident + Debug) => {
    impl_scalar!($typ, $bytes, $get, $put);
    impl ::std::fmt::Debug for $typ {
      fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
      }
    }
  };
  ($typ:ident, $bytes:expr, $get:ident, $put:ident) => {
    impl<'a> Packetable<'a> for $typ {
      type Error = Error;

      fn bytes(&self) -> usize {
        $bytes
      }

      unsafe fn write_into_unchecked(&self, buf: &mut impl $crate::BufMut) {
        buf.$put((*self).into())
      }

      fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
        if buf.remaining() < $bytes { return Err(Error::InsufficientBytesReadError); }
        Ok(buf.$get().into())
      }
    }
  };
}

// Unsigned 8bits integer.

impl<'a> Packetable<'a> for &'a [u8] {
  type Error = Error;

  fn bytes(&self) -> usize {
    self.len()
  }

  unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
    buf.put(*self)
  }

  fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
    let bytes = *buf;
    buf.advance(bytes.len());
    Ok(bytes)
  }
}

// Unsigned 16bits integers.

#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display, From, Into, Not, Add, Mul)]
pub struct u16le(u16);

impl_scalar!(u16le, 2, get_u16_le, put_u16_le + Debug);

#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display, From, Into, Not, Add, Mul)]
pub struct u16be(u16);

impl_scalar!(u16be, 2, get_u16, put_u16 + Debug);

// Unsigned 32bits integers.

#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display, From, Into, Not, Add, Mul)]
pub struct u32le(u32);

impl_scalar!(u32le, 4, get_u32_le, put_u32_le + Debug);

#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display, From, Into, Not, Add, Mul)]
pub struct u32be(u32);

impl_scalar!(u32be, 4, get_u32, put_u32 + Debug);

// Unsigned 64bits integers.

#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display, From, Into, Not, Add, Mul)]
pub struct u64le(u64);

impl_scalar!(u64le, 8, get_u64_le, put_u64_le + Debug);

#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display, From, Into, Not, Add, Mul)]
pub struct u64be(u64);

impl_scalar!(u64be, 8, get_u64, put_u64 + Debug);

// Unsigned 128bits integers.

#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display, From, Into, Not, Add, Mul)]
pub struct u128le(u128);

impl_scalar!(u128le, 16, get_u128_le, put_u128_le + Debug);

#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Display, From, Into, Not, Add, Mul)]
pub struct u128be(u128);

impl_scalar!(u128be, 16, get_u128, put_u128 + Debug);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn packetable_u16le() {
    let val: u16le = 0x0102.into();
    let mut buf = [0u8; 2];
    unsafe { val.write_into_unchecked(&mut buf.as_mut()) };
    assert_eq!(buf, [2, 1]);
    assert_eq!(u16le::read_from(&mut buf.as_ref()).unwrap(), val);
  }

  #[test]
  fn packetable_u16be() {
    let val: u16be = 0x0102.into();
    let mut buf = [0u8; 2];
    unsafe { val.write_into_unchecked(&mut buf.as_mut()) };
    assert_eq!(buf, [1, 2]);
    assert_eq!(u16be::read_from(&mut buf.as_ref()).unwrap(), val);
  }

  #[test]
  fn packetable_u32le() {
    let val: u32le = 0x01020304.into();
    let mut buf = [0u8; 4];
    unsafe { val.write_into_unchecked(&mut buf.as_mut()) };
    assert_eq!(buf, [4, 3, 2, 1]);
    assert_eq!(u32le::read_from(&mut buf.as_ref()).unwrap(), val);
  }

  #[test]
  fn packetable_u32be() {
    let val: u32be = 0x01020304.into();
    let mut buf = [0u8; 4];
    unsafe { val.write_into_unchecked(&mut buf.as_mut()) };
    assert_eq!(buf, [1, 2, 3, 4]);
    assert_eq!(u32be::read_from(&mut buf.as_ref()).unwrap(), val);
  }

  #[test]
  fn packetable_u64le() {
    let val: u64le = 0x0102030405060708.into();
    let mut buf = [0u8; 8];
    unsafe { val.write_into_unchecked(&mut buf.as_mut()) };
    assert_eq!(buf, [8, 7, 6, 5, 4, 3, 2, 1]);
    assert_eq!(u64le::read_from(&mut buf.as_ref()).unwrap(), val);
  }

  #[test]
  fn packetable_u64be() {
    let val: u64be = 0x0102030405060708.into();
    let mut buf = [0u8; 8];
    unsafe { val.write_into_unchecked(&mut buf.as_mut()) };
    assert_eq!(buf, [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(u64be::read_from(&mut buf.as_ref()).unwrap(), val);
  }

  #[test]
  fn packetable_u128le() {
    let val: u128le = 0x0102030405060708090A0B0C0D0E0F10.into();
    let mut buf = [0u8; 16];
    unsafe { val.write_into_unchecked(&mut buf.as_mut()) };
    assert_eq!(buf, [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    assert_eq!(u128le::read_from(&mut buf.as_ref()).unwrap(), val);
  }

  #[test]
  fn packetable_u128be() {
    let val: u128be = 0x0102030405060708090A0B0C0D0E0F10.into();
    let mut buf = [0u8; 16];
    unsafe { val.write_into_unchecked(&mut buf.as_mut()) };
    assert_eq!(buf, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    assert_eq!(u128be::read_from(&mut buf.as_ref()).unwrap(), val);
  }

  #[test]
  fn insufficient_bytes_error() {
    let mut buf = [0u8; 2].as_ref();
    assert_eq!(u32le::read_from(&mut buf), Err(Error::InsufficientBytesReadError));
  }
}
