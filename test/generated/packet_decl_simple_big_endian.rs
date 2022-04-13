#[derive(Debug)]
struct FooData {
    x: u8,
    y: u16,
}

#[derive(Debug, Clone)]
pub struct FooPacket {
    foo: Arc<FooData>,
}

#[derive(Debug)]
pub struct FooBuilder {
    pub x: u8,
    pub y: u16,
}

impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 1 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                field: "x".to_string(),
                wanted: 1,
                got: bytes.len(),
            });
        }
        let x = u8::from_be_bytes([bytes[0]]);
        if bytes.len() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                field: "y".to_string(),
                wanted: 3,
                got: bytes.len(),
            });
        }
        let y = u16::from_be_bytes([bytes[1], bytes[2]]);
        Ok(Self { x, y })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let x = self.x;
        buffer[0..1].copy_from_slice(&x.to_be_bytes()[0..1]);
        let y = self.y;
        buffer[1..3].copy_from_slice(&y.to_be_bytes()[0..2]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 3;
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
    pub fn get_x(&self) -> u8 {
        self.foo.as_ref().x
    }
    pub fn get_y(&self) -> u16 {
        self.foo.as_ref().y
    }
}

impl FooBuilder {
    pub fn build(self) -> FooPacket {
        let foo = Arc::new(FooData { x: self.x, y: self.y });
        FooPacket::new(foo).unwrap()
    }
}
