#![cfg_attr(not(feature = "std"), no_std)]

// `alloc` feature.
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{boxed::Box, vec, vec::Vec};

#[cfg(test)]
mod test;

/// The `upack` crate provides a way to pack and unpack protocol packets.
/// This functionality is essential for network programming and other related tasks.
use core::{convert::Infallible, fmt, marker::PhantomData, mem, ops::Deref, slice};

/// The `Packed` trait provides a way to define how types can be serialized
/// (packed into bytes) and deserialized (unpacked from bytes). This trait lies at
/// the core of this library's functionality, allowing for a flexible,
/// type-safe manner to handle binary data.
///
/// A type implementing `Packed` is expected to encapsulate the
/// semantics of a protocol packet or any other kind of structured binary data.
/// This includes knowing its own size in bytes, and how to convert itself into bytes and
/// vice versa.
///
/// # Associated Types
///
/// `type Error`: This associated type signifies the type of error that deserialization
/// (ie. unpacked from bytes) could potentially return.
/// It must implement the `From<InsufficientBytesError>` as deserialization error could be
/// either insufficient available bytes or invalid data, the later being implementation defined.
pub trait Packed: Sized {
  /// Defines the associated error type that can be returned when attempting
  /// to unpack this type.
  type Error: fmt::Debug;

  /// Attempts to read this type from a byte buffer.
  ///
  /// This function is responsible for extracting the data for a `Packed` type
  /// from a buffer of bytes. It returns a `Result`, where an `Ok` variant indicates
  /// that the operation succeeded and includes the created `Packed` instance.
  /// The `Err` variant indicates that the operation failed, and includes an error of
  /// the associated type `Self::Error`.
  fn read_from<R: Reader>(rd: &mut R) -> Result<Self, Self::Error>;

  /// Attempts to write this type into a buffer, returning an error if there's not enough
  /// space in the buffer when the writer if fallible.
  fn write_into<W: Writer>(&self, wr: &mut W) -> Option<usize>;

  #[inline(always)]
  fn bytes(&self) -> usize {
    // default naive implementation.
    return self.write_into(&mut NoopWriter).unwrap_or(0);

    struct NoopWriter;
    impl Writer for NoopWriter {
      #[inline(always)]
      fn write(&mut self, src: &[u8]) -> Option<usize> {
        Some(src.len())
      }
    }
  }

  #[cfg(feature = "alloc")]
  fn to_bytes(&self) -> Box<[u8]> {
    let mut wr = vec![];
    let _ = wr.pack(self);
    wr.into_boxed_slice()
  }
}

/// The `Unpacker` trait provides a high-level interface to unpack data from a `Reader`.
/// The `unpack` method should always be preferred over the naive `Packed::read_from` as
/// it generate far more optimized code specific to each writer.
pub trait Unpacker: Reader {
  /// Unpack a `Packed` struct.
  fn unpack<P: Packed>(&mut self) -> Result<P, P::Error> {
    // default naive implementation.
    self.read_from()
  }
}

/// The `Packer` trait provides a high-level interface to pack data into a `Writer`.
/// The `pack` method should always be preferred over the naive `Packed::write_into` as
/// it generate far more optimized code specific to each reader.
pub trait Packer: Writer {
  /// Pack a `Packed` struct.
  fn pack(&mut self, p: &impl Packed) -> Option<usize> {
    // default naive implementation.
    p.write_into(self)
  }
}

/// The `Reader` trait provides a low-level interface to read a certain amount of bytes from
/// different data-structures.
pub trait Reader: Sized {
  /// Read `n` bytes. Return a `InsufficientBytesError` when there are not enough bytes to
  /// read from.
  fn read_chunk<'a>(&mut self, n: usize) -> Option<&'a [u8]>;

  /// Read all remaining bytes from this reader. The result can be empty.
  fn read_all<'a>(&mut self) -> &'a [u8];

  /// Read a compile-time known amount of `N` bytes. Return a `InsufficientBytesError` when
  /// there are not enough bytes to read from.
  #[inline(always)]
  fn read_fixed<'a, const N: usize>(&mut self) -> Option<&'a [u8; N]> {
    // default implementation.
    #[cfg(any(debug_assertions, not(feature = "unsafe")))]
    return self.read_chunk(N)?.try_into().ok();
    #[cfg(all(not(debug_assertions), feature = "unsafe"))]
    {
      let chunk = self.read_chunk(N)?;
      // SAFETY: `self.read_chunk(N)?` has returned `N` bytes, do not double check.
      Some(unsafe { &*(chunk.as_ptr() as *const [u8; N]) })
    }
  }

