use crate::ast;
use crate::backends::rust::field::Field;
use crate::backends::rust::get_field_range;
use crate::backends::rust::types::Integer;
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

    /// Read data for a chunk.
    pub fn generate_read(
        &self,
        packet_name: &str,
        endianness_value: ast::EndiannessValue,
        offset: usize,
    ) -> proc_macro2::TokenStream {
        assert!(offset % 8 == 0, "Chunks must be byte-aligned, got offset: {offset}");
        let getter = match endianness_value {
            ast::EndiannessValue::BigEndian => format_ident!("from_be_bytes"),
            ast::EndiannessValue::LittleEndian => format_ident!("from_le_bytes"),
        };

        let chunk_name = self.get_name();
        let chunk_width = self.get_width();
        let chunk_type = Integer::new(chunk_width);
        assert!(chunk_width % 8 == 0, "Chunks must have a byte size, got width: {chunk_width}");

        let range = get_field_range(offset, chunk_width);
        let indices = range.map(syn::Index::from).collect::<Vec<_>>();

        // TODO(mgeisler): emit just a single length check per chunk. We
        // could even emit a single length check per packet.
        let length_checks = self.generate_length_checks(packet_name, offset);

        // When the chunk_type.width is larger than chunk_width (e.g.
        // chunk_width is 24 but chunk_type.width is 32), then we need
        // zero padding.
        let zero_padding_len = (chunk_type.width - chunk_width) / 8;
        // We need the padding on the MSB side of the payload, so for
        // big-endian, we need to padding on the left, for little-endian
        // we need it on the right.
        let (zero_padding_before, zero_padding_after) = match endianness_value {
            ast::EndiannessValue::BigEndian => {
                (vec![syn::Index::from(0); zero_padding_len], vec![])
            }
            ast::EndiannessValue::LittleEndian => {
                (vec![], vec![syn::Index::from(0); zero_padding_len])
            }
        };

        quote! {
            #(#length_checks)*
            let #chunk_name = #chunk_type::#getter([
                #(#zero_padding_before,)* #(bytes[#indices]),* #(, #zero_padding_after)*
            ]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backends::rust::field::ScalarField;
    use crate::test_utils::assert_expr_eq;

    #[test]
    fn test_generate_read_8bit() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 8 })];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_read("Foo", ast::EndiannessValue::BigEndian, 80),
            quote! {
                if bytes.len() < 11 {
                    return Err(Error::InvalidLengthError {
                        obj: "Foo".to_string(),
                        field: "a".to_string(),
                        wanted: 11,
                        got: bytes.len(),
                    });
                }
                let a = u8::from_be_bytes([bytes[10]]);
            },
        );
    }

    #[test]
    fn test_generate_read_16bit_le() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 16 })];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_read("Foo", ast::EndiannessValue::LittleEndian, 80),
            quote! {
                if bytes.len() < 12 {
                    return Err(Error::InvalidLengthError {
                        obj: "Foo".to_string(),
                        field: "a".to_string(),
                        wanted: 12,
                        got: bytes.len(),
                    });
                }
                let a = u16::from_le_bytes([bytes[10], bytes[11]]);
            },
        );
    }

    #[test]
    fn test_generate_read_16bit_be() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 16 })];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_read("Foo", ast::EndiannessValue::BigEndian, 80),
            quote! {
                if bytes.len() < 12 {
                    return Err(Error::InvalidLengthError {
                        obj: "Foo".to_string(),
                        field: "a".to_string(),
                        wanted: 12,
                        got: bytes.len(),
                    });
                }
                let a = u16::from_be_bytes([bytes[10], bytes[11]]);
            },
        );
    }

    #[test]
    fn test_generate_read_24bit_le() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 24 })];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_read("Foo", ast::EndiannessValue::LittleEndian, 80),
            quote! {
                if bytes.len() < 13 {
                    return Err(Error::InvalidLengthError {
                        obj: "Foo".to_string(),
                        field: "a".to_string(),
                        wanted: 13,
                        got: bytes.len(),
                    });
                }
                let a = u32::from_le_bytes([bytes[10], bytes[11], bytes[12], 0]);
            },
        );
    }

    #[test]
    fn test_generate_read_24bit_be() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 24 })];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_read("Foo", ast::EndiannessValue::BigEndian, 80),
            quote! {
                if bytes.len() < 13 {
                    return Err(Error::InvalidLengthError {
                        obj: "Foo".to_string(),
                        field: "a".to_string(),
                        wanted: 13,
                        got: bytes.len(),
                    });
                }
                let a = u32::from_be_bytes([0, bytes[10], bytes[11], bytes[12]]);
            },
        );
    }

    #[test]
    fn test_generate_read_multiple_fields() {
        let fields = [
            Field::Scalar(ScalarField { id: String::from("a"), width: 16 }),
            Field::Scalar(ScalarField { id: String::from("b"), width: 24 }),
        ];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_read("Foo", ast::EndiannessValue::BigEndian, 80),
            quote! {
                if bytes.len() < 12 {
                    return Err(Error::InvalidLengthError {
                        obj: "Foo".to_string(),
                        field: "a".to_string(),
                        wanted: 12,
                        got: bytes.len(),
                    });
                }
                if bytes.len() < 15 {
                    return Err(Error::InvalidLengthError {
                        obj: "Foo".to_string(),
                        field: "b".to_string(),
                        wanted: 15,
                        got: bytes.len(),
                    });
                }
                let chunk =
                    u64::from_be_bytes([0, 0, 0, bytes[10], bytes[11], bytes[12], bytes[13], bytes[14]]);
            },
        );
    }
}
