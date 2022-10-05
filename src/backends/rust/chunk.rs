use crate::backends::rust::field::Field;

/// A chunk of field.
///
/// While fields can have arbitrary widths, a chunk is always an
/// integer number of bytes wide.
pub struct Chunk<'a> {
    fields: &'a [Field],
}

impl Chunk<'_> {
    /// Construct a new `Chunk` from the fields.
    pub fn new(fields: &[Field]) -> Chunk {
        // TODO(mgeisler): check that the width % 8 == 0?
        Chunk { fields }
    }

    /// Return the width in bits.
    pub fn get_width(self) -> usize {
        self.fields.iter().map(|field| field.get_width()).sum()
    }
}
