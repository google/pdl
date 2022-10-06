fn main() {
    if bytes.len() < 15 {
        return Err(Error::InvalidLengthError {
            obj: "Foo".to_string(),
            wanted: 15,
            got: bytes.len(),
        });
    }
    let chunk =
        u64::from_be_bytes([0, 0, 0, bytes[10], bytes[11], bytes[12], bytes[13], bytes[14]]);
    let a = chunk as u16;
    let b = ((chunk >> 16) & 0xffffff) as u32;
}
