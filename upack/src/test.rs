#[cfg(feature = "std")]
use rand::Rng;

use crate::*;

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

impl<'a> Packed<'a> for Color {
  type Error = InsufficientBytesError;

  fn read_from<R: Reader<'a>>(rd: &mut R) -> Result<Self, Self::Error> {
    Ok(Self { r: rd.read_from()?, g: rd.read_from()?, b: rd.read_from()? })
  }
  fn write_into<W: Writer>(&self, wr: &mut W) -> Option<usize> {
    Some(self.r.write_into(wr)? + self.g.write_into(wr)? + self.b.write_into(wr)?)
  }
}

impl<'a> Packed<'a> for Paw<'a> {
  type Error = InsufficientBytesError;

  fn read_from<R: Reader<'a>>(rd: &mut R) -> Result<Self, Self::Error> {
    Ok(Self {
      colors: {
        let bytes: u16 = rd.read_from()?;
        let mut bytes = rd.read_chunk(bytes as usize).ok_or(InsufficientBytesError)?;
        bytes.read_from()?
      },
    })
  }

  fn write_into<W: Writer>(&self, wr: &mut W) -> Option<usize> {
    Some((self.colors.bytes() as u16).write_into(wr)? + self.colors.write_into(wr)?)
  }
}

impl<'a> Packed<'a> for Dog<'a> {
  type Error = InsufficientBytesError;

  fn read_from<R: Reader<'a>>(rd: &mut R) -> Result<Self, Self::Error> {
    Ok(Self {
      age: rd.read_from()?,
      weight: rd.read_from()?,
      paws: {
        let bytes: u16 = rd.read_from()?;
        let mut bytes = rd.read_chunk(bytes as usize).ok_or(InsufficientBytesError)?;
        bytes.read_from()?
      },
    })
  }

  fn write_into<W: Writer>(&self, wr: &mut W) -> Option<usize> {
    Some(
      self.age.write_into(wr)?
        + self.weight.write_into(wr)?
        + (self.paws.bytes() as u16).write_into(wr)?
        + self.paws.write_into(wr)?,
    )
  }
}

#[test]
fn array_of_u16_le() {
  let a: Array<'_, le<u16>> = Array::read_from(&mut [1, 0, 2, 0, 3, 0].as_slice()).unwrap();
  assert_eq!(a.iter().map(|x| x.into_inner()).sum::<u16>(), 6);
}

#[test]
fn array_of_u16_be() {
  let a: Array<'_, be<u16>> = Array::read_from(&mut [0, 1, 0, 2, 0, 3].as_slice()).unwrap();
  assert_eq!(a.iter().map(|x| x.into_inner()).sum::<u16>(), 6);
}

// Create random data for testing.
#[cfg(feature = "std")]
fn generate_random_colors() -> Vec<Color> {
  let mut rng = rand::thread_rng();
  let mut colors = vec![];
  for _ in 0..rng.gen_range(0..64) {
    colors.push(Color { r: rng.gen(), g: rng.gen(), b: rng.gen() });
  }
  colors
}

// Create random data for testing.
#[cfg(feature = "std")]
fn generate_random_paws() -> Vec<Paw<'static>> {
  let mut rng = rand::thread_rng();
  let mut paws = vec![];
  for _ in 0..rng.gen_range(0..64) {
    paws.push(Paw { colors: Array::from(generate_random_colors()) });
  }
  paws
}

// Create random data for testing.
#[cfg(feature = "std")]
fn generate_random_dog() -> Dog<'static> {
  let mut rng = rand::thread_rng();
  Dog { age: rng.gen::<u16>(), weight: rng.gen::<u32>(), paws: Array::from(generate_random_paws()) }
}

// Test the serialization and deserialization of a Color.
#[cfg(feature = "std")]
#[test]
fn color_packetable() {
  let color = Color { r: rand::random(), g: rand::random(), b: rand::random() };
  let mut buf = vec![0u8; color.bytes()].into_boxed_slice();
  color.write_into(&mut buf.as_mut()).unwrap();
  let expected_color = Color::read_from(&mut buf.as_ref()).unwrap();
  assert_eq!(color, expected_color);
  assert_eq!(&color.to_bytes()[..], buf.as_ref());
}

