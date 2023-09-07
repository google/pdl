#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use pdl_runtime::{Error, Packet, Private};
type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooData {
    padding: u8,
    x: Vec<u32>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    #[cfg_attr(feature = "serde", serde(flatten))]
    foo: FooData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooBuilder {
    pub padding: u8,
    pub x: Vec<u32>,
}
impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 1
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 1,
                got: bytes.get().remaining(),
            });
        }
        let chunk = bytes.get_mut().get_u8();
        let x_size = (chunk & 0x1f) as usize;
        let padding = ((chunk >> 5) & 0x7);
        if bytes.get().remaining() < x_size {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: x_size,
                got: bytes.get().remaining(),
            });
        }
        if x_size % 3 != 0 {
            return Err(Error::InvalidArraySize {
                array: x_size,
                element: 3,
            });
        }
        let x_count = x_size / 3;
        let mut x = Vec::with_capacity(x_count);
        for _ in 0..x_count {
            x.push(Ok::<_, Error>(bytes.get_mut().get_uint(3) as u32)?);
        }
        Ok(Self { padding, x })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        if (self.x.len() * 3) > 0x1f {
            panic!(
                "Invalid length for {}::{}: {} > {}", "Foo", "x", (self.x.len() * 3),
                0x1f
            );
        }
        if self.padding > 0x7 {
            panic!(
                "Invalid value for {}::{}: {} > {}", "Foo", "padding", self.padding, 0x7
            );
        }
        let value = (self.x.len() * 3) as u8 | (self.padding << 5);
        buffer.put_u8(value);
        for elem in &self.x {
            buffer.put_uint(*elem as u64, 3);
        }
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        1 + self.x.len() * 3
    }
}
impl Packet for Foo {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.foo.get_size());
        self.foo.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Foo> for Bytes {
    fn from(packet: Foo) -> Self {
        packet.to_bytes()
    }
}
impl From<Foo> for Vec<u8> {
    fn from(packet: Foo) -> Self {
        packet.to_vec()
    }
}
impl Foo {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = FooData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    fn new(foo: FooData) -> Result<Self> {
        Ok(Self { foo })
    }
    pub fn get_padding(&self) -> u8 {
        self.foo.padding
    }
    pub fn get_x(&self) -> &Vec<u32> {
        &self.foo.x
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.foo.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.foo.get_size()
    }
}
impl FooBuilder {
    pub fn build(self) -> Foo {
        let foo = FooData {
            padding: self.padding,
            x: self.x,
        };
        Foo::new(foo).unwrap()
    }
}
impl From<FooBuilder> for Foo {
    fn from(builder: FooBuilder) -> Foo {
        builder.build().into()
    }
}