  /// Convenient wrapper to read a `Packed` structure from this reader.
  #[inline(always)]
  fn read_from<P: Packed>(&mut self) -> Result<P, P::Error> {
    P::read_from(self)
  }
}

/// The `Writer` trait provides a low-level interface to write a certain amount of bytes into
/// different data-structures.
pub trait Writer: Sized {
  /// Write the `src` buffer into this writer. Return a `Self::Error` in case of error,
  /// which implementation specific.
  fn write(&mut self, src: &[u8]) -> Option<usize>;
}

/// The `ChunkWriter` trait is an extension of a `Writer` that can return chunks of writable data.
/// This trait is not usable from the `Packed` implementation point-of-view.
/// It is used for high-level `pack`/`unpack` with the `Packer` and `Unpacker` traits.
pub trait ChunkWriter: Writer {
  /// Write `n` bytes. Return a `Self::Error` in case of error,
  /// which implementation specific.
  fn write_chunk(&mut self, n: usize) -> Option<&mut [u8]>;

  /// Write a compile-time known amount of `N` bytes.
  #[inline(always)]
  fn write_fixed<const N: usize>(&mut self) -> Option<&mut [u8; N]> {
    // default implementation.
    #[cfg(any(debug_assertions, not(feature = "unsafe")))]
    return self.write_chunk(N)?.try_into().ok();
    #[cfg(all(not(debug_assertions), feature = "unsafe"))]
    {
      let chunk = self.write_chunk(N)?;
      // SAFETY: `self.write_chunk(N)?` has returned `N` bytes, do not double check.
      Some(unsafe { &mut *(chunk.as_mut_ptr() as *mut [u8; N]) })
    }
  }
}

pub trait Endianness: Copy {}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NativeEndian;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LittleEndian;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BigEndian;

impl Endianness for NativeEndian {}
impl Endianness for LittleEndian {}
impl Endianness for BigEndian {}

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
/// use upack::{Array, InsufficientBytesError, Packed, Reader, Writer, Unpacker};
///
/// #[derive(Debug, Clone, Eq, PartialEq)]
/// struct Color { r: u8, g: u8, b: u8, }
///
/// impl Packed for Color {
///   type Error = InsufficientBytesError;
///
///   fn read_from<R: Reader>(rd: &mut R) -> Result<Self, Self::Error> {
///     Ok(Self { r: rd.read_from()?, g: rd.read_from()?, b: rd.read_from()? })
///   }
///
///   fn write_into<W: Writer>(&self, wr: &mut W) -> Option<usize> {
///     Some(self.r.write_into(wr)? + self.g.write_into(wr)? + self.b.write_into(wr)?)
///   }
/// }
///
/// // 'Lazy' mode
/// let lazy_arr = (&b"\x01\x02\x03\x04\x04\x06"[..]).unpack::<Array<'_, Color>>().unwrap();
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

pub enum ArrayItem<'a, T> {
  Borrowed(&'a T),
  Owned(T),
}

/// General crate error type which represents the following cases:
/// - When there are not enough bytes to read from a buffer.
/// - When there are not enough space in a buffer to write into.
#[derive(Eq, PartialEq)]
pub struct InsufficientBytesError;

/// `InfallibleRw` is a wrapper around `read_from` and `write_into` operations that never fails.
pub struct InfallibleRw<'rw, T> {
  // Mutable a reference to a `Reader` or a `Writer`.
  inner: &'rw mut T,
}

// `InsufficientBytesError`'s impls ----------------------------------------------------------------

impl fmt::Debug for InsufficientBytesError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("insufficient bytes")
  }
}

// `InfallibleRw`'s impls --------------------------------------------------------------------------

#[inline(always)]
fn unreachable__() -> ! {
  #[cfg(any(debug_assertions, not(feature = "unsafe")))]
  unreachable!();
  #[cfg(all(not(debug_assertions), feature = "unsafe"))]
  // SAFETY: None, the caller is responsible for safety.
  unsafe {
    core::hint::unreachable_unchecked()
  };
}

impl<'rw, R: Reader> InfallibleRw<'rw, R> {
  #[inline(always)]
  #[cfg(not(feature = "unsafe"))]
  pub fn unpack<P: Packed>(rd: &'rw mut R) -> P {
    P::read_from(&mut Self { inner: rd }).unwrap()
  }

  #[inline(always)]
  #[cfg(feature = "unsafe")]
  pub unsafe fn unpack<P: Packed>(rd: &'rw mut R) -> P {
    match P::read_from(&mut Self { inner: rd }) {
      Ok(p) => p,
      _ => unreachable__(),
    }
  }
}

