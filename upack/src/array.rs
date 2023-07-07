use std::{borrow, fmt, ops, slice};
use crate::{Packetable, Buf, BufMut};

pub struct Array<'a, T> {
  inner: ArrayImpl<'a, T>,
}

enum ArrayImpl<'a, T> {
  Lazy { len: usize, bytes: &'a [u8] },
  Borrowed { bytes: usize, items: &'a [T] },
  Owned { bytes: usize, items: Box<[T]> },
}

/// Public API.
impl<'a, T: Packetable<'a>> Array<'a, T> {
  pub fn iter(&self) -> ArrayIter<'_, 'a, T> {
    match &self.inner {
      &ArrayImpl::Lazy { bytes, .. } => ArrayIter::Lazy(bytes),
      &ArrayImpl::Borrowed { items, .. } => ArrayIter::Borrowed(items.iter()),
      ArrayImpl::Owned { items, .. } => ArrayIter::Owned(items.iter()),
    }
  }

  pub fn len(&self) -> usize {
    match &self.inner {
      &ArrayImpl::Lazy { len, .. } => len,
      &ArrayImpl::Borrowed { items, .. } => items.len(),
      ArrayImpl::Owned { items, .. } => items.len(),
    }
  }
}

/// Private API.
impl<'a, T: Packetable<'a>> Array<'a, T> {
  fn lazy(len: usize, bytes: &'a [u8]) -> Self {
    Self { inner: ArrayImpl::Lazy { len, bytes } }
  }

  fn borrowed(bytes: usize, items: &'a [T]) -> Self {
    Self { inner: ArrayImpl::Borrowed { bytes, items } }
  }

  fn owned(bytes: usize, items: Box<[T]>) -> Self {
    Self { inner: ArrayImpl::Owned { bytes, items } }
  }
}

impl<'a, T: Packetable<'a>> Clone for Array<'a, T> {
  fn clone(&self) -> Self {
    match &self.inner {
      &ArrayImpl::Lazy { len, bytes } => Array::lazy(len, bytes),
      &ArrayImpl::Borrowed { bytes, items } => Array::borrowed(bytes, items),
      ArrayImpl::Owned { bytes, items } => Array::owned(*bytes, items.clone()),
    }
  }
}

impl<'a, T: Packetable<'a>> Packetable<'a> for Array<'a, T> {
  type Error = T::Error;

  fn bytes(&self) -> usize {
    match &self.inner {
      &ArrayImpl::Lazy { bytes, .. } => bytes.len(),
      &ArrayImpl::Borrowed { bytes, .. } => bytes,
      ArrayImpl::Owned { bytes, .. } => *bytes,
    }
  }

  unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
    match &self.inner {
      &ArrayImpl::Lazy { bytes, .. } => buf.put(bytes),
      &ArrayImpl::Borrowed { items, .. } => items.iter().for_each(|x| x.write_into_unchecked(buf)),
      ArrayImpl::Owned { items, .. } => items.iter().for_each(|x| x.write_into_unchecked(buf)),
    }
  }

  fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
    let bytes = *buf;
    let mut len = 0;
    while buf.has_remaining() {
      let _ = T::read_from(buf)?;
      len += 1;
    }
    Ok(Self::lazy(len, bytes))
  }
}

pub enum ArrayIter<'s, 'a, T> {
  Lazy(&'a [u8]),
  Borrowed(slice::Iter<'a, T>),
  Owned(slice::Iter<'s, T>),
}

impl<'a> ops::Deref for Array<'a, u8> {
  type Target = [u8];

  fn deref(&self) -> &Self::Target {
    match &self.inner {
      &ArrayImpl::Lazy { bytes, .. } => bytes,
      &ArrayImpl::Borrowed { items, .. } => items,
      ArrayImpl::Owned { items, .. } => &items[..],
    }
  }
}

impl<'a, T: Packetable<'a>> From<&'a [T]> for Array<'a, T> {
  fn from(items: &'a [T]) -> Array<'a, T> {
    let bytes = items.iter().map(|x| x.bytes()).sum();
    Self::borrowed(bytes, items)
  }
}

impl<T: Packetable<'static>> From<Box<[T]>> for Array<'static, T> {
  fn from(items: Box<[T]>) -> Array<'static, T> {
    let bytes = items.iter().map(|x| x.bytes()).sum();
    Self::owned(bytes, items)
  }
}

impl<T: Packetable<'static>> From<Vec<T>> for Array<'static, T> {
  fn from(items: Vec<T>) -> Array<'static, T> {
    Self::from(items.into_boxed_slice())
  }
}

impl<T: Packetable<'static>, const N: usize> From<&'static [T; N]> for Array<'static, T> {
  fn from(items: &'static [T; N]) -> Array<'static, T> {
    Self::from(items.as_slice())
  }
}

impl<T: Packetable<'static>, const N: usize> From<[T; N]> for Array<'static, T> {
  fn from(items: [T; N]) -> Array<'static, T> {
    let bytes = items.iter().map(|x| x.bytes()).sum();
    Self::owned(bytes, Box::new(items))
  }
}

impl<'s, 'a: 's, T: Packetable<'a>> Iterator for ArrayIter<'s, 'a, T> {
  type Item = borrow::Cow<'s, T>;
  fn next(&mut self) -> Option<Self::Item> {
    match self {
      ArrayIter::Lazy(bytes) if !bytes.has_remaining() => None,
      ArrayIter::Lazy(bytes) => {
        // SAFETY: content has been validated.
        let item = unsafe { T::read_from(bytes).unwrap_unchecked() };
        Some(borrow::Cow::Owned(item))
      }
      ArrayIter::Borrowed(it) => it.next().map(|item| borrow::Cow::Borrowed(item)),
      ArrayIter::Owned(it) => it.next().map(|item| borrow::Cow::Borrowed(item)),
    }
  }
}

impl<'a, T: Packetable<'a> + fmt::Debug> fmt::Debug for Array<'a, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.iter().fold(&mut f.debug_list(), |d, item| d.entry(&item)).finish()
  }
}

