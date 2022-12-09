use crate::backends::rust::{mask_bits, types};
use crate::{ast, lint};
use quote::{format_ident, quote};

/// A single bit-field value.
struct BitField {
    value: proc_macro2::TokenStream, // An expression which produces a value.
    shift: usize,                    // A bit-shift to apply to `value`.
}

pub struct FieldSerializer<'a> {
    scope: &'a lint::Scope<'a>,
    endianness: ast::EndiannessValue,
    packet_name: &'a str,
    span: &'a proc_macro2::Ident,
    chunk: Vec<BitField>,
    code: Vec<proc_macro2::TokenStream>,
    shift: usize,
}

impl<'a> FieldSerializer<'a> {
    pub fn new(
        scope: &'a lint::Scope<'a>,
        endianness: ast::EndiannessValue,
        packet_name: &'a str,
        span: &'a proc_macro2::Ident,
    ) -> FieldSerializer<'a> {
        FieldSerializer {
            scope,
            endianness,
            packet_name,
            span,
            chunk: Vec::new(),
            code: Vec::new(),
            shift: 0,
        }
    }

    fn endianness_suffix(&'a self, width: usize) -> &'static str {
        if width > 8 && self.endianness == ast::EndiannessValue::LittleEndian {
            "_le"
        } else {
            ""
        }
    }

    /// Write an unsigned integer `value` to `self.span`.
    ///
    /// The generated code requires that `self.span` is a mutable
    /// `bytes::BufMut` value.
    fn put_uint(
        &'a self,
        value: &proc_macro2::TokenStream,
        width: usize,
    ) -> proc_macro2::TokenStream {
        let span = &self.span;
        let suffix = self.endianness_suffix(width);
        let value_type = types::Integer::new(width);
        if value_type.width == width {
            let put_u = format_ident!("put_u{}{}", width, suffix);
            quote! {
                #span.#put_u(#value)
            }
        } else {
            let put_uint = format_ident!("put_uint{}", suffix);
            let value_nbytes = proc_macro2::Literal::usize_unsuffixed(width / 8);
            let cast = (value_type.width < 64).then(|| quote!(as u64));
            quote! {
                #span.#put_uint(#value #cast, #value_nbytes)
            }
        }
    }

    pub fn add(&mut self, field: &ast::Field) {
        if field.is_bitfield(self.scope) {
            self.add_bit_field(field);
            return;
        }

        todo!("not yet supported: {field:?}")
    }

    fn add_bit_field(&mut self, field: &ast::Field) {
        let width = field.width(self.scope).unwrap();

        match field {
            ast::Field::Scalar { id, width, .. } => {
                let field_name = format_ident!("{id}");
                let field_type = types::Integer::new(*width);
                if field_type.width > *width {
                    let packet_name = &self.packet_name;
                    let max_value = mask_bits(*width);
                    self.code.push(quote! {
                        if self.#field_name > #max_value {
                            panic!(
                                "Invalid value for {}::{}: {} > {}",
                                #packet_name, #id, self.#field_name, #max_value
                            );
                        }
                    });
                }
                self.chunk.push(BitField { value: quote!(self.#field_name), shift: self.shift });
            }
            ast::Field::Typedef { id, .. } => {
                let field_name = format_ident!("{id}");
                let field_type = types::Integer::new(width);
                let to_u = format_ident!("to_u{}", field_type.width);
                // TODO(mgeisler): remove `unwrap` and return error to
                // caller in generated code.
                self.chunk.push(BitField {
                    value: quote!(self.#field_name.#to_u().unwrap()),
                    shift: self.shift,
                });
            }
            _ => todo!(),
        }

        self.shift += width;
        if self.shift % 8 == 0 {
            self.pack_bit_fields()
        }
    }

    fn pack_bit_fields(&mut self) {
        assert_eq!(self.shift % 8, 0);
        let chunk_type = types::Integer::new(self.shift);
        let chunk_len = self.chunk.len();
        let values = self
            .chunk
            .drain(..)
            .map(|BitField { mut value, shift }| {
                if chunk_len > 1 {
                    // We will be combining values with `|`, so we
                    // need to cast them first. If there is a single
                    // value in the chunk, `self.put_uint` will cast.
                    value = quote! { (#value as #chunk_type) };
                }
                if shift > 0 {
                    let op = quote!(<<);
                    let shift = proc_macro2::Literal::usize_unsuffixed(shift);
                    value = quote! { (#value #op #shift) };
                }
                value
            })
            .collect::<Vec<_>>();

        match values.as_slice() {
            [] => todo!(),
            [value] => {
                let put = self.put_uint(value, self.shift);
                self.code.push(quote! {
                    #put;
                });
            }
            _ => {
                let put = self.put_uint(&quote!(value), self.shift);
                self.code.push(quote! {
                    let value = #(#values)|*;
                    #put;
                });
            }
        }

        self.shift = 0;
    }
}

impl quote::ToTokens for FieldSerializer<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let code = &self.code;
        tokens.extend(quote! {
            #(#code)*
        });
    }
}
