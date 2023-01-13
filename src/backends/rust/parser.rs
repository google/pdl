use crate::backends::rust::{mask_bits, types};
use crate::{ast, lint};
use quote::{format_ident, quote};

/// A single bit-field.
struct BitField<'a> {
    shift: usize, // The shift to apply to this field.
    field: &'a ast::Field,
}

pub struct FieldParser<'a> {
    scope: &'a lint::Scope<'a>,
    endianness: ast::EndiannessValue,
    packet_name: &'a str,
    span: &'a proc_macro2::Ident,
    chunk: Vec<BitField<'a>>,
    code: Vec<proc_macro2::TokenStream>,
    shift: usize,
    offset: usize,
}

impl<'a> FieldParser<'a> {
    pub fn new(
        scope: &'a lint::Scope<'a>,
        endianness: ast::EndiannessValue,
        packet_name: &'a str,
        span: &'a proc_macro2::Ident,
    ) -> FieldParser<'a> {
        FieldParser {
            scope,
            endianness,
            packet_name,
            span,
            chunk: Vec::new(),
            code: Vec::new(),
            shift: 0,
            offset: 0,
        }
    }

    pub fn add(&mut self, field: &'a ast::Field) {
        if field.is_bitfield(self.scope) {
            self.add_bit_field(field);
            return;
        }

        todo!("not yet supported: {field:?}")
    }

    fn add_bit_field(&mut self, field: &'a ast::Field) {
        self.chunk.push(BitField { shift: self.shift, field });
        self.shift += field.width(self.scope).unwrap();
        if self.shift % 8 != 0 {
            return;
        }

        let size = self.shift / 8;
        let end_offset = self.offset + size;

        let wanted = proc_macro2::Literal::usize_unsuffixed(size);
        let packet_name = &self.packet_name;
        self.code.push(quote! {
            if bytes.remaining() < #wanted {
                return Err(Error::InvalidLengthError {
                    obj: #packet_name.to_string(),
                    wanted: #wanted,
                    got: bytes.remaining(),
                });
            }
        });

        let chunk_type = types::Integer::new(self.shift);
        // TODO(mgeisler): generate Rust variable names which cannot
        // conflict with PDL field names. An option would be to start
        // Rust variable names with `_`, but that has a special
        // semantic in Rust.
        let chunk_name = format_ident!("chunk");

        let get = types::get_uint(self.endianness, self.shift, self.span);
        if self.chunk.len() > 1 {
            // Multiple values: we read into a local variable.
            self.code.push(quote! {
                let #chunk_name = #get;
            });
        }

        let single_value = self.chunk.len() == 1; // && self.chunk[0].offset == 0;
        for BitField { shift, field } in self.chunk.drain(..) {
            let mut v = if single_value {
                // Single value: read directly.
                quote! { #get }
            } else {
                // Multiple values: read from `chunk_name`.
                quote! { #chunk_name }
            };

            if shift > 0 {
                let shift = proc_macro2::Literal::usize_unsuffixed(shift);
                v = quote! { (#v >> #shift) }
            }

            let width = field.width(self.scope).unwrap();
            let value_type = types::Integer::new(width);
            if !single_value && width < value_type.width {
                // Mask value if we grabbed more than `width` and if
                // `as #value_type` doesn't already do the masking.
                let mask = mask_bits(width);
                v = quote! { (#v & #mask) };
            }

            if value_type.width < chunk_type.width {
                v = quote! { #v as #value_type };
            }

            self.code.push(match field {
                ast::Field::Scalar { id, .. } => {
                    let id = format_ident!("{id}");
                    quote! {
                        let #id = #v;
                    }
                }
                ast::Field::Typedef { id, type_id, .. } => {
                    let id = format_ident!("{id}");
                    let type_id = format_ident!("{type_id}");
                    let from_u = format_ident!("from_u{}", value_type.width);
                    // TODO(mgeisler): Remove the `unwrap` from the
                    // generated code and return the error to the
                    // caller.
                    quote! {
                        let #id = #type_id::#from_u(#v).unwrap();
                    }
                }
                ast::Field::Reserved { .. } => {
                    // Nothing to do here.
                    quote! {}
                }
                _ => todo!(),
            });
        }

        self.offset = end_offset;
        self.shift = 0;
    }

    pub fn done(&mut self) {}
}

impl quote::ToTokens for FieldParser<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let code = &self.code;
        tokens.extend(quote! {
            #(#code)*
        });
    }
}
