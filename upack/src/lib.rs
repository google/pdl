/// The `upack` crate provides a way to pack and unpack protocol packets.
/// This functionality is essential for network programming and other related tasks.

// Re-exports from the `bytes` crate, for convenient access to its Buf and BufMut traits.
pub use bytes::{BufMut, Buf};
use thiserror::Error;

// Re-exporting the array module.
mod array;
pub use array::Array;

// Re-exporting the scalar module.
mod scalar;
pub use scalar::{u16le, u16be, u32le, u32be, u64le, u64be, u128le, u128be};

/// The `Packetable` trait provides a way to define how types can be serialized
/// (packed into bytes) and deserialized (unpacked from bytes). This trait lies at
/// the core of this library's functionality, allowing for a flexible,
/// type-safe manner to handle binary data.
///
/// A type implementing `Packetable` is expected to encapsulate the
/// semantics of a protocol packet or any other kind of structured binary data.
/// This includes knowing its own size in bytes, and how to convert itself into bytes and
/// vice versa.
///
/// # Type Parameters
///
/// 'a: The lifetime parameter 'a allows the data to have references to other
/// data, tying the lifetime of the packet to the lifetime of the referenced data.
///
/// # Associated Types
///
/// `type Error`: This associated type signifies the type of error that methods of this
/// trait could potentially return. It must implement the `From<Error>` trait to be
/// compatible with the general error type of this library.
///
/// # Safety
///
/// Some methods in this trait, like `write_into_unchecked()`, are marked `unsafe`. This
/// implies that these methods guarantee safety only when certain conditions are met.
/// For `write_into_unchecked()`, it's necessary to ensure that the buffer provided has
/// enough space. Failure to meet these preconditions could lead to Undefined Behavior.
///
/// # Design Considerations
///
/// The `Packetable` trait was designed to be simple, yet powerful. Each method has a
/// clear responsibility: `bytes()` provides the size, `write_into_unchecked()` writes
/// into a buffer without checking the space, `read_from()` reads from a buffer, and
/// so on. This distribution of responsibilities makes the trait flexible, easy to
/// implement, and easy to use.
///
/// One decision worth discussing is the use of the `unsafe` keyword. `unsafe` is used
/// in Rust to signify that a piece of code has potential safety risks and it's the
/// responsibility of the programmer to ensure safety. The `write_into_unchecked()`
/// method is marked `unsafe` because it writes bytes into a buffer without checking
/// if the buffer has enough space. This design allows for more performant code by
/// skipping unnecessary checks when the programmer is certain of safety.
///
pub trait Packetable<'a>: Sized + Clone {
  /// Defines the associated error type that can be returned when attempting 
  /// to pack or unpack this type.
  type Error: From<Error>;

  /// Returns the number of bytes that this type would occupy when packed.
  ///
  /// This method is critical for the functioning of the `Packetable` trait, as it allows
  /// the system to correctly allocate buffer space when writing data. If the size reported
  /// by this function is incorrect, it could lead to a variety of problems, such as buffer
  /// overflows or underflows, data corruption, or program crashes. Therefore, it is essential
  /// to ensure the returned size accurately reflects the actual byte size of the instance when
  /// serialized.
  ///
  /// The method is expected to compute and return the size dynamically, which allows for
  /// `Packetable` instances to change their size according to the state they're in, such as
  /// a variable-length packet.
  ///
  /// # Examples
  ///
  /// Consider a type `Packet` implementing `Packetable`, where each instance has a fixed size of
  /// 6 bytes:
  /// ```
  /// use upack::{Buf, BufMut, Error, Packetable};
  ///
  /// #[derive(Debug, Clone, Eq, PartialEq)]
  /// struct Packet(u32, u16);
  ///
  /// impl<'a> Packetable<'a> for Packet {
  ///   type Error = Error;
  ///
  ///   fn bytes(&self) -> usize { 6 }
  ///   unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
  ///     unimplemented!()
  ///   }
  ///   fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
  ///     unimplemented!()
  ///   }
  /// }
  /// ```
  ///
  /// Now consider a different type `VarPacket` which has a variable length:
  /// ```
  /// use upack::{Buf, BufMut, Error, Packetable, Array, u16le};
  ///
  /// #[derive(Debug, Clone, Eq, PartialEq)]
  /// struct Packet<'raw>(u32, Array<'raw, u16le>);
  ///
  /// impl<'a> Packetable<'a> for Packet<'a> {
  ///   type Error = Error;
  ///
  ///   fn bytes(&self) -> usize { 4 + self.1.bytes() }
  ///   unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
  ///     unimplemented!()
  ///   }
  ///   fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
  ///     unimplemented!()
  ///   }
  /// }
  /// ```
  ///
  /// As shown in the examples above, `bytes()` can handle both fixed and variable-length packets.
  fn bytes(&self) -> usize;

  /// Attempts to write this type into a buffer, returning an error if there's not enough
  /// space in the buffer.
  ///
  /// This method is the safe counterpart of `write_into_unchecked`. It performs a
  /// check to ensure that the buffer has enough space to accommodate the bytes
  /// being written. If the buffer does not have enough space, it returns an
  /// `Error::InsufficientBytesWriteError`.
  ///
  /// This function is intended to be used in most cases when you want to write an instance
  /// of a `Packetable` type into a byte buffer. It provides safety checks and error
  /// handling to prevent buffer overflows and other issues.
  ///
  /// # Example
  ///
  /// ```
  /// use upack::{Buf, BufMut, Error, Packetable};
  ///
  /// #[derive(Debug, Clone, Eq, PartialEq)]
  /// struct Packet(u32, u16);
  ///
  /// impl<'a> Packetable<'a> for Packet {
  ///   type Error = Error;
  ///
  ///   fn bytes(&self) -> usize { 6 }
  ///   unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
  ///     buf.put_u32_le(self.0);
  ///     buf.put_u16_le(self.1);
  ///   }
  ///   fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
  ///     if buf.len() < 6 {
  ///       return Err(Error::InsufficientBytesReadError);
  ///     }
  ///     Ok(Packet(buf.get_u32_le(), buf.get_u16_le()))
  ///   }
  /// }
  ///
  /// let mut buf = [0u8; 6];
  /// let packet = Packet(42, 24);
  /// assert_eq!(packet.write_into(&mut &mut buf[..]), Ok(()));
  /// ```
  fn write_into(&self, buf: &mut impl BufMut) -> Result<(), Self::Error> {
    if buf.remaining_mut() < self.bytes() {
      return Err(Self::Error::from(Error::InsufficientBytesWriteError));
    }
    // SAFETY: destination buffer has been checked.
    unsafe { Ok(Self::write_into_unchecked(self, buf)) }
  }

  /// Writes the bytes of this type into a buffer without checking if there's enough space.
  ///
  /// # Safety
  ///
  /// This method is unsafe because it does not perform any checks to ensure that the buffer
  /// is large enough to accommodate the bytes being written. This function is not intended
  /// to be called directly, and should only be used internally within implementations of
  /// `Packetable` trait. Users should use the safe `write_into` method instead, which
  /// performs necessary checks and handles errors appropriately.
  ///
  /// # Note
  ///
  /// The `write_into_unchecked` method forms part of the underlying mechanism that allows
  /// `Packetable` types to be serialized into bytes. It is an internal part of the
  /// serialization process and it's unsafe due to lack of runtime checks. Using it
  /// incorrectly can lead to buffer overflows, and potentially, undefined behaviour.
  unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut);

  /// Attempts to read this type from a byte buffer.
  ///
  /// This function is responsible for extracting the data for a `Packetable` type
  /// from a buffer of bytes. It returns a `Result`, where an `Ok` variant indicates
  /// that the operation succeeded and includes the created `Packetable` instance.
  /// The `Err` variant indicates that the operation failed, and includes an error of
  /// the associated type `Self::Error`.
  ///
  /// The `read_from` method is part of the core functionality of the `Packetable` trait,
  /// enabling types to be deserialized from a sequence of bytes. The exact behavior of
  /// this function, including how it reads bytes from the buffer and how it handles
  /// errors, depends on the specific implementation for the `Packetable` type.
  ///
  /// # Example
  ///
  /// ```
  /// use upack::{Buf, BufMut, Error, Packetable};
  ///
  /// #[derive(Debug, Clone, Eq, PartialEq)]
  /// struct Packet(u32, u16);
  ///
  /// impl<'a> Packetable<'a> for Packet {
  ///   type Error = Error;
  ///
  ///   fn bytes(&self) -> usize { 6 }
  ///   unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
  ///     buf.put_u32_le(self.0);
  ///     buf.put_u16_le(self.1);
  ///   }
  ///   fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
  ///     if buf.len() < 6 {
  ///       return Err(Error::InsufficientBytesReadError);
  ///     }
  ///     Ok(Packet(buf.get_u32_le(), buf.get_u16_le()))
  ///   }
  /// }
  ///
  /// let buf = [42u8, 0, 0, 0, 24u8, 0];
  /// let packet = Packet::read_from(&mut &buf[..]);
  /// assert_eq!(packet, Ok(Packet(42, 24)));
  /// ```
  fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error>;

  /// Attempts to construct this type from a byte slice, and returns the remaining 
  /// part of the slice.
  ///
  /// This function consumes a byte slice, reads bytes to form an instance of the `Packetable` type,
  /// and then returns that instance along with the unused portion of the byte slice.
  ///
  /// This method is particularly useful for deserializing multiple `Packetable` instances
  /// from the same byte slice.
  ///
  /// # Example
  ///
  /// ```
  /// use upack::{Buf, BufMut, Error, Packetable};
  ///
  /// #[derive(Debug, Clone, Eq, PartialEq)]
  /// struct Packet(u32);
  ///
  /// impl<'a> Packetable<'a> for Packet {
  ///   type Error = Error;
  ///
  ///   fn bytes(&self) -> usize { 4 }
  ///   unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
  ///     buf.put_u32_le(self.0);
  ///   }
  ///   fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
  ///     if buf.len() < 4 {
  ///       return Err(Error::InsufficientBytesReadError);
  ///     }
  ///     Ok(Packet(buf.get_u32_le()))
  ///   }
  /// }
  ///
  /// let buf = [42u8, 0, 0, 0, 24u8, 0, 0, 0];
  /// let (packet, rest) = Packet::from_bytes(&buf[..]).unwrap();
  /// assert_eq!(packet, Packet(42));
  /// assert_eq!(rest, [24u8, 0, 0, 0]);
  /// ```
  fn from_bytes(mut bytes: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
    Self::read_from(&mut bytes).map(|res| (res, bytes))
  }

  /// Packs this type into bytes and returns the result as a `Vec<u8>`.
  ///
  /// This function is responsible for transforming an instance of a `Packetable` type into a `Vec<u8>`.
  /// It does so by allocating a new `Vec<u8>` and using the `write_into` method to serialize the instance into the vector.
  ///
  /// # Example
  ///
  /// ```
  /// use upack::{Buf, BufMut, Error, Packetable};
  ///
  /// #[derive(Clone)]
  /// struct Packet(u32);
  ///
  /// impl<'a> Packetable<'a> for Packet {
  ///   type Error = Error;
  ///
  ///   fn bytes(&self) -> usize { 4 }
  ///   unsafe fn write_into_unchecked(&self, buf: &mut impl BufMut) {
  ///     buf.put_u32_le(self.0);
  ///   }
  ///   fn read_from(buf: &mut &'a [u8]) -> Result<Self, Self::Error> {
  ///     if buf.len() < 4 {
  ///       return Err(Error::InsufficientBytesReadError);
  ///     }
  ///     Ok(Packet(buf.get_u32_le()))
  ///   }
  /// }
  ///
  /// let packet = Packet(42);
  /// let bytes = packet.to_bytes();
  /// assert_eq!(bytes, vec![42u8, 0, 0, 0]);
  /// ```
  fn to_bytes(&self) -> Vec<u8> {
    let mut res = vec![];
    let _ = self.write_into(&mut res);
    res
  }
}

/// The `Error` enum represents all the possible errors that can occur
/// when packing and unpacking protocol packets.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {
  /// This error represents the case when there are not enough bytes to read from a buffer.
  #[error("insufficient bytes to read from")]
  InsufficientBytesReadError,

  /// This error represents the case when there is not enough space in a buffer to write into.
  #[error("insufficient bytes to write into")]
  InsufficientBytesWriteError,
}
