#[derive(Debug)]
struct FooData {}

#[derive(Debug, Clone)]
pub struct FooPacket {
    foo: Arc<FooData>,
}

#[derive(Debug)]
pub struct FooBuilder {}

impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        true
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(Self {})
    }
    fn write_to(&self, buffer: &mut BytesMut) {}
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        let ret = 0;
        let ret = ret + 0;
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
}

impl FooBuilder {
    pub fn build(self) -> FooPacket {
        let foo = Arc::new(FooData {});
        FooPacket::new(foo).unwrap()
    }
}