impl<'rw, R: Reader> Reader for InfallibleRw<'rw, R> {
  #[inline(always)]
  fn read_chunk<'a>(&mut self, n: usize) -> Option<&'a [u8]> {
    match self.inner.read_chunk(n) {
      Some(chunk) => Some(chunk),
      _ => unreachable__(),
    }
  }

  #[inline(always)]
  fn read_all<'a>(&mut self) -> &'a [u8] {
    self.inner.read_all()
  }
}

impl<'rw, W: Writer> InfallibleRw<'rw, W> {
  #[inline(always)]
  #[cfg(not(feature = "unsafe"))]
  pub fn pack(wr: &'rw mut W, p: &impl Packed) -> usize {
    p.write_into(&mut Self { inner: wr }).unwrap()
  }

  #[inline(always)]
  #[cfg(feature = "unsafe")]
  pub unsafe fn pack(wr: &'rw mut W, p: &impl Packed) -> usize {
    match p.write_into(&mut Self { inner: wr }) {
      Some(n) => n,
      _ => unreachable__(),
    }
  }
}

impl<'rw, W: Writer> Writer for InfallibleRw<'rw, W> {
  #[inline(always)]
  fn write(&mut self, src: &[u8]) -> Option<usize> {
    match self.inner.write(src) {
      Some(n) => Some(n),
      _ => unreachable__(),
    }
  }
}

// scalar's impls ----------------------------------------------------------------------------------

macro_rules! impl_scalar {
  () => {
    impl_scalar!(impl Float);
    impl_scalar!(impl Unsigned for u16, u32, u64, u128);
    impl_scalar!(impl Signed   for u16, u32, u64, u128);
  };
  (impl Float) => {
    impl_scalar!(struct Float);
    impl_scalar!(impl Float<4, f32>, |x| x.to_ne_bytes(), |x| f32::from_ne_bytes(x));
    impl_scalar!(impl Float<8, f64>, |x| x.to_ne_bytes(), |x| f64::from_ne_bytes(x));
  };
  (impl $int:ident for $i2:ident, $i4:ident, $i8:ident, $i16:ident) => {
    impl_scalar!(struct $int);
    impl_scalar!(impl $int<2,  $i2>,  const |x| x.to_ne_bytes(), const |x| $i2::from_ne_bytes(x));
    impl_scalar!(impl $int<4,  $i4>,  const |x| x.to_ne_bytes(), const |x| $i4::from_ne_bytes(x));
    impl_scalar!(impl $int<8,  $i8>,  const |x| x.to_ne_bytes(), const |x| $i8::from_ne_bytes(x));
    impl_scalar!(impl $int<16, $i16>, const |x| x.to_ne_bytes(), const |x| $i16::from_ne_bytes(x));
    impl_scalar!(
      impl $int<3, $i4>,
      const |x| {
        #[cfg(target_endian = "little")]
        let [a, b, c, _] = x.to_ne_bytes();
        #[cfg(target_endian = "big")]
        let [_, a, b, c] = x.to_ne_bytes();
        [a, b, c]
      },
      const |x| {
        let [a, b, c] = x;
        #[cfg(target_endian = "little")]
        let x = [a, b, c, 0];
        #[cfg(target_endian = "big")]
        let x = [0, a, b, c];
        $i4::from_ne_bytes(x)
      }
    );
  };
  (struct $name:ident) => {
    #[repr(transparent)]
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    pub struct $name<const BYTES: usize, E = NativeEndian> {
      native: [u8; BYTES],
      endianness: PhantomData<*const E>,
    }

    impl<const N: usize> $name<N, NativeEndian> {
      #[inline(always)]
      pub fn from_bytes(value: [u8; N]) -> Self {
        Self { native: value, endianness: PhantomData }
      }

      #[inline(always)]
      pub fn into_bytes(self) -> [u8; N] {
        self.native
      }
    }

    impl<const N: usize> $name<N, LittleEndian> {
      #[inline(always)]
      #[allow(unused_mut)]
      pub fn from_bytes(mut value: [u8; N]) -> Self {
        #[cfg(not(target_endian = "little"))]
        value.reverse();
        Self { native: value, endianness: PhantomData }
      }

      #[inline(always)]
      #[allow(unused_mut)]
      pub fn into_bytes(mut self) -> [u8; N] {
        #[cfg(not(target_endian = "little"))]
        self.native.reverse();
        self.native
      }
    }

    impl<const N: usize> $name<N, BigEndian> {
      #[inline(always)]
      #[allow(unused_mut)]
      pub fn from_bytes(mut value: [u8; N]) -> Self {
        #[cfg(not(target_endian = "big"))]
        value.reverse();
        Self { native: value, endianness: PhantomData }
      }

      #[inline(always)]
      #[allow(unused_mut)]
      pub fn into_bytes(mut self) -> [u8; N] {
        #[cfg(not(target_endian = "big"))]
        self.native.reverse();
        self.native
      }
    }
  };
  (impl $int:ident <$N:tt, $typ:ident>, $($fc:ident)? |$f:ident| $from:expr, $($ic:ident)? |$i:ident| $into:expr) => {
    impl<E: Endianness> $int<$N, E> {
      #[inline(always)]
      pub $($fc)? fn new($f: $typ) -> Self {
        Self { native: $from, endianness: PhantomData }
      }

      #[inline(always)]
      pub $($ic)? fn into_inner(self) -> $typ {
        let $i = self.native;
        $into
      }
    }

    impl<E: Endianness> From<$typ> for $int<$N, E> {
      #[inline(always)]
      fn from(value: $typ) -> Self {
        Self::new(value)
      }
    }

    impl<E: Endianness> Into<$typ> for $int<$N, E> {
      #[inline(always)]
      fn into(self) -> $typ {
        self.into_inner()
      }
    }

    impl<E: Endianness> fmt::Display for $int<$N, E> {
      #[inline(always)]
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.into_inner().fmt(f)
      }
    }

    impl<E: Endianness> fmt::Debug for $int<$N, E> {
      #[inline(always)]
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.into_inner().fmt(f)
      }
    }

    impl_scalar!(impl Packed for $int<$N>);
  };
  (impl Packed for $int:ident <$N:tt, $E:ident>) => {
    impl Packed for $int<$N, $E> {
      type Error = InsufficientBytesError;

      #[inline(always)]
      fn read_from<R: Reader>(rd: &mut R) -> Result<Self, Self::Error> {
        Ok(Self::from_bytes(*rd.read_fixed().ok_or(InsufficientBytesError)?))
      }

      #[inline(always)]
      fn write_into<W: Writer>(&self, wr: &mut W) -> Option<usize> {
        wr.write(&self.into_bytes())
      }

      #[inline(always)]
      fn bytes(&self) -> usize {
        $N
      }
    }
  };
  (impl Packed for $typ:ident <$N:tt>) => {
    impl_scalar!(impl Packed for $typ<$N, NativeEndian>);
    impl_scalar!(impl Packed for $typ<$N, LittleEndian>);
    impl_scalar!(impl Packed for $typ<$N, BigEndian>);
  };
}

