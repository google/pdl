//! Utility functions for dealing with Rust integer types.

use crate::ast;
use crate::parser::ast as parser_ast;
use quote::{format_ident, quote};

/// A Rust integer type such as `u8`.
#[derive(Copy, Clone)]
pub struct Integer {
    pub width: usize,
}

impl Integer {
    /// Get the Rust integer type for the given bit width.
    ///
    /// This will round up the size to the nearest Rust integer size.
    /// PDL supports integers up to 64 bit, so it is an error to call
    /// this with a width larger than 64.
    pub fn new(width: usize) -> Integer {
        for integer_width in [8, 16, 32, 64] {
            if width <= integer_width {
                return Integer { width: integer_width };
            }
        }
        panic!("Cannot construct Integer with width: {width}")
    }
}

impl quote::ToTokens for Integer {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let t: syn::Type = syn::parse_str(&format!("u{}", self.width))
            .expect("Could not parse integer, unsupported width?");
        t.to_tokens(tokens);
    }
}

pub fn rust_type(field: &parser_ast::Field) -> proc_macro2::TokenStream {
    match &field.desc {
        ast::FieldDesc::Scalar { width, .. } => {
            let field_type = Integer::new(*width);
            quote!(#field_type)
        }
        ast::FieldDesc::Typedef { type_id, .. } => {
            let field_type = format_ident!("{type_id}");
            quote!(#field_type)
        }
        _ => todo!(),
    }
}

/// Suffix for `Buf::get_*` and `BufMut::put_*` methods when reading a
/// value with the given `width`.
fn endianness_suffix(endianness: ast::EndiannessValue, width: usize) -> &'static str {
    if width > 8 && endianness == ast::EndiannessValue::LittleEndian {
        "_le"
    } else {
        ""
    }
}

/// Parse an unsigned integer with the given `width`.
///
/// The generated code requires that `span` is a mutable `bytes::Buf`
/// value.
pub fn get_uint(
    endianness: ast::EndiannessValue,
    width: usize,
    span: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let suffix = endianness_suffix(endianness, width);
    let value_type = Integer::new(width);
    if value_type.width == width {
        let get_u = format_ident!("get_u{}{}", value_type.width, suffix);
        quote! {
            #span.#get_u()
        }
    } else {
        let get_uint = format_ident!("get_uint{}", suffix);
        let value_nbytes = proc_macro2::Literal::usize_unsuffixed(width / 8);
        let cast = (value_type.width < 64).then(|| quote!(as #value_type));
        quote! {
            #span.#get_uint(#value_nbytes) #cast
        }
    }
}

/// Write an unsigned integer `value` to `span`.
///
/// The generated code requires that `span` is a mutable
/// `bytes::BufMut` value.
pub fn put_uint(
    endianness: ast::EndiannessValue,
    value: &proc_macro2::TokenStream,
    width: usize,
    span: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let suffix = endianness_suffix(endianness, width);
    let value_type = Integer::new(width);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_new() {
        assert_eq!(Integer::new(0).width, 8);
        assert_eq!(Integer::new(8).width, 8);
        assert_eq!(Integer::new(9).width, 16);
        assert_eq!(Integer::new(64).width, 64);
    }

    #[test]
    #[should_panic]
    fn test_integer_new_panics_on_large_width() {
        Integer::new(65);
    }
}
