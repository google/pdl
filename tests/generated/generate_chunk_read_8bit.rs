fn main() {
    if bytes.len() < 11 {
        return Err(Error::InvalidLengthError {
            obj: "Foo".to_string(),
            wanted: 11,
            got: bytes.len(),
        });
    }
    let a = u8::from_be_bytes([bytes[10]]);
}
