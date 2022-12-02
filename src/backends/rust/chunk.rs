use crate::ast;
use crate::backends::rust::field::Field;
use crate::backends::rust::types::Integer;
use quote::{format_ident, quote};

fn endianness_suffix(width: usize, endianness_value: ast::EndiannessValue) -> &'static str {
    if width > 8 && endianness_value == ast::EndiannessValue::LittleEndian {
        "_le"
    } else {
        ""
    }
}

/// Parse an unsigned integer from `buffer`.
///
/// The generated code requires that `buffer` is a mutable
/// `bytes::Buf` value.
fn get_uint(
    endianness: ast::EndiannessValue,
    buffer: proc_macro2::Ident,
    width: usize,
) -> proc_macro2::TokenStream {
    let suffix = endianness_suffix(width, endianness);
    let rust_integer_widths = [8, 16, 32, 64];
    if rust_integer_widths.contains(&width) {
        // We can use Buf::get_uNN.
        let get_u = format_ident!("get_u{}{}", width, suffix);
        quote! {
            #buffer.#get_u()
        }
    } else {
        // We fall back to Buf::get_uint.
        let get_uint = format_ident!("get_uint{}", suffix);
        let value_type = Integer::new(width);
        let value_nbytes = proc_macro2::Literal::usize_unsuffixed(width / 8);
        quote! {
            #buffer.#get_uint(#value_nbytes) as #value_type
        }
    }
}