impl_scalar!();

macro_rules! impl_scalars {
  (impl Packed for [$($typ:ident),+]) => {
    $(impl_scalars!(impl Packed for $typ);)+
  };
  (impl Packed for $typ:ident) => {
    impl_scalars!(impl Packed for $typ [$typ::from_ne_bytes, to_ne_bytes]);
  };
  (impl Packed for $typ:ident $(<$generic:tt>)? [$from:path, $into:ident]) => {
    impl Packed for $typ $(<$generic>)? {
      type Error = InsufficientBytesError;

      #[inline(always)]
      fn read_from<R: Reader>(rd: &mut R) -> Result<Self, Self::Error> {
        Ok($from(*rd.read_fixed().ok_or(InsufficientBytesError)?).into())
      }

      #[inline(always)]
      fn write_into<W: Writer>(&self, wr: &mut W) -> Option<usize> {
        wr.write(&self.$into())
      }

      #[inline(always)]
      fn bytes(&self) -> usize {
        mem::size_of::<Self>()
      }
    }
  };
}

impl_scalars!(impl Packed for [i16, u16, i32, u32, i64, u64, i128, u128, f32, f64, usize, isize]);

impl Packed for u8 {
  type Error = InsufficientBytesError;

  #[inline(always)]
  fn read_from<R: Reader>(rd: &mut R) -> Result<Self, Self::Error> {
    Ok(rd.read_fixed::<1>().ok_or(InsufficientBytesError)?[0])
  }

  #[inline(always)]
  fn write_into<W: Writer>(&self, wr: &mut W) -> Option<usize> {
    wr.write(&[*self])
  }

  #[inline(always)]
  fn bytes(&self) -> usize {
    1
  }
}

impl Packed for &[u8] {
  type Error = Infallible;

