#[derive(Debug)]
struct FooData {
    x: u8,
    y: u16,
    z: u32,
}

#[derive(Debug, Clone)]
pub struct FooPacket {
    foo: Arc<FooData>,
}

#[derive(Debug)]
pub struct FooBuilder {
    pub x: u8,
    pub y: u16,
    pub z: u32,
}

impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        if bytes.len() < 6 {
            return false;
        }
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
        let x = u8::from_le_bytes([bytes[0]]);
        if bytes.len() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                field: "y".to_string(),
                wanted: 3,
                got: bytes.len(),
            });
        }
        let y = u16::from_le_bytes([bytes[1], bytes[2]]);
        if bytes.len() < 6 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                field: "z".to_string(),
                wanted: 6,
                got: bytes.len(),
            });
        }
        let z = u32::from_le_bytes([bytes[3], bytes[4], bytes[5], 0]);
        let z = z & 0xffffff;
        Ok(Self { x, y, z })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        let x = self.x;
        buffer[0..1].copy_from_slice(&x.to_le_bytes()[0..1]);
        let y = self.y;
        buffer[1..3].copy_from_slice(&y.to_le_bytes()[0..2]);
        let z = self.z;
        let z = z & 0xffffff;
        buffer[3..6].copy_from_slice(&z.to_le_bytes()[0..3]);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 6;
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
    pub fn get_z(&self) -> u32 {
        self.foo.as_ref().z
    }
}

impl FooBuilder {
    pub fn build(self) -> FooPacket {
        let foo = Arc::new(FooData { x: self.x, y: self.y, z: self.z });
        FooPacket::new(foo).unwrap()
    }
}
