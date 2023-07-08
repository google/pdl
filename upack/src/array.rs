//! This module provides `Array`, a container for `Packetable` types.
//! It's useful when dealing with sequences of packetable items.

use std::{borrow, fmt, ops, slice};
use crate::{Packetable, Buf, BufMut};

/// `Array` is a flexible and efficient container for storing sequences of items.
///
/// The `Array` struct can be used to store items in one of three modes: Lazy, Borrowed, or Owned.
/// The item type must implement the `Packetable` trait, allowing for conversion between bytes and
/// the item type.
///
/// In 'Lazy' mode, the `Array` is constructed from a byte slice, and items are converted from bytes
/// on demand. This is efficient when you have large sequences and don't want to convert all items at once.
///
/// In 'Borrowed' mode, the `Array` holds a reference to an existing array of items. This is useful
/// when you want to avoid data copying.
///
/// In 'Owned' mode, the `Array` owns its items. This is useful when you need to modify the items, or
/// when the original data is not available for the lifetime of the `Array`.
///
/// # Example
/// ```
/// use upack::{Array, Buf, BufMut, Error, Packetable};
///
/// #[derive(Debug, Clone, Eq, PartialEq)]
/// struct Color { r: u8, g: u8, b: u8, }
///
/// impl<'a> Packetable<'a> for Color {
///   type Error = Error;
///
///   fn bytes(&self) -> usize { 3 }///
///   unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
///     buf.put_u8(self.r);
///     buf.put_u8(self.g);
///     buf.put_u8(self.b);
///   }///
///   fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
///     if buf.remaining() < 3 {
///       return Err(Error::InsufficientBytesReadError);
///     }
///     Ok(Self { r: buf.get_u8(), g: buf.get_u8(), b: buf.get_u8() })
///   }
/// }
///
/// // 'Lazy' mode
/// let (lazy_arr, _) = Array::<'_, Color>::from_bytes(b"\x01\x02\x03\x04\x04\x06").unwrap();
/// assert_eq!(lazy_arr.len(), 2);
/// for item in lazy_arr.iter() {
///   println!("{:?}", item);
/// }
/// // Color { r: 0, g: 1, b: 2 }
/// // Color { r: 4, g: 5, b: 6 }
///
/// // 'Borrowed' mode
/// const COLORS: [Color; 2] = [
///   Color { r: 0, g: 1, b: 2 },
///   Color { r: 4, g: 5, b: 6 },
/// ];
///
/// let borrowed_arr = Array::from(&COLORS);
/// assert_eq!(borrowed_arr.len(), 2);
/// for item in borrowed_arr.iter() {
///   println!("{:?}", item);
/// }
/// // Color { r: 0, g: 1, b: 2 }
/// // Color { r: 4, g: 5, b: 6 }
/// ```
///
/// `Array` provides several methods, like `iter()` and `len()`, for manipulating and accessing its items.
/// Also, as `Array` implements `Packetable`, it can be converted to and from bytes.
/// ```
pub struct Array<'a, T> {
  inner: ArrayImpl<'a, T>,
}