  #[inline(always)]
  fn read_from<R: Reader>(rd: &mut R) -> Result<Self, Self::Error> {
    Ok(rd.read_all())
  }

  #[inline(always)]
  fn write_into<W: Writer>(&self, wr: &mut W) -> Option<usize> {
    wr.write(self)
  }

  #[inline(always)]
  fn bytes(&self) -> usize {
    self.len()
  }
}

impl<R: Reader + Copy> Unpacker for R {
  #[inline(always)]
  fn unpack<P: Packed>(&mut self) -> Result<P, P::Error> {
    let _ = P::read_from(&mut self.clone())?;
    #[cfg(not(feature = "unsafe"))]
    return Ok(InfallibleRw::unpack(self));
    #[cfg(feature = "unsafe")]
    // SAFETY: error handling has been done above, this shall not fail.
    Ok(unsafe { InfallibleRw::unpack(self) })
  }
}

impl Reader for &[u8] {
  #[inline(always)]
  fn read_chunk<'a>(&mut self, n: usize) -> Option<&'a [u8]> {
    let len = self.len();
    if n > len {
      return None;
    }
    #[cfg(not(feature = "unsafe"))]
    let (chunk, rest) = self.split_at(n);
    #[cfg(feature = "unsafe")]
    // SAFETY: bounds check has already been done above.
    let (chunk, rest) = unsafe {
      let ptr = self.as_ptr();
      (slice::from_raw_parts(ptr, n), slice::from_raw_parts(ptr.add(n), len - n))
    };
    *self = rest;
    Some(chunk)
  }

  #[inline(always)]
  fn read_all<'a>(&mut self) -> &'a [u8] {
    let len = self.len();
    #[cfg(not(feature = "unsafe"))]
    let (chunk, rest) = self.split_at(len);
    #[cfg(feature = "unsafe")]
    // SAFETY: bounds check has already been done above.
    let (chunk, rest) = unsafe {
      let ptr = self.as_ptr();
      (slice::from_raw_parts(ptr, len), slice::from_raw_parts(ptr.add(len), 0))
    };
    *self = rest;
    chunk
  }
}

impl Writer for &mut [u8] {
  #[inline(always)]
  fn write(&mut self, src: &[u8]) -> Option<usize> {
    let (n, len) = (src.len(), self.len());
    if n > len {
      None?;
    }
    #[cfg(not(feature = "unsafe"))]
    // Lifetime dance taken from `impl Write for &mut [u8]`.
    let rest = {
      let (chunk, rest) = mem::replace(self, &mut []).split_at_mut(n);
      chunk.copy_from_slice(src);
      rest
    };
    #[cfg(feature = "unsafe")]
    // SAFETY: bounds check has already been done above.
    let rest = unsafe {
      let ptr = self.as_mut_ptr();
      core::ptr::copy_nonoverlapping(src.as_ptr(), ptr, n);
      slice::from_raw_parts_mut(ptr.add(n), len - n)
    };
    *self = rest;
    Some(n)
  }
}

impl ChunkWriter for &mut [u8] {
  #[inline(always)]
  fn write_chunk(&mut self, n: usize) -> Option<&mut [u8]> {
    let len = self.len();
    if n > len {
      None?;
    }
    #[cfg(not(feature = "unsafe"))]
    // Lifetime dance taken from `impl Write for &mut [u8]`.
    let (chunk, rest) = mem::replace(self, &mut []).split_at_mut(n);
    #[cfg(feature = "unsafe")]
    // SAFETY: bounds check has already been done above.
    let (chunk, rest) = unsafe {
      let ptr = self.as_mut_ptr();
      (slice::from_raw_parts_mut(ptr, n), slice::from_raw_parts_mut(ptr.add(n), len - n))
    };
    *self = rest;
    Some(chunk)
  }
}

#[cfg(feature = "alloc")]
impl Writer for Vec<u8> {
  #[inline(always)]
  fn write(&mut self, src: &[u8]) -> Option<usize> {
    self.extend(src);
    Some(src.len())
  }
}

#[cfg(feature = "alloc")]
impl ChunkWriter for Vec<u8> {
  #[inline(always)]
  fn write_chunk(&mut self, n: usize) -> Option<&mut [u8]> {
    #[cfg(not(feature = "unsafe"))]
    fn infallible_write_chunk(v: &mut Vec<u8>, n: usize) -> &mut [u8] {
      let start = v.len();
      v.extend(core::iter::repeat(0u8).take(n));
      &mut v[start..]
    }

    #[inline(never)]
    #[cfg(feature = "unsafe")]
    fn infallible_write_chunk(v: &mut Vec<u8>, n: usize) -> &mut [u8] {
      v.reserve(n);
      // SAFETY: `n` bytes has been reserved above.
      unsafe {
        let ptr = v.as_mut_ptr().add(v.len());
        v.set_len(n);
        slice::from_raw_parts_mut(ptr, n)
      }
    }

    Some(infallible_write_chunk(self, n))
  }
}

