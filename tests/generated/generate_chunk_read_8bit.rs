fn main() {
    if bytes.remaining() < 1 {
        return Err(Error::InvalidLengthError {
            obj: "Foo".to_string(),
            wanted: 1,
            got: bytes.remaining(),
        });
    }
    let a = bytes.get_u8();
}
