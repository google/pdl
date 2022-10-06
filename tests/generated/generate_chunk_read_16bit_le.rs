fn main() {
    if bytes.len() < 12 {
        return Err(Error::InvalidLengthError {
            obj: "Foo".to_string(),
            wanted: 12,
            got: bytes.len(),
        });
    }
    let a = u16::from_le_bytes([bytes[10], bytes[11]]);
}