impl<'a, T: Packetable<'a>> Array<'a, T> {
  /// Returns an iterator over the `Array`.
  ///
  /// This method will provide an iterator depending on how the `Array` is constructed, which can
  /// be in one of three ways: Lazily from bytes, from borrowed items, or from owned items.
  ///
  /// When iterating, for 'Lazy' arrays, bytes are converted to `Packetable` items on the fly. For
  /// 'Borrowed' and 'Owned' arrays, items are returned directly.
  ///
  /// # Example
  /// ```
  /// use upack::{Array, Buf, BufMut, Error, Packetable, u16le};
  ///
  /// // 'Lazy' mode
  /// let (lazy_arr, _) = Array::<'_, u16le>::from_bytes(b"\x01\x00\x02\x00\x03\x00").unwrap();
  /// assert_eq!(lazy_arr.len(), 3);
  /// for item in lazy_arr.iter() {
  ///   println!("{:?}", item);
  /// }
  ///
  /// // 'Borrowed' mode
  /// let items = lazy_arr.iter().map(|x| x.into_owned()).collect::<Vec<_>>();
  /// let borrowed_arr = Array::from(&items[..]);
  /// assert_eq!(borrowed_arr.len(), 3);
  /// for item in borrowed_arr.iter() {
  ///   println!("{:?}", item);
  /// }
  ///
  /// assert_eq!(lazy_arr, borrowed_arr);
  ///
  /// // 'Owned' mode
  /// let owned_arr = Array::from(items);
  /// assert_eq!(owned_arr.len(), 3);
  /// for item in owned_arr.iter() {
  ///   println!("{:?}", item);
  /// }
  ///
  /// assert_eq!(lazy_arr, owned_arr);
  /// ```
  pub fn iter(&self) -> ArrayIter<'_, 'a, T> {
    match &self.inner {
      &ArrayImpl::Lazy { bytes, .. } => ArrayIter::Lazy(bytes),
      &ArrayImpl::Borrowed { items, .. } => ArrayIter::Borrowed(items.iter()),
      ArrayImpl::Owned { items, .. } => ArrayIter::Owned(items.iter()),
    }
  }

  /// Returns the length of the `Array`.
  ///
  /// For 'Lazy' arrays, this is the number of `Packetable` items that can be constructed from the
  /// bytes. For 'Borrowed' and 'Owned' arrays, this is the length of the underlying items array.
  ///
  /// # Example
  /// ```
  /// use upack::{Array, Buf, BufMut, Error, Packetable, u16le};
  ///
  /// // 'Lazy' mode
  /// let (lazy_arr, _) = Array::<'_, u16le>::from_bytes(b"\x01\x00\x02\x00\x03\x00").unwrap();
  /// assert_eq!(lazy_arr.len(), 3);
  ///
  /// // 'Borrowed' mode
  /// let items = lazy_arr.iter().map(|x| x.into_owned()).collect::<Vec<_>>();
  /// let borrowed_arr = Array::from(&items[..]);
  /// assert_eq!(borrowed_arr.len(), 3);
  ///
  /// // 'Owned' mode
  /// let owned_arr = Array::from(items);
  /// assert_eq!(owned_arr.len(), 3);
  /// ```
  pub fn len(&self) -> usize {
    match &self.inner {
      &ArrayImpl::Lazy { len, .. } => len,
      &ArrayImpl::Borrowed { items, .. } => items.len(),
      ArrayImpl::Owned { items, .. } => items.len(),
    }
  }
}

/// `ArrayImpl` is the internal representation of an `Array`.
///
/// This enum is the core of the `Array` struct.
/// It represents the three ways an `Array` can be constructed: Lazily from bytes, from borrowed
/// items, or from owned items.
///
/// # Variants
///
/// - `Lazy`: The `Lazy` variant represents an `Array` that is constructed from a byte slice.
///   Items are not immediately converted from bytes to `Packetable` items; instead, this conversion
///   is done lazily, as items are requested. This approach can offer performance benefits when
///   dealing with large sequences of items. The byte slice and a factory function to convert bytes
///   to `Packetable` items are stored.
///
/// - `Borrowed`: The `Borrowed` variant represents an `Array` that has been constructed from a
///   borrowed slice of `Packetable` items. This allows us to create an `Array` without copying
///   data, but the downside is that the original data needs to be kept alive for as long as the
///   `Array` exists.
///
/// - `Owned`: The `Owned` variant represents an `Array` that owns its items. When an `Array` is
///   created in this mode, the items are copied into the `Array`. This can use more memory than
///   the `Lazy` or `Borrowed` modes, but it allows for modification of items and doesn't require
///   keeping original data alive.
enum ArrayImpl<'a, T> {
  Lazy { len: usize, bytes: &'a [u8] },
  Borrowed { bytes: usize, items: &'a [T] },
  Owned { bytes: usize, items: Box<[T]> },
}

// Implementing helper functions for the Array struct.
impl<'a, T: Packetable<'a>> Array<'a, T> {
  /// Constructs an Array from raw bytes, meant for lazy loading.
  fn lazy(len: usize, bytes: &'a [u8]) -> Self {
    Self { inner: ArrayImpl::Lazy { len, bytes } }
  }

  /// Constructs an Array from a slice of Packetable items. Array doesn't own the items,
  /// hence it's borrowed.
  fn borrowed(bytes: usize, items: &'a [T]) -> Self {
    Self { inner: ArrayImpl::Borrowed { bytes, items } }
  }

  /// Constructs an Array from owned Packetable items.
  /// Array owns the items.
  fn owned(bytes: usize, items: Box<[T]>) -> Self {
    Self { inner: ArrayImpl::Owned { bytes, items } }
  }
}