// Test the serialization and deserialization of a Paw.
#[test]
#[cfg(feature = "std")]
fn paw_packetable() {
  let paw = Paw { colors: Array::from(generate_random_colors()) };
  let mut buf = vec![0u8; paw.bytes()].into_boxed_slice();
  paw.write_into(&mut buf.as_mut()).unwrap();
  let expected_paw = Paw::read_from(&mut buf.as_ref()).unwrap();
  assert_eq!(paw, expected_paw);
  assert_eq!(&paw.to_bytes()[..], buf.as_ref());
}

// Test the serialization and deserialization of a Dog.
#[test]
#[cfg(feature = "std")]
fn dog_packetable() {
  let dog = generate_random_dog();
  let mut buf = vec![0u8; dog.bytes()].into_boxed_slice();
  dog.write_into(&mut buf.as_mut()).unwrap();
  let expected_dog = Dog::read_from(&mut buf.as_ref()).unwrap();
  assert_eq!(dog, expected_dog);
  assert_eq!(&dog.to_bytes()[..], buf.as_ref());
}

// Test the serialization and deserialization of an array of Dogs.
#[test]
#[cfg(feature = "std")]
fn array_of_dog() {
  let mut dogs = vec![];
  for _ in 0..512 {
    dogs.push(generate_random_dog());
  }
  let array = Array::from(dogs);
  let mut buf = vec![0u8; array.bytes()];
  array.write_into(&mut buf.as_mut_slice()).unwrap();
  let expected_array = Array::<Dog>::read_from(&mut buf.as_slice()).unwrap();
  assert_eq!(array, expected_array);
}

// Test that an error is returned when the buffer is too small.
#[test]
fn insufficient_bytes_error_color() {
  let buf = [0u8; 1].as_ref();
  assert_eq!(Color::read_from(&mut buf.as_ref()), Err(InsufficientBytesError));
}

// Test that an error is returned when the buffer is too small.
#[test]
fn insufficient_bytes_error_paw() {
  let buf = [].as_ref();
  assert_eq!(Paw::read_from(&mut buf.as_ref()), Err(InsufficientBytesError));
}

// Test that an error is returned when the buffer is too small.
#[test]
fn insufficient_bytes_error_dog() {
  let buf = [0u8; 1].as_ref();
  assert_eq!(Dog::read_from(&mut buf.as_ref()), Err(InsufficientBytesError));
}

// Test that an error is returned when the buffer is too small for an array of Dogs.
#[test]
fn insufficient_bytes_error_array_of_dog() {
  let buf = [0u8; 1].as_ref();
  assert_eq!(Array::<Dog>::read_from(&mut buf.as_ref()), Err(InsufficientBytesError));
}

// Test that an error is returned when the buffer is too small for a Dog.
#[test]
#[cfg(feature = "std")]
fn insufficient_bytes_error_dog_write() {
  let dog = generate_random_dog();
  let mut buf = vec![0u8; 0].into_boxed_slice(); // zero-sized buffer
  assert_eq!(dog.write_into(&mut buf.as_mut()), None);
}

// Test that an error is returned when the buffer is too small for a Paw.
#[test]
#[cfg(feature = "std")]
fn insufficient_bytes_error_paw_write() {
  let paw = Paw { colors: Array::from(generate_random_colors()) };
  let mut buf = vec![0u8; 0].into_boxed_slice(); // zero-sized buffer
  assert_eq!(paw.write_into(&mut buf.as_mut()), None);
}

// Test that an error is returned when the buffer is too small for a Color.
#[test]
#[cfg(feature = "std")]
fn insufficient_bytes_error_color_write() {
  let color = Color { r: rand::random(), g: rand::random(), b: rand::random() };
  let mut buf = vec![0u8; 0].into_boxed_slice(); // zero-sized buffer
  assert_eq!(color.write_into(&mut buf.as_mut()), None);
}

// Test that an error is returned when the buffer is too small for an array of Dogs.
#[test]
#[cfg(feature = "std")]
fn insufficient_bytes_error_array_of_dog_write() {
  let mut dogs = vec![];
  for _ in 0..512 {
    dogs.push(generate_random_dog());
  }
  let array = Array::from(dogs);
  let mut buf = vec![0u8; 0].into_boxed_slice(); // zero-sized buffer
  assert_eq!(array.write_into(&mut buf.as_mut()), None);
}
