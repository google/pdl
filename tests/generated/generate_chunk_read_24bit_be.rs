fn main() {
    if bytes.len() < 13 {
        return Err(Error::InvalidLengthError {
            obj: "Foo".to_string(),
            wanted: 13,
            got: bytes.len(),
        });
    }
    let a = u32::from_be_bytes([0, bytes[10], bytes[11], bytes[12]]);
}