impl<W: ChunkWriter> Packer for W {
  #[inline(always)]
  fn pack(&mut self, p: &impl Packed) -> Option<usize> {
    let mut chunk = self.write_chunk(p.bytes())?;
    #[cfg(not(feature = "unsafe"))]
    return Some(InfallibleRw::pack(&mut chunk, p));
    #[cfg(feature = "unsafe")]
    // SAFETY: error handling has been done above, this shall not fail.
    Some(unsafe { InfallibleRw::pack(&mut chunk, p) })
  }
}

// `Array` impls -----------------------------------------------------------------------------------

impl<'a, T: Packed> Array<'a, T> {
  /// Returns an iterator over the `Array`.
  ///
  /// This method will provide an iterator depending on how the `Array` is constructed, which can
  /// be in one of three ways: Lazily from bytes, from borrowed items, or from owned items.
  ///
  /// When iterating, for 'Lazy' arrays, bytes are converted to `Packed` items on the fly. For
  /// 'Borrowed' and 'Owned' arrays, items are returned directly.
  ///
  /// # Example
  /// ```
  /// use upack::{Unsigned, LittleEndian, Array, Packed, Unpacker};
  ///
  /// // 'Lazy' mode
  /// let lazy_arr = (&b"\x01\x00\x02\x00\x03\x00"[..])
  ///   .unpack::<Array<'_, Unsigned<2, LittleEndian>>>()
  ///   .unwrap();
  /// assert_eq!(lazy_arr.len(), 3);
  /// for item in lazy_arr.iter() {
  ///   println!("{:?}", item);
  /// }
  ///
  /// let items = lazy_arr.iter().map(|x| x.clone()).collect::<Vec<_>>();
  /// assert_eq!(&items[..], &[1.into(), 2.into(), 3.into()]);
  ///
  /// // 'Borrowed' mode
  /// let borrowed_arr = Array::from(&items[..]);
  /// assert_eq!(borrowed_arr.len(), 3);
  /// for item in borrowed_arr.iter() {
  ///   println!("{:?}", item);
  /// }
  ///
  /// assert_eq!(lazy_arr, borrowed_arr);
  ///
  /// // 'Owned' mode
  /// #[cfg(feature = "alloc")]
  /// {
  ///   let owned_arr = Array::from(items);
  ///   assert_eq!(owned_arr.len(), 3);
  ///   for item in owned_arr.iter() {
  ///     println!("{:?}", item);
  ///   }
  ///
  ///   assert_eq!(lazy_arr, owned_arr);
  /// }
  /// ```
  pub fn iter(&self) -> ArrayIter<'_, 'a, T> {
    match &self.inner {
      &ArrayImpl::Lazy { bytes, .. } => ArrayIter::Lazy(bytes),
      &ArrayImpl::Borrowed { items, .. } => ArrayIter::Borrowed(items.iter()),
      #[cfg(feature = "alloc")]
      ArrayImpl::Owned { items, .. } => ArrayIter::Borrowed(items.iter()),
    }
  }

