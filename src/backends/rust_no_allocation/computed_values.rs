use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::backends::intermediate::{
    ComputedOffset, ComputedOffsetId, ComputedValue, ComputedValueId,
};

/// This trait is implemented on computed quantities (offsets and values) that can be retrieved via a function call
pub trait Declarable {
    fn get_name(&self) -> String;

    fn get_ident(&self) -> Ident {
        format_ident!("try_get_{}", self.get_name())
    }

    fn call_fn(&self) -> TokenStream {
        let fn_name = self.get_ident();
        quote! { self.#fn_name()? }
    }

    fn declare_fn(&self, body: TokenStream) -> TokenStream {
        let fn_name = self.get_ident();
        quote! {
            #[inline]
            fn #fn_name(&self) -> Result<usize, ParseError> {
                #body
            }
        }
    }
}

impl Declarable for ComputedValueId<'_> {
    fn get_name(&self) -> String {
        match self {
            ComputedValueId::FieldSize(field) => format!("{field}_size"),
            ComputedValueId::FieldElementSize(field) => format!("{field}_element_size"),
            ComputedValueId::FieldCount(field) => format!("{field}_count"),
            ComputedValueId::Custom(i) => format!("custom_value_{i}"),
        }
    }
}

impl Declarable for ComputedOffsetId<'_> {
    fn get_name(&self) -> String {
        match self {
            ComputedOffsetId::HeaderStart => "header_start_offset".to_string(),
            ComputedOffsetId::PacketEnd => "packet_end_offset".to_string(),
            ComputedOffsetId::FieldOffset(field) => format!("{field}_offset"),
            ComputedOffsetId::FieldEndOffset(field) => format!("{field}_end_offset"),
            ComputedOffsetId::Custom(i) => format!("custom_offset_{i}"),
            ComputedOffsetId::TrailerStart => "trailer_start_offset".to_string(),
        }
    }
}

/// This trait is implemented on computed expressions that are computed on-demand (i.e. not via a function call)
pub trait Computable {
    fn compute(&self) -> TokenStream;
}

impl Computable for ComputedValue<'_> {
    fn compute(&self) -> TokenStream {
        match self {
            ComputedValue::Constant(k) => quote! { Ok(#k) },
            ComputedValue::CountStructsUpToSize { base_id, size, struct_type } => {
                let base_offset = base_id.call_fn();
                let size = size.call_fn();
                let struct_type = format_ident!("{struct_type}View");
                quote! {
                    let mut cnt = 0;
                    let mut view = self.buf.offset(#base_offset)?;
                    let mut remaining_size = #size;
                    while remaining_size > 0 {
                        let next_struct_size = #struct_type::try_parse(view)?.try_get_size()?;
                        if next_struct_size > remaining_size {
                            return Err(ParseError::OutOfBoundsAccess);
                        }
                        remaining_size -= next_struct_size;
                        view = view.offset(next_struct_size * 8)?;
                        cnt += 1;
                    }
                    Ok(cnt)
                }
            }
            ComputedValue::SizeOfNStructs { base_id, n, struct_type } => {
                let base_offset = base_id.call_fn();
                let n = n.call_fn();
                let struct_type = format_ident!("{struct_type}View");
                quote! {
                    let mut view = self.buf.offset(#base_offset)?;
                    let mut size = 0;
                    for _ in 0..#n {
                        let next_struct_size = #struct_type::try_parse(view)?.try_get_size()?;
                        size += next_struct_size;
                        view = view.offset(next_struct_size * 8)?;
                    }
                    Ok(size)
                }
            }
            ComputedValue::Product(x, y) => {
                let x = x.call_fn();
                let y = y.call_fn();
                quote! { #x.checked_mul(#y).ok_or(ParseError::ArithmeticOverflow) }
            }
            ComputedValue::Divide(x, y) => {
                let x = x.call_fn();
                let y = y.call_fn();
                quote! {
                    if #y == 0 || #x % #y != 0 {
                        return Err(ParseError::DivisionFailure)
                    }
                    Ok(#x / #y)
                }
            }
            ComputedValue::Difference(x, y) => {
                let x = x.call_fn();
                let y = y.call_fn();
                quote! {
                   let bit_difference = #x.checked_sub(#y).ok_or(ParseError::ArithmeticOverflow)?;
                   if bit_difference % 8 != 0 {
                       return Err(ParseError::DivisionFailure);
                   }
                   Ok(bit_difference / 8)
                }
            }
            ComputedValue::ValueAt { offset, width } => {
                let offset = offset.call_fn();
                quote! { self.buf.offset(#offset)?.slice(#width)?.try_parse() }
            }
        }
    }
}

impl Computable for ComputedOffset<'_> {
    fn compute(&self) -> TokenStream {
        match self {
            ComputedOffset::ConstantPlusOffsetInBits(base_id, offset) => {
                let base_id = base_id.call_fn();
                quote! { #base_id.checked_add_signed(#offset as isize).ok_or(ParseError::ArithmeticOverflow) }
            }
            ComputedOffset::SumWithOctets(x, y) => {
                let x = x.call_fn();
                let y = y.call_fn();
                quote! {
                    #x.checked_add(#y.checked_mul(8).ok_or(ParseError::ArithmeticOverflow)?)
                      .ok_or(ParseError::ArithmeticOverflow)
                }
            }
            ComputedOffset::Alias(alias) => {
                let alias = alias.call_fn();
                quote! { Ok(#alias) }
            }
        }
    }
}