impl<'a, T: Packetable<'a> + PartialEq> PartialEq for Array<'a, T> {
  fn eq(&self, other: &Self) -> bool {
    self.iter().zip(other.iter()).all(|(a, b)| a == b)
  }
}

impl<'a, T: Packetable<'a> + Eq> Eq for Array<'a, T> { }

#[cfg(test)]
mod tests {
  use crate::*;
  use super::*;

  #[test]
  fn array_of_u16_le() {
    let a: Array<'_, u16le> = Array::read_from(&mut [1, 0, 2, 0, 3, 0].as_slice()).unwrap();
    println!("{:?}", a);
  }

  #[test]
  fn array_of_u16_be() {
    let a: Array<'_, u16be> = Array::read_from(&mut [0, 1, 0, 2, 0, 3].as_slice()).unwrap();
    println!("{:?}", a);
  }

  #[test]
  fn array_of_dog() {
    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Color {
      r: u8,
      g: u8,
      b: u8,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Paw<'raw> {
      colors: Array<'raw, Color>,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Dog<'raw> {
      age: u16,
      weight: u32,
      paws: Array<'raw, Paw<'raw>>,
    }

    impl<'a> Packetable<'a> for Color {
      type Error = Error;

      fn bytes(&self) -> usize {
        3
      }

      unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.r);
        buf.put_u8(self.g);
        buf.put_u8(self.b);
      }

      fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
        if buf.remaining() < 3 {
          return Err(Error::InsufficientBytesReadError);
        }
        Ok(Self { r: buf.get_u8(), g: buf.get_u8(), b: buf.get_u8() })
      }
    }

    impl<'a> Packetable<'a> for Paw<'a> {
      type Error = Error;

      fn bytes(&self) -> usize {
        1 + self.colors.bytes()
      }

      unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.colors.bytes() as u8);
        self.colors.write_into_unchecked(buf);
      }

      fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
        if buf.remaining() < 1 {
          return Err(Error::InsufficientBytesReadError);
        }
        let colors_size = buf.get_u8() as usize;
        if (colors_size % 3) != 0 {
          // FIXME: custom error type.
          return Err(Error::InsufficientBytesReadError);
        }
        let mut colors_chunk = &buf[..colors_size];
        buf.advance(colors_size);
        let colors = Array::read_from(&mut colors_chunk)?;
        Ok(Self { colors })
      }
    }

    impl<'a> Packetable<'a> for Dog<'a> {
      type Error = Error;

      fn bytes(&self) -> usize {
        7 + self.paws.bytes()
      }

      unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
        buf.put_u16_le(self.age);
        buf.put_u32_le(self.weight);
        buf.put_u8(self.paws.bytes() as u8);
        self.paws.write_into_unchecked(buf);
      }

      fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
        if buf.remaining() < 7 {
          return Err(Error::InsufficientBytesReadError);
        }
        let age = buf.get_u16_le();
        let weight = buf.get_u32_le();
        let paws_size = buf.get_u8() as usize;
        let mut paws_chunk = &buf[..paws_size];
        buf.advance(paws_size);
        let paws = Array::read_from(&mut paws_chunk)?;
        Ok(Self { age, weight, paws })
      }
    }

    let ref_dog = Dog {
      age: 1,
      weight: 3,
      paws: Array::from([
        Paw {
          colors: Array::from([
            Color { r: 1, g: 2, b: 3, },
            Color { r: 7, g: 8, b: 9, },
          ]),
        },
        Paw {
          colors: Array::from([
            Color { r: 7, g: 8, b: 9, },
          ]),
        },
        Paw {
          colors: Array::from([
            Color { r: 4, g: 5, b: 6, },
            Color { r: 7, g: 8, b: 9, },
            Color { r: 1, g: 2, b: 3, },
          ]),
        },
        Paw {
          colors: Array::from([
            Color { r: 4, g: 5, b: 6, },
          ]),
        },
      ]),
    };
    let ref_dog_bytes = ref_dog.to_bytes();
    let parsed_dog_input = [
      1, 0, 3, 0, 0, 0,
      7 + 4 + 10 + 4,
      6, 1, 2, 3, 7, 8, 9,
      3, 7, 8, 9,
      9, 4, 5, 6, 7, 8, 9, 1, 2, 3,
      3, 4, 5, 6,
    ];
    assert_eq!(ref_dog_bytes.as_slice(), parsed_dog_input.as_slice());
    let parsed_dog = Dog::read_from(&mut parsed_dog_input.as_slice()).unwrap();
    assert_eq!(ref_dog, parsed_dog);
    let parsed_dog_bytes = parsed_dog.to_bytes();
    assert_eq!(parsed_dog_bytes, ref_dog_bytes);
  }
}
