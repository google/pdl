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
    pub fn generate_length_check(
        &self,
        packet_name: &str,
        offset: usize,
    ) -> proc_macro2::TokenStream {
        let range = get_field_range(offset, self.get_width());
        let wanted_length = syn::Index::from(range.end);
        quote! {
            if bytes.len() < #wanted_length {
                return Err(Error::InvalidLengthError {
                    obj: #packet_name.to_string(),
                    wanted: #wanted_length,
                    got: bytes.len(),
                });
            }
        }
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
        let length_check = self.generate_length_check(packet_name, offset);

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

        let read_adjustments = self.generate_read_adjustments();

        quote! {
            #length_check
            let #chunk_name = #chunk_type::#getter([
                #(#zero_padding_before,)* #(bytes[#indices]),* #(, #zero_padding_after)*
            ]);
            #read_adjustments
        }
    }

    fn generate_read_adjustments(&self) -> proc_macro2::TokenStream {
        // If there is a single field in the chunk, then we don't have to
        // shift, mask, or cast.
        if self.fields.len() == 1 {
            return quote! {};
        }

        let chunk_width = self.get_width();
        let chunk_type = Integer::new(chunk_width);

        let mut field_parsers = Vec::new();
        let mut field_offset = 0;
        for field in self.fields {
            field_parsers.push(field.generate_read_adjustment(field_offset, chunk_type));
            field_offset += field.get_width();
        }

        quote! {
            #(#field_parsers)*
        }
    }

    pub fn generate_write(
        &self,
        endianness_value: ast::EndiannessValue,
        offset: usize,
    ) -> proc_macro2::TokenStream {
        let writer = match endianness_value {
            ast::EndiannessValue::BigEndian => format_ident!("to_be_bytes"),
            ast::EndiannessValue::LittleEndian => format_ident!("to_le_bytes"),
        };

        let chunk_width = self.get_width();
        let chunk_name = self.get_name();
        assert!(chunk_width % 8 == 0, "Chunks must have a byte size, got width: {chunk_width}");

        let range = get_field_range(offset, chunk_width);
        let start = syn::Index::from(range.start);
        let end = syn::Index::from(range.end);
        // TODO(mgeisler): let slice = (chunk_type_width > chunk_width).then( ... )
        let chunk_byte_width = syn::Index::from(chunk_width / 8);
        let write_adjustments = self.generate_write_adjustments();
        quote! {
            #write_adjustments
            buffer[#start..#end].copy_from_slice(&#chunk_name.#writer()[0..#chunk_byte_width]);
        }
    }

    fn generate_write_adjustments(&self) -> proc_macro2::TokenStream {
        if let [field] = self.fields {
            // If there is a single field in the chunk, then we don't have to
            // shift, mask, or cast.
            let field_name = field.get_ident();
            return quote! {
                let #field_name = self.#field_name;
            };
        }

        let chunk_width = self.get_width();
        let chunk_type = Integer::new(chunk_width);

        let mut field_parsers = Vec::new();
        let mut field_offset = 0;
        for field in self.fields {
            field_parsers.push(field.generate_write_adjustment(field_offset, chunk_type));
            field_offset += field.get_width();
        }

        quote! {
            let chunk = 0;
            #(#field_parsers)*
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
                if bytes.len() < 15 {
                    return Err(Error::InvalidLengthError {
                        obj: "Foo".to_string(),
                        wanted: 15,
                        got: bytes.len(),
                    });
                }
                let chunk =
                    u64::from_be_bytes([0, 0, 0, bytes[10], bytes[11], bytes[12], bytes[13], bytes[14]]);
                let a = chunk as u16;
                let b = ((chunk >> 16) & 0xffffff) as u32;
            },
        );
    }

    #[test]
    fn test_generate_read_adjustments_8bit() {
        let fields = vec![
            Field::Scalar(ScalarField { id: String::from("a"), width: 3 }),
            Field::Scalar(ScalarField { id: String::from("b"), width: 5 }),
        ];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_read_adjustments(),
            quote! {
                let a = (chunk & 0x7);
                let b = ((chunk >> 3) & 0x1f);
            },
        );
    }

    #[test]
    fn test_generate_read_adjustments_48bit() {
        let fields = vec![
            Field::Scalar(ScalarField { id: String::from("a"), width: 3 }),
            Field::Scalar(ScalarField { id: String::from("b"), width: 8 }),
            Field::Scalar(ScalarField { id: String::from("c"), width: 10 }),
            Field::Scalar(ScalarField { id: String::from("d"), width: 18 }),
            Field::Scalar(ScalarField { id: String::from("e"), width: 9 }),
        ];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_read_adjustments(),
            quote! {
                let a = (chunk & 0x7) as u8;
                let b = (chunk >> 3) as u8;
                let c = ((chunk >> 11) & 0x3ff) as u16;
                let d = ((chunk >> 21) & 0x3ffff) as u32;
                let e = ((chunk >> 39) & 0x1ff) as u16;
            },
        );
    }

    #[test]
    fn test_generate_write_8bit() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 8 })];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_write(ast::EndiannessValue::BigEndian, 80),
            quote! {
                let a = self.a;
                buffer[10..11].copy_from_slice(&a.to_be_bytes()[0..1]);
            },
        );
    }

    #[test]
    fn test_generate_write_16bit() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 16 })];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_write(ast::EndiannessValue::BigEndian, 80),
            quote! {
                let a = self.a;
                buffer[10..12].copy_from_slice(&a.to_be_bytes()[0..2]);
            },
        );
    }

    #[test]
    fn test_generate_write_24bit() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 24 })];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_write(ast::EndiannessValue::BigEndian, 80),
            quote! {
                let a = self.a;
                buffer[10..13].copy_from_slice(&a.to_be_bytes()[0..3]);
            },
        );
    }

    #[test]
    fn test_generate_write_multiple_fields() {
        let fields = [
            Field::Scalar(ScalarField { id: String::from("a"), width: 16 }),
            Field::Scalar(ScalarField { id: String::from("b"), width: 24 }),
        ];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_write(ast::EndiannessValue::BigEndian, 80),
            quote! {
                let chunk = 0;
                let chunk = chunk | (self.a as u64);
                let chunk = chunk | (((self.b as u64) & 0xffffff) << 16);
                buffer[10..15].copy_from_slice(&chunk.to_be_bytes()[0..5]);
            },
        );
    }

    #[test]
    fn test_generate_write_adjustments_8bit() {
        let fields = vec![
            Field::Scalar(ScalarField { id: String::from("a"), width: 3 }),
            Field::Scalar(ScalarField { id: String::from("b"), width: 5 }),
        ];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_write_adjustments(),
            quote! {
                let chunk = 0;
                let chunk = chunk | (self.a & 0x7) ;
                let chunk = chunk | ((self.b & 0x1f) << 3);
            },
        );
    }

    #[test]
    fn test_generate_write_adjustments_48bit() {
        let fields = vec![
            Field::Scalar(ScalarField { id: String::from("a"), width: 3 }),
            Field::Scalar(ScalarField { id: String::from("b"), width: 8 }),
            Field::Scalar(ScalarField { id: String::from("c"), width: 10 }),
            Field::Scalar(ScalarField { id: String::from("d"), width: 18 }),
            Field::Scalar(ScalarField { id: String::from("e"), width: 9 }),
        ];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_write_adjustments(),
            quote! {
                let chunk = 0;
                let chunk = chunk | ((self.a as u64) & 0x7);
                let chunk = chunk | ((self.b as u64) << 3);
                let chunk = chunk | (((self.c as u64) & 0x3ff) << 11);
                let chunk = chunk | (((self.d as u64) & 0x3ffff) << 21);
                let chunk = chunk | (((self.e as u64) & 0x1ff) << 39);
            },
        );
    }
}