  /// Returns the length of the `Array`.
  ///
  /// For 'Lazy' arrays, this is the number of `Packed` items that can be constructed from the
  /// bytes. For 'Borrowed' and 'Owned' arrays, this is the length of the underlying items array.
  ///
  /// # Example
  /// ```
  /// use upack::{Unsigned, LittleEndian, Array, Packed, Unpacker};
  ///
  /// // 'Lazy' mode
  /// let lazy_arr = (&b"\x01\x00\x02\x00\x03\x00"[..])
  ///   .unpack::<Array<'_, Unsigned<2, LittleEndian>>>()
  ///   .unwrap();
  /// assert_eq!(lazy_arr.len(), 3);
  ///
  /// let items = lazy_arr.iter().map(|x| x.clone()).collect::<Vec<_>>();
  ///
  /// // 'Borrowed' mode
  /// let borrowed_arr = Array::from(&items[..]);
  /// assert_eq!(borrowed_arr.len(), 3);
  ///
  /// // 'Owned' mode
  /// #[cfg(feature = "alloc")]
  /// {
  ///   let owned_arr = Array::from(items);
  ///   assert_eq!(owned_arr.len(), 3);
  /// }
  /// ```
  pub fn len(&self) -> usize {
    match &self.inner {
      &ArrayImpl::Lazy { len, .. } => len,
      &ArrayImpl::Borrowed { items, .. } => items.len(),
      #[cfg(feature = "alloc")]
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
///   Items are not immediately converted from bytes to `Packed` items; instead, this conversion
///   is done lazily, as items are requested. This approach can offer performance benefits when
///   dealing with large sequences of items. The byte slice and a factory function to convert bytes
///   to `Packed` items are stored.
///
/// - `Borrowed`: The `Borrowed` variant represents an `Array` that has been constructed from a
///   borrowed slice of `Packed` items. This allows us to create an `Array` without copying
///   data, but the downside is that the original data needs to be kept alive for as long as the
///   `Array` exists.
///
/// - `Owned`: The `Owned` variant represents an `Array` that owns its items. When an `Array` is
///   created in this mode, the items are copied into the `Array`. This can use more memory than
///   the `Lazy` or `Borrowed` modes, but it allows for modification of items and doesn't require
///   keeping original data alive.
enum ArrayImpl<'a, T> {
  Lazy {
    len: usize,
    bytes: &'a [u8],
  },
  Borrowed {
    bytes: usize,
    items: &'a [T],
  },
  #[cfg(feature = "alloc")]
  Owned {
    bytes: usize,
    items: Box<[T]>,
  },
}

// Implementing helper functions for the Array struct.
impl<'a, T: Packed> Array<'a, T> {
  /// Constructs an Array from raw bytes, meant for lazy loading.
  #[inline(always)]
  fn lazy(len: usize, bytes: &'a [u8]) -> Self {
    ArrayImpl::lazy(len, bytes).into()
  }

  /// Constructs an Array from a slice of `Packed` items. Array doesn't own the items,
  /// hence it's borrowed.
  #[inline(always)]
  fn borrowed(items: &'a [T]) -> Self {
    ArrayImpl::borrowed(items.iter().map(|item| item.bytes()).sum(), items).into()
  }

  /// Constructs an Array from owned `Packed` items.
  /// Array owns the items.
  #[cfg(feature = "alloc")]
  #[inline(always)]
  fn owned(items: Box<[T]>) -> Self {
    ArrayImpl::owned(items.iter().map(|item| item.bytes()).sum(), items).into()
  }
}

impl<'a, T: Packed> ArrayImpl<'a, T> {
  /// Constructs an Array from raw bytes, meant for lazy loading.
  #[inline(always)]
  fn lazy(len: usize, bytes: &'a [u8]) -> Self {
    ArrayImpl::Lazy { len, bytes }
  }

  /// Constructs an Array from a slice of `Packed` items. Array doesn't own the items,
  /// hence it's borrowed.
  #[inline(always)]
  fn borrowed(bytes: usize, items: &'a [T]) -> Self {
    ArrayImpl::Borrowed { bytes, items }
  }

  /// Constructs an Array from owned `Packed` items.
  /// Array owns the items.
  #[cfg(feature = "alloc")]
  #[inline(always)]
  fn owned(bytes: usize, items: Box<[T]>) -> Self {
    ArrayImpl::Owned { bytes, items }
  }
}

impl<'a, T> From<ArrayImpl<'a, T>> for Array<'a, T> {
  #[inline(always)]
  fn from(inner: ArrayImpl<'a, T>) -> Self {
    Self { inner }
  }
}

impl<T> Deref for ArrayItem<'_, T> {
  type Target = T;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    match self {
      ArrayItem::Borrowed(item) => item,
      ArrayItem::Owned(item) => &item,
    }
  }
}

// Implementing Clone for Array struct.
impl<T: Packed + Clone> Clone for Array<'_, T> {
  fn clone(&self) -> Self {
    // Depending on the Array type (Lazy, Borrowed, Owned), different cloning
    // mechanisms are applied.
    match &self.inner {
      &ArrayImpl::Lazy { len, bytes } => Array::lazy(len, bytes),
      &ArrayImpl::Borrowed { items, bytes } => ArrayImpl::borrowed(bytes, items).into(),
      #[cfg(feature = "alloc")]
      ArrayImpl::Owned { items, bytes } => ArrayImpl::owned(*bytes, items.clone()).into(),
    }
  }
}

