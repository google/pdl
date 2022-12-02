fn main() {
    if bytes.remaining() < 5 {
        return Err(Error::InvalidLengthError {
            obj: "Foo".to_string(),
            wanted: 5,
            got: bytes.remaining(),
        });
    }
    let chunk = bytes.get_uint(5) as u64;
    let a = chunk as u16;
    let b = ((chunk >> 16) & 0xffffff) as u32;
}
