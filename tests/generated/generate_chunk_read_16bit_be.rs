fn main() {
    if bytes.len() < 12 {
        return Err(Error::InvalidLengthError {
            obj: "Foo".to_string(),
            wanted: 12,
            got: bytes.len(),
        });
    }
    let a = u16::from_be_bytes([bytes[10], bytes[11]]);
}