// Implementing `Packed` trait for Array struct. This trait encapsulates operations to
// serialize/deserialize the struct.
impl<T: Packed> Packed for Array<'_, T> {
  type Error = T::Error;

  #[inline(always)]
  fn read_from<R: Reader>(rd: &mut R) -> Result<Self, Self::Error> {
    let bytes = rd.read_all();
    let (mut it, mut len) = (bytes, 0);
    while !it.is_empty() {
      let _ = T::read_from(&mut it)?;
      len += 1;
    }
    Ok(Array::lazy(len, bytes))
  }

  #[inline(always)]
  fn write_into<W: Writer>(&self, wr: &mut W) -> Option<usize> {
    let mut bytes = 0;
    for item in self.iter() {
      bytes += item.write_into(wr)?;
    }
    Some(bytes)
  }

  #[inline(always)]
  fn bytes(&self) -> usize {
    match &self.inner {
      ArrayImpl::Lazy { bytes, .. } => bytes.len(),
      ArrayImpl::Borrowed { bytes, .. } => *bytes,
      #[cfg(feature = "alloc")]
      ArrayImpl::Owned { bytes, .. } => *bytes,
    }
  }
}

pub enum ArrayIter<'s, 'a, T> {
  Lazy(&'a [u8]),
  Borrowed(slice::Iter<'s, T>),
}

// Implementing the Deref trait, which allows instances of this type to be treated as a slice.
impl Deref for Array<'_, u8> {
  type Target = [u8];

  // Implementing the deref method to return the byte slice.
  fn deref(&self) -> &Self::Target {
    // Different strategies based on the Array type.
    match &self.inner {
      &ArrayImpl::Lazy { bytes, .. } => bytes,
      &ArrayImpl::Borrowed { items, .. } => items,
      #[cfg(feature = "alloc")]
      ArrayImpl::Owned { items, .. } => &items[..],
    }
  }
}

// Various From implementations allow for creating an Array from different types.
// These implementations are self-explanatory and follow a similar pattern.

impl<'a, T: Packed> From<&'a [T]> for Array<'a, T> {
  fn from(items: &'a [T]) -> Array<'a, T> {
    Self::borrowed(items)
  }
}

impl<T: Packed, const N: usize> From<&'static [T; N]> for Array<'static, T> {
  fn from(items: &'static [T; N]) -> Array<'static, T> {
    Self::from(items.as_slice())
  }
}

#[cfg(feature = "alloc")]
impl<T: Packed> From<Box<[T]>> for Array<'static, T> {
  fn from(items: Box<[T]>) -> Array<'static, T> {
    Self::owned(items)
  }
}

#[cfg(feature = "alloc")]
impl<T: Packed> From<Vec<T>> for Array<'static, T> {
  fn from(items: Vec<T>) -> Array<'static, T> {
    Self::from(items.into_boxed_slice())
  }
}

#[cfg(feature = "alloc")]
impl<T: Packed, const N: usize> From<[T; N]> for Array<'static, T> {
  fn from(items: [T; N]) -> Array<'static, T> {
    Self::owned(Box::new(items))
  }
}

impl<'s, T: Packed> Iterator for ArrayIter<'s, '_, T> {
  type Item = ArrayItem<'s, T>;
  fn next(&mut self) -> Option<Self::Item> {
    match self {
      ArrayIter::Lazy(bytes) if bytes.is_empty() => None,
      ArrayIter::Lazy(bytes) => {
        #[cfg(not(feature = "unsafe"))]
        let item = InfallibleRw::unpack(bytes);
        #[cfg(feature = "unsafe")]
        // SAFETY: error handling has been done at array construction, this shall not fail.
        let item = unsafe { InfallibleRw::unpack(bytes) };
        Some(ArrayItem::Owned(item))
      }
      ArrayIter::Borrowed(it) => it.next().map(|item| ArrayItem::Borrowed(item)),
    }
  }
}

// Implementing Debug trait for Array, which is useful for debugging and printing the Array.

impl<T: Packed + fmt::Debug> fmt::Debug for ArrayItem<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.deref().fmt(f)
  }
}

impl<T: Packed + fmt::Debug> fmt::Debug for Array<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Array is printed as a list of debug representations of its items.
    self.iter().fold(&mut f.debug_list(), |d, item| d.entry(item.deref())).finish()
  }
}

// Implementing PartialEq and Eq traits to allow for Array comparison.

impl<T: Packed + PartialEq> PartialEq for Array<'_, T> {
  fn eq(&self, other: &Self) -> bool {
    // Arrays are considered equal if all their items are equal.
    self.iter().zip(other.iter()).all(|(a, b)| a.deref() == b.deref())
  }
}

impl<T: Packed + Eq> Eq for Array<'_, T> {}
