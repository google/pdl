#[derive(Debug)]
struct FooData {
    a: u8,
    b: u8,
    c: u8,
    d: u32,
    e: u16,
    f: u8,
}

#[derive(Debug, Clone)]
pub struct FooPacket {
    foo: Arc<FooData>,
}

#[derive(Debug)]
pub struct FooBuilder {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u32,
    pub e: u16,
    pub f: u8,
}

impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 7 {
            return false;
        }
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                field: "a".to_string(),
                wanted: 1,
                got: bytes.len(),
            });
        }
        if bytes.len() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                field: "b".to_string(),
                wanted: 2,
                got: bytes.len(),
            });
        }
        let chunk = u16::from_be_bytes([bytes[0], bytes[1]]);
        let a = (chunk & 0x7) as u8;
        let b = (chunk >> 3) as u8;
        let c = ((chunk >> 11) & 0x1f) as u8;
        if bytes.len() < 5 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                field: "d".to_string(),
                wanted: 5,
                got: bytes.len(),
            });
        }
        let d = u32::from_be_bytes([0, bytes[2], bytes[3], bytes[4]]);
        if bytes.len() < 7 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                field: "e".to_string(),
                wanted: 7,
                got: bytes.len(),
            });
        }
        let chunk = u16::from_be_bytes([bytes[5], bytes[6]]);
        let e = (chunk & 0xfff);
        let f = ((chunk >> 12) & 0xf) as u8;
        Ok(Self { a, b, c, d, e, f })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let chunk = 0;
        let chunk = chunk | ((self.a as u16) & 0x7);
        let chunk = chunk | ((self.b as u16) << 3);
        let chunk = chunk | (((self.c as u16) & 0x1f) << 11);
        buffer[0..2].copy_from_slice(&chunk.to_be_bytes()[0..2]);
        let d = self.d;
        buffer[2..5].copy_from_slice(&d.to_be_bytes()[0..3]);
        let chunk = 0;
        let chunk = chunk | (self.e & 0xfff);
        let chunk = chunk | (((self.f as u16) & 0xf) << 12);
        buffer[5..7].copy_from_slice(&chunk.to_be_bytes()[0..2]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 7;
        ret
    }
}

impl Packet for FooPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::new();
        buffer.resize(self.foo.get_total_size(), 0);
        self.foo.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<FooPacket> for Bytes {
    fn from(packet: FooPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<FooPacket> for Vec<u8> {
    fn from(packet: FooPacket) -> Self {
        packet.to_vec()
    }
}

impl FooPacket {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self::new(Arc::new(FooData::parse(bytes)?)).unwrap())
    }
    fn new(root: Arc<FooData>) -> std::result::Result<Self, &'static str> {
        let foo = root;
        Ok(Self { foo })
    }
    pub fn get_a(&self) -> u8 {
        self.foo.as_ref().a
    }
    pub fn get_b(&self) -> u8 {
        self.foo.as_ref().b
    }
    pub fn get_c(&self) -> u8 {
        self.foo.as_ref().c
    }
    pub fn get_d(&self) -> u32 {
        self.foo.as_ref().d
    }
    pub fn get_e(&self) -> u16 {
        self.foo.as_ref().e
    }
    pub fn get_f(&self) -> u8 {
        self.foo.as_ref().f
    }
}

impl FooBuilder {
    pub fn build(self) -> FooPacket {
        let foo =
            Arc::new(FooData { a: self.a, b: self.b, c: self.c, d: self.d, e: self.e, f: self.f });
        FooPacket::new(foo).unwrap()
    }
}
