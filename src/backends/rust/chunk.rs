use crate::backends::rust::field::Field;
use crate::backends::rust::get_field_range;
use quote::{format_ident, quote};

/// A chunk of field.
///
/// While fields can have arbitrary widths, a chunk is always an
/// integer number of bytes wide.
pub struct Chunk<'a> {
    pub fields: &'a [Field],
}

impl Chunk<'_> {
    /// Construct a new `Chunk` from the fields.
    pub fn new(fields: &[Field]) -> Chunk {
        // TODO(mgeisler): check that the width % 8 == 0?
        Chunk { fields }
    }

    /// Generate a name for this chunk.
    ///
    /// The name is `"chunk"` if there is more than one field.
    pub fn get_name(&self) -> proc_macro2::Ident {
        match self.fields {
            [field] => field.get_ident(),
            _ => format_ident!("chunk"),
        }
    }

    /// Return the width in bits.
    pub fn get_width(&self) -> usize {
        self.fields.iter().map(|field| field.get_width()).sum()
    }

    /// Generate length checks for this chunk.
    pub fn generate_length_checks(
        &self,
        packet_name: &str,
        offset: usize,
    ) -> Vec<proc_macro2::TokenStream> {
        let mut field_offset = offset;
        let mut last_field_range_end = 0;
        let mut length_checks = Vec::new();
        for field in self.fields {
            let id = field.get_id();
            let width = field.get_width();
            let field_range = get_field_range(field_offset, width);
            field_offset += width;
            if field_range.end == last_field_range_end {
                continue;
            }

            last_field_range_end = field_range.end;
            let range_end = syn::Index::from(field_range.end);
            length_checks.push(quote! {
                if bytes.len() < #range_end {
                    return Err(Error::InvalidLengthError {
                        obj: #packet_name.to_string(),
                        field: #id.to_string(),
                        wanted: #range_end,
                        got: bytes.len(),
                    });
                }
            });
        }
        length_checks
    }
}
