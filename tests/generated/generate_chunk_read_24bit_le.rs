fn main() {
    if bytes.remaining() < 3 {
        return Err(Error::InvalidLengthError {
            obj: "Foo".to_string(),
            wanted: 3,
            got: bytes.remaining(),
        });
    }
    let a = bytes.get_uint_le(3) as u32;
}
