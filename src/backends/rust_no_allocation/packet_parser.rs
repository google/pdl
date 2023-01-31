use std::iter::empty;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ast;

use crate::backends::intermediate::{
    ComputedOffsetId, ComputedValueId, PacketOrStruct, PacketOrStructLength, Schema,
};

use super::computed_values::{Computable, Declarable};
use super::utils::get_integer_type;

pub fn generate_packet(
    id: &str,
    fields: &[ast::Field],
    parent_id: Option<&str>,
    schema: &Schema,
    curr_schema: &PacketOrStruct,
) -> Result<TokenStream, String> {
    let id_ident = format_ident!("{id}View");

    let needs_external = matches!(curr_schema.length, PacketOrStructLength::NeedsExternal);

    let length_getter = if needs_external {
        ComputedOffsetId::PacketEnd.declare_fn(quote! { Ok(self.buf.get_size_in_bits()) })
    } else {
        quote! {}
    };

    let computed_getters = empty()
        .chain(
            curr_schema.computed_offsets.iter().map(|(decl, defn)| decl.declare_fn(defn.compute())),
        )
        .chain(
            curr_schema.computed_values.iter().map(|(decl, defn)| decl.declare_fn(defn.compute())),
        );

    let field_getters = fields.iter().map(|field| {
        match field {
            ast::Field::Padding { .. }
            | ast::Field::Reserved { .. }
            | ast::Field::Fixed { .. }
            | ast::Field::ElementSize { .. }
            | ast::Field::Count { .. }
            | ast::Field::Size { .. } => {
                // no-op, no getter generated for this type
                quote! {}
            }
            ast::Field::Group { .. } => unreachable!(),
            ast::Field::Checksum { .. } => {
                unimplemented!("checksums not yet supported with this backend")
            }
            ast::Field::Payload { .. } | ast::Field::Body { .. } => {
                let name = if matches!(field, ast::Field::Payload { .. }) { "_payload_"} else { "_body_"};
                let payload_start_offset = ComputedOffsetId::FieldOffset(name).call_fn();
                let payload_end_offset = ComputedOffsetId::FieldEndOffset(name).call_fn();
                quote! {
                    fn try_get_payload(&self) -> Result<SizedBitSlice<'a>, ParseError> {
                        let payload_start_offset = #payload_start_offset;
                        let payload_end_offset = #payload_end_offset;
                        self.buf.offset(payload_start_offset)?.slice(payload_end_offset - payload_start_offset)
                    }

                    fn try_get_raw_payload(&self) -> Result<impl Iterator<Item = Result<u8, ParseError>> + '_, ParseError> {
                        let view = self.try_get_payload()?;
                        let count = (view.get_size_in_bits() + 7) / 8;
                        Ok((0..count).map(move |i| Ok(view.offset(i*8)?.slice(8.min(view.get_size_in_bits() - i*8))?.try_parse()?)))
                    }

                    pub fn get_raw_payload(&self) -> impl Iterator<Item = u8> + '_ {
                        self.try_get_raw_payload().unwrap().map(|x| x.unwrap())
                    }
                }
            }
            ast::Field::Array { id, width, type_id, .. } => {
                let (elem_type, return_type) = if let Some(width) = width {
                    let ident = get_integer_type(*width);
                    (ident.clone(), quote!{ #ident })
                } else if let Some(type_id) = type_id {
                    if schema.enums.contains_key(type_id.as_str()) {
                        let ident = format_ident!("{}", type_id);
                        (ident.clone(), quote! { #ident })
                    } else {
                        let ident = format_ident!("{}View", type_id);
                        (ident.clone(), quote! { #ident<'a> })
                    }
                } else {
                    unreachable!()
                };

                let try_getter_name = format_ident!("try_get_{id}_iter");
                let getter_name = format_ident!("get_{id}_iter");

                let start_offset = ComputedOffsetId::FieldOffset(id).call_fn();
                let count = ComputedValueId::FieldCount(id).call_fn();

                let element_size_known = curr_schema
                    .computed_values
                    .contains_key(&ComputedValueId::FieldElementSize(id));

                let body = if element_size_known {
                    let element_size = ComputedValueId::FieldElementSize(id).call_fn();
                    let parsed_curr_view = if width.is_some() {
                        quote! { curr_view.try_parse() }
                    } else {
                        quote! { #elem_type::try_parse(curr_view.into()) }
                    };
                    quote! {
                        let view = self.buf.offset(#start_offset)?;
                        let count = #count;
                        let element_size = #element_size;
                        Ok((0..count).map(move |i| {
                            let curr_view = view.offset(element_size.checked_mul(i * 8).ok_or(ParseError::ArithmeticOverflow)?)?
                                    .slice(element_size.checked_mul(8).ok_or(ParseError::ArithmeticOverflow)?)?;
                            #parsed_curr_view
                        }))
                    }
                } else {
                    quote! {
                        let mut view = self.buf.offset(#start_offset)?;
                        let count = #count;
                        Ok((0..count).map(move |i| {
                            let parsed = #elem_type::try_parse(view.into())?;
                            view = view.offset(parsed.try_get_size()? * 8)?;
                            Ok(parsed)
                        }))
                    }
                };

                quote! {
                    fn #try_getter_name(&self) -> Result<impl Iterator<Item = Result<#return_type, ParseError>> + 'a, ParseError> {
                        #body
                    }

                    #[inline]
                    pub fn #getter_name(&self) -> impl Iterator<Item = #return_type> + 'a {
                        self.#try_getter_name().unwrap().map(|x| x.unwrap())
                    }
                }
            }
            ast::Field::Scalar { id, width, .. } => {
                let try_getter_name = format_ident!("try_get_{id}");
                let getter_name = format_ident!("get_{id}");
                let offset = ComputedOffsetId::FieldOffset(id).call_fn();
                let scalar_type = get_integer_type(*width);
                quote! {
                    fn #try_getter_name(&self) -> Result<#scalar_type, ParseError> {
                        self.buf.offset(#offset)?.slice(#width)?.try_parse()
                    }

                    #[inline]
                    pub fn #getter_name(&self) -> #scalar_type {
                        self.#try_getter_name().unwrap()
                    }
                }
            }
            ast::Field::Typedef { id, type_id, .. } => {
                let try_getter_name = format_ident!("try_get_{id}");
                let getter_name = format_ident!("get_{id}");

                let (type_ident, return_type) = if schema.enums.contains_key(type_id.as_str()) {
                    let ident = format_ident!("{type_id}");
                    (ident.clone(), quote! { #ident })
                } else {
                    let ident = format_ident!("{}View", type_id);
                    (ident.clone(), quote! { #ident<'a> })
                };
                let offset = ComputedOffsetId::FieldOffset(id).call_fn();
                let end_offset_known = curr_schema
                    .computed_offsets
                    .contains_key(&ComputedOffsetId::FieldEndOffset(id));
                let sliced_view = if end_offset_known {
                    let end_offset = ComputedOffsetId::FieldEndOffset(id).call_fn();
                    quote! { self.buf.offset(#offset)?.slice(#end_offset.checked_sub(#offset).ok_or(ParseError::ArithmeticOverflow)?)? }
                } else {
                    quote! { self.buf.offset(#offset)? }
                };

                quote! {
                    fn #try_getter_name(&self) -> Result<#return_type, ParseError> {
                        #type_ident::try_parse(#sliced_view.into())
                    }

                    #[inline]
                    pub fn #getter_name(&self) -> #return_type {
                        self.#try_getter_name().unwrap()
                    }
                }
            }
        }
    });

    let backing_buffer = if needs_external {
        quote! { SizedBitSlice<'a> }
    } else {
        quote! { BitSlice<'a> }
    };

    let parent_ident = match parent_id {
        Some(parent) => format_ident!("{parent}View"),
        None => match curr_schema.length {
            PacketOrStructLength::Static(_) => format_ident!("BitSlice"),
            PacketOrStructLength::Dynamic => format_ident!("BitSlice"),
            PacketOrStructLength::NeedsExternal => format_ident!("SizedBitSlice"),
        },
    };

    let buffer_extractor = if parent_id.is_some() {
        quote! { parent.try_get_payload().unwrap().into() }
    } else {
        quote! { parent }
    };

    let field_validators = fields.iter().map(|field| match field {
        ast::Field::Checksum { .. } => unimplemented!(),
        ast::Field::Group { .. } => unreachable!(),
        ast::Field::Padding { .. }
        | ast::Field::Size { .. }
        | ast::Field::Count { .. }
        | ast::Field::ElementSize { .. }
        | ast::Field::Body { .. }
        | ast::Field::Fixed { .. }
        | ast::Field::Reserved { .. } => {
            quote! {}
        }
        ast::Field::Payload { .. } => {
            quote! {
                self.try_get_payload()?;
                self.try_get_raw_payload()?;
            }
        }
        ast::Field::Array { id, .. } => {
            let iter_ident = format_ident!("try_get_{id}_iter");
            quote! {
                for elem in self.#iter_ident()? {
                    elem?;
                }
            }
        }
        ast::Field::Scalar { id, .. } | ast::Field::Typedef { id, .. } => {
            let getter_ident = format_ident!("try_get_{id}");
            quote! { self.#getter_ident()?; }
        }
    });

    let packet_end_offset = ComputedOffsetId::PacketEnd.call_fn();

    let owned_id_ident = format_ident!("Owned{id_ident}");
    let builder_ident = format_ident!("{id}Builder");

    Ok(quote! {
        #[derive(Clone, Copy, Debug)]
        pub struct #id_ident<'a> {
            buf: #backing_buffer,
        }

        impl<'a> #id_ident<'a> {
            #length_getter

            #(#computed_getters)*

            #(#field_getters)*

            #[inline]
            fn try_get_header_start_offset(&self) -> Result<usize, ParseError> {
                Ok(0)
            }

            #[inline]
            fn try_get_size(&self) -> Result<usize, ParseError> {
                let size = #packet_end_offset;
                if size % 8 != 0 {
                    return Err(ParseError::MisalignedPayload);
                }
                Ok(size / 8)
            }

            fn validate(&self) -> Result<(), ParseError> {
                #(#field_validators)*
                Ok(())
            }
        }

        impl<'a> Packet<'a> for #id_ident<'a> {
            type Parent = #parent_ident<'a>;
            type Owned = #owned_id_ident;
            type Builder = #builder_ident;

            fn try_parse_from_buffer(buf: impl Into<SizedBitSlice<'a>>) -> Result<Self, ParseError> {
                let out = Self { buf: buf.into().into() };
                out.validate()?;
                Ok(out)
            }

            fn try_parse(parent: #parent_ident<'a>) -> Result<Self, ParseError> {
                let out = Self { buf: #buffer_extractor };
                out.validate()?;
                Ok(out)
            }

            fn to_owned_packet(&self) -> #owned_id_ident {
                #owned_id_ident {
                    buf: self.buf.backing.to_owned().into(),
                    start_bit_offset: self.buf.start_bit_offset,
                    end_bit_offset: self.buf.end_bit_offset,
                }
            }
        }

        #[derive(Debug)]
        pub struct #owned_id_ident {
            buf: Box<[u8]>,
            start_bit_offset: usize,
            end_bit_offset: usize,
        }

        impl OwnedPacket for #owned_id_ident {
            fn try_parse(buf: Box<[u8]>) -> Result<Self, ParseError> {
                #id_ident::try_parse_from_buffer(&buf[..])?;
                let end_bit_offset = buf.len() * 8;
                Ok(Self { buf, start_bit_offset: 0, end_bit_offset })
            }
        }

        impl #owned_id_ident {
            pub fn view<'a>(&'a self) -> #id_ident<'a> {
                #id_ident {
                    buf: SizedBitSlice(BitSlice {
                        backing: &self.buf[..],
                        start_bit_offset: self.start_bit_offset,
                        end_bit_offset: self.end_bit_offset,
                    })
                    .into(),
                }
            }
        }

        impl<'a> From<&'a #owned_id_ident> for #id_ident<'a> {
            fn from(x: &'a #owned_id_ident) -> Self {
                x.view()
            }
        }
    })
}
