fn main() {
    if bytes.remaining() < 2 {
        return Err(Error::InvalidLengthError {
            obj: "Foo".to_string(),
            wanted: 2,
            got: bytes.remaining(),
        });
    }
    let a = bytes.get_u16_le();
}