/// Write an unsigned integer `value` to `buffer`.
///
/// The generated code requires that `buffer` is a mutable
/// `bytes::BufMut` value.
fn put_uint(
    endianness: ast::EndiannessValue,
    buffer: proc_macro2::Ident,
    value: proc_macro2::TokenStream,
    width: usize,
) -> proc_macro2::TokenStream {
    let suffix = endianness_suffix(width, endianness);
    let rust_integer_widths = [8, 16, 32, 64];
    if rust_integer_widths.contains(&width) {
        // We can use BufMut::put_uNN.
        let put_u = format_ident!("put_u{}{}", width, suffix);
        quote! {
            #buffer.#put_u(#value)
        }
    } else {
        // We fall back to BufMut::put_uint.
        let put_uint = format_ident!("put_uint{}", suffix);
        let value_nbytes = proc_macro2::Literal::usize_unsuffixed(width / 8);
        quote! {
            #buffer.#put_uint(#value as u64, #value_nbytes)
        }
    }
}

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
    pub fn name(&self) -> proc_macro2::Ident {
        match self.fields {
            [field] => field.ident(),
            _ => format_ident!("chunk"),
        }
    }

    /// Return the width in bits.
    pub fn width(&self) -> usize {
        self.fields.iter().map(|field| field.width()).sum()
    }

    /// Generate length checks for this chunk.
    pub fn generate_length_check(&self, packet_name: &str) -> proc_macro2::TokenStream {
        let wanted_length = proc_macro2::Literal::usize_unsuffixed(self.width() / 8);
        quote! {
            if bytes.remaining() < #wanted_length {
                return Err(Error::InvalidLengthError {
                    obj: #packet_name.to_string(),
                    wanted: #wanted_length,
                    got: bytes.remaining(),
                });
            }
        }
    }

    /// Read data for a chunk.
    pub fn generate_read(
        &self,
        packet_name: &str,
        endianness_value: ast::EndiannessValue,
    ) -> proc_macro2::TokenStream {
        let chunk_name = self.name();
        let chunk_width = self.width();
        assert!(chunk_width % 8 == 0, "Chunks must have a byte size, got width: {chunk_width}");

        let length_check = self.generate_length_check(packet_name);
        let read = get_uint(endianness_value, format_ident!("bytes"), chunk_width);
        let read_adjustments = self.generate_read_adjustments();

        quote! {
            #length_check
            let #chunk_name = #read;
            #read_adjustments
        }
    }

    fn generate_read_adjustments(&self) -> proc_macro2::TokenStream {
        // If there is a single field in the chunk, then we don't have to
        // shift, mask, or cast.
        if self.fields.len() == 1 {
            return quote! {};
        }

        let chunk_width = self.width();
        let chunk_type = Integer::new(chunk_width);

        let mut field_parsers = Vec::new();
        let mut field_offset = 0;
        for field in self.fields {
            field_parsers.push(field.generate_read_adjustment(field_offset, chunk_type));
            field_offset += field.width();
        }

        quote! {
            #(#field_parsers)*
        }
    }

    pub fn generate_write(
        &self,
        endianness_value: ast::EndiannessValue,
    ) -> proc_macro2::TokenStream {
        let chunk_width = self.width();
        let chunk_name = self.name();
        assert!(chunk_width % 8 == 0, "Chunks must have a byte size, got width: {chunk_width}");

        // TODO(mgeisler): let slice = (chunk_type_width > chunk_width).then( ... )
        let write_adjustments = self.generate_write_adjustments();
        let write =
            put_uint(endianness_value, format_ident!("buffer"), quote!(#chunk_name), chunk_width);
        quote! {
            #write_adjustments
            #write;
        }
    }

    fn generate_write_adjustments(&self) -> proc_macro2::TokenStream {
        if let [field] = self.fields {
            // If there is a single field in the chunk, then we don't have to
            // shift, mask, or cast.
            let field_name = field.ident();
            return quote! {
                let #field_name = self.#field_name;
            };
        }

        let chunk_width = self.width();
        let chunk_type = Integer::new(chunk_width);

        let mut field_parsers = Vec::new();
        let mut field_offset = 0;
        for field in self.fields {
            field_parsers.push(field.generate_write_adjustment(field_offset, chunk_type));
            field_offset += field.width();
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
    use crate::test_utils::{assert_expr_eq, assert_snapshot_eq, rustfmt};

    #[test]
    fn test_generate_read_8bit() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 8 })];
        let chunk = Chunk::new(&fields);
        let chunk_read = chunk.generate_read("Foo", ast::EndiannessValue::BigEndian);
        let code = quote! { fn main() { #chunk_read } };
        assert_snapshot_eq(
            "tests/generated/generate_chunk_read_8bit.rs",
            &rustfmt(&code.to_string()),
        );
    }

    #[test]
    fn test_generate_read_16bit_le() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 16 })];
        let chunk = Chunk::new(&fields);
        let chunk_read = chunk.generate_read("Foo", ast::EndiannessValue::LittleEndian);
        let code = quote! { fn main() { #chunk_read } };
        assert_snapshot_eq(
            "tests/generated/generate_chunk_read_16bit_le.rs",
            &rustfmt(&code.to_string()),
        );
    }

    #[test]
    fn test_generate_read_16bit_be() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 16 })];
        let chunk = Chunk::new(&fields);
        let chunk_read = chunk.generate_read("Foo", ast::EndiannessValue::BigEndian);
        let code = quote! { fn main() { #chunk_read } };
        assert_snapshot_eq(
            "tests/generated/generate_chunk_read_16bit_be.rs",
            &rustfmt(&code.to_string()),
        );
    }

    #[test]
    fn test_generate_read_24bit_le() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 24 })];
        let chunk = Chunk::new(&fields);
        let chunk_read = chunk.generate_read("Foo", ast::EndiannessValue::LittleEndian);
        let code = quote! { fn main() { #chunk_read } };
        assert_snapshot_eq(
            "tests/generated/generate_chunk_read_24bit_le.rs",
            &rustfmt(&code.to_string()),
        );
    }

    #[test]
    fn test_generate_read_24bit_be() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 24 })];
        let chunk = Chunk::new(&fields);
        let chunk_read = chunk.generate_read("Foo", ast::EndiannessValue::BigEndian);
        let code = quote! { fn main() { #chunk_read } };
        assert_snapshot_eq(
            "tests/generated/generate_chunk_read_24bit_be.rs",
            &rustfmt(&code.to_string()),
        );
    }

    #[test]
    fn test_generate_read_multiple_fields() {
        let fields = [
            Field::Scalar(ScalarField { id: String::from("a"), width: 16 }),
            Field::Scalar(ScalarField { id: String::from("b"), width: 24 }),
        ];
        let chunk = Chunk::new(&fields);
        let chunk_read = chunk.generate_read("Foo", ast::EndiannessValue::BigEndian);
        let code = quote! { fn main() { #chunk_read } };
        assert_snapshot_eq(
            "tests/generated/generate_chunk_read_multiple_fields.rs",
            &rustfmt(&code.to_string()),
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
            chunk.generate_write(ast::EndiannessValue::BigEndian),
            quote! {
                let a = self.a;
                buffer.put_u8(a);
            },
        );
    }

    #[test]
    fn test_generate_write_16bit() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 16 })];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_write(ast::EndiannessValue::BigEndian),
            quote! {
                let a = self.a;
                buffer.put_u16(a);
            },
        );
    }

    #[test]
    fn test_generate_write_24bit() {
        let fields = [Field::Scalar(ScalarField { id: String::from("a"), width: 24 })];
        let chunk = Chunk::new(&fields);
        assert_expr_eq(
            chunk.generate_write(ast::EndiannessValue::BigEndian),
            quote! {
                let a = self.a;
                buffer.put_uint(a as u64, 3);
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
            chunk.generate_write(ast::EndiannessValue::BigEndian),
            quote! {
                let chunk = 0;
                let chunk = chunk | (self.a as u64);
                let chunk = chunk | (((self.b as u64) & 0xffffff) << 16);
                buffer.put_uint(chunk as u64, 5);
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