// Implementing Clone for Array struct.
impl<'a, T: Packetable<'a>> Clone for Array<'a, T> {
  fn clone(&self) -> Self {
    // Depending on the Array type (Lazy, Borrowed, Owned), different cloning
    // mechanisms are applied.
    match &self.inner {
      &ArrayImpl::Lazy { len, bytes } => Array::lazy(len, bytes),
      &ArrayImpl::Borrowed { bytes, items } => Array::borrowed(bytes, items),
      ArrayImpl::Owned { bytes, items } => Array::owned(*bytes, items.clone()),
    }
  }
}

// Implementing Packetable trait for Array struct. This trait encapsulates operations to
// serialize/deserialize the struct.
impl<'a, T: Packetable<'a>> Packetable<'a> for Array<'a, T> {
  type Error = T::Error;

  // Returns the byte size of the Array.
  fn bytes(&self) -> usize {
    // Depending on the Array type, different strategies are used to count the bytes.
    match &self.inner {
      &ArrayImpl::Lazy { bytes, .. } => bytes.len(),
      &ArrayImpl::Borrowed { bytes, .. } => bytes,
      ArrayImpl::Owned { bytes, .. } => *bytes,
    }
  }

  // Writes Array into a buffer. Different strategies are applied based on the Array type.
  unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
    match &self.inner {
      &ArrayImpl::Lazy { bytes, .. } => buf.put(bytes),
      &ArrayImpl::Borrowed { items, .. } => items.iter().for_each(|x| x.write_into_unchecked(buf)),
      ArrayImpl::Owned { items, .. } => items.iter().for_each(|x| x.write_into_unchecked(buf)),
    }
  }

  // Reads an Array from a buffer.
  fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
    let bytes = *buf;
    let mut len = 0;
    // Loop over the buffer to count the number of Packetable items.
    while buf.has_remaining() {
      let _ = T::read_from(buf)?;
      len += 1;
    }
    // Create a Lazy array based on the obtained byte slice and length.
    Ok(Self::lazy(len, bytes))
  }
}

pub enum ArrayIter<'s, 'a, T> {
  Lazy(&'a [u8]),
  Borrowed(slice::Iter<'a, T>),
  Owned(slice::Iter<'s, T>),
}

// Implementing the Deref trait, which allows instances of this type to be treated as a slice.
impl<'a> ops::Deref for Array<'a, u8> {
  type Target = [u8];

  // Implementing the deref method to return the byte slice.
  fn deref(&self) -> &Self::Target {
    // Different strategies based on the Array type.
    match &self.inner {
      &ArrayImpl::Lazy { bytes, .. } => bytes,
      &ArrayImpl::Borrowed { items, .. } => items,
      ArrayImpl::Owned { items, .. } => &items[..],
    }
  }
}

// Various From implementations allow for creating an Array from different types.
// These implementations are self-explanatory and follow a similar pattern.

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

// Implementing Debug trait for Array, which is useful for debugging and printing the Array.
impl<'a, T: Packetable<'a> + fmt::Debug> fmt::Debug for Array<'a, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Array is printed as a list of debug representations of its items.
    self.iter().fold(&mut f.debug_list(), |d, item| d.entry(&item)).finish()
  }
}

// Implementing PartialEq and Eq traits to allow for Array comparison.

impl<'a, T: Packetable<'a> + PartialEq> PartialEq for Array<'a, T> {
  fn eq(&self, other: &Self) -> bool {
    // Arrays are considered equal if all their items are equal.
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
