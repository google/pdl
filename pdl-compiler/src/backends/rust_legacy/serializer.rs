// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::backends::rust_legacy::{mask_bits, types, ToIdent, ToUpperCamelCase};
use crate::{analyzer, ast};
use quote::{format_ident, quote};

/// A single bit-field value.
struct BitField {
    value: proc_macro2::TokenStream, // An expression which produces a value.
    field_type: types::Integer,      // The type of the value.
    shift: usize,                    // A bit-shift to apply to `value`.
}

pub struct FieldSerializer<'a> {
    scope: &'a analyzer::Scope<'a>,
    schema: &'a analyzer::Schema,
    endianness: ast::EndiannessValue,
    packet_name: &'a str,
    span: &'a proc_macro2::Ident,
    chunk: Vec<BitField>,
    code: Vec<proc_macro2::TokenStream>,
    shift: usize,
}

impl<'a> FieldSerializer<'a> {
    pub fn new(
        scope: &'a analyzer::Scope<'a>,
        schema: &'a analyzer::Schema,
        endianness: ast::EndiannessValue,
        packet_name: &'a str,
        span: &'a proc_macro2::Ident,
    ) -> FieldSerializer<'a> {
        FieldSerializer {
            scope,
            schema,
            endianness,
            packet_name,
            span,
            chunk: Vec::new(),
            code: Vec::new(),
            shift: 0,
        }
    }

    pub fn add(&mut self, field: &ast::Field) {
        match &field.desc {
            _ if field.cond.is_some() => self.add_optional_field(field),
            _ if self.scope.is_bitfield(field) => self.add_bit_field(field),
            ast::FieldDesc::Array { id, width, .. } => self.add_array_field(
                id,
                *width,
                self.schema.padded_size(field.key),
                self.scope.get_type_declaration(field),
            ),
            ast::FieldDesc::Typedef { id, type_id } => {
                self.add_typedef_field(id, type_id);
            }
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body { .. } => {
                self.add_payload_field();
            }
            // Padding field handled in serialization of associated array field.
            ast::FieldDesc::Padding { .. } => (),
            _ => todo!("Cannot yet serialize {field:?}"),
        }
    }

    fn add_optional_field(&mut self, field: &ast::Field) {
        self.code.push(match &field.desc {
            ast::FieldDesc::Scalar { id, width } => {
                let name = id;
                let id = id.to_ident();
                let backing_type = types::Integer::new(*width);
                let write = types::put_uint(self.endianness, &quote!(*#id), *width, self.span);

                let range_check = (backing_type.width > *width).then(|| {
                    let packet_name = &self.packet_name;
                    let max_value = mask_bits(*width, "u64");

                    quote! {
                        if *#id > #max_value {
                            return Err(EncodeError::InvalidScalarValue {
                                packet: #packet_name,
                                field: #name,
                                value: *#id as u64,
                                maximum_value: #max_value as u64,
                            })
                        }
                    }
                });

                quote! {
                    if let Some(#id) = &self.#id {
                        #range_check
                        #write
                    }
                }
            }
            ast::FieldDesc::Typedef { id, type_id } => match &self.scope.typedef[type_id].desc {
                ast::DeclDesc::Enum { width, .. } => {
                    let id = id.to_ident();
                    let backing_type = types::Integer::new(*width);
                    let write = types::put_uint(
                        self.endianness,
                        &quote!(#backing_type::from(#id)),
                        *width,
                        self.span,
                    );
                    quote! {
                        if let Some(#id) = &self.#id {
                            #write
                        }
                    }
                }
                ast::DeclDesc::Struct { .. } => {
                    let id = id.to_ident();
                    let span = self.span;
                    quote! {
                        if let Some(#id) = &self.#id {
                            #id.write_to(#span)?;
                        }
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        })
    }

    fn add_bit_field(&mut self, field: &ast::Field) {
        let width = self.schema.field_size(field.key).static_().unwrap();
        let shift = self.shift;

        match &field.desc {
            ast::FieldDesc::Flag { optional_field_ids, .. } => {
                assert!(
                    optional_field_ids.len() == 1,
                    "condition flag reuse not supported by legacy generator"
                );

                let (optional_field_id, set_value) = &optional_field_ids[0];
                let optional_field_id = optional_field_id.to_ident();
                let cond_value_present =
                    syn::parse_str::<syn::LitInt>(&format!("{}", set_value)).unwrap();
                let cond_value_absent =
                    syn::parse_str::<syn::LitInt>(&format!("{}", 1 - set_value)).unwrap();
                self.chunk.push(BitField {
                    value: quote! {
                        if self.#optional_field_id.is_some() {
                            #cond_value_present
                        } else {
                            #cond_value_absent
                        }
                    },
                    field_type: types::Integer::new(1),
                    shift,
                });
            }
            ast::FieldDesc::Scalar { id, width } => {
                let field_name = id.to_ident();
                let field_type = types::Integer::new(*width);
                if field_type.width > *width {
                    let packet_name = &self.packet_name;
                    let max_value = mask_bits(*width, "u64");
                    self.code.push(quote! {
                        if self.#field_name > #max_value {
                            return Err(EncodeError::InvalidScalarValue {
                                packet: #packet_name,
                                field: #id,
                                value: self.#field_name as u64,
                                maximum_value: #max_value,
                            })
                        }
                    });
                }
                self.chunk.push(BitField { value: quote!(self.#field_name), field_type, shift });
            }
            ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                let field_type = types::Integer::new(width);
                let enum_id = enum_id.to_ident();
                let tag_id = format_ident!("{}", tag_id.to_upper_camel_case());
                self.chunk.push(BitField {
                    value: quote!(#field_type::from(#enum_id::#tag_id)),
                    field_type,
                    shift,
                });
            }
            ast::FieldDesc::FixedScalar { value, .. } => {
                let field_type = types::Integer::new(width);
                let value = proc_macro2::Literal::usize_unsuffixed(*value);
                self.chunk.push(BitField { value: quote!(#value), field_type, shift });
            }
            ast::FieldDesc::Typedef { id, .. } => {
                let field_name = id.to_ident();
                let field_type = types::Integer::new(width);
                self.chunk.push(BitField {
                    value: quote!(#field_type::from(self.#field_name)),
                    field_type,
                    shift,
                });
            }
            ast::FieldDesc::Reserved { .. } => {
                // Nothing to do here.
            }
            ast::FieldDesc::Size { field_id, width, .. } => {
                let packet_name = &self.packet_name;
                let max_value = mask_bits(*width, "usize");

                let decl = self.scope.typedef.get(self.packet_name).unwrap();
                let value_field = self
                    .scope
                    .iter_fields(decl)
                    .find(|field| match &field.desc {
                        ast::FieldDesc::Payload { .. } => field_id == "_payload_",
                        ast::FieldDesc::Body { .. } => field_id == "_body_",
                        _ => field.id() == Some(field_id),
                    })
                    .unwrap();

                let field_name = field_id.to_ident();
                let field_type = types::Integer::new(*width);
                // TODO: size modifier

                let value_field_decl = self.scope.get_type_declaration(value_field);

                let field_size_name = format_ident!("{field_id}_size");
                let array_size = match (&value_field.desc, value_field_decl.map(|decl| &decl.desc))
                {
                    (ast::FieldDesc::Payload { size_modifier: Some(size_modifier) }, _) => {
                        let size_modifier = proc_macro2::Literal::usize_unsuffixed(
                            size_modifier
                                .parse::<usize>()
                                .expect("failed to parse the size modifier"),
                        );
                        if let ast::DeclDesc::Packet { .. } = &decl.desc {
                            quote! { (self.child.get_total_size() + #size_modifier) }
                        } else {
                            quote! { (self.payload.len() + #size_modifier) }
                        }
                    }
                    (ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body { .. }, _) => {
                        if let ast::DeclDesc::Packet { .. } = &decl.desc {
                            quote! { self.child.get_total_size() }
                        } else {
                            quote! { self.payload.len() }
                        }
                    }
                    (ast::FieldDesc::Array { width: Some(width), .. }, _)
                    | (ast::FieldDesc::Array { .. }, Some(ast::DeclDesc::Enum { width, .. })) => {
                        let byte_width = syn::Index::from(width / 8);
                        if byte_width.index == 1 {
                            quote! { self.#field_name.len() }
                        } else {
                            quote! { (self.#field_name.len() * #byte_width) }
                        }
                    }
                    (ast::FieldDesc::Array { .. }, _) => {
                        self.code.push(quote! {
                            let #field_size_name = self.#field_name
                                .iter()
                                .map(|elem| elem.get_size())
                                .sum::<usize>();
                        });
                        quote! { #field_size_name }
                    }
                    _ => panic!("Unexpected size field: {field:?}"),
                };

                self.code.push(quote! {
                    if #array_size > #max_value {
                        return Err(EncodeError::SizeOverflow {
                            packet: #packet_name,
                            field: #field_id,
                            size: #array_size,
                            maximum_size: #max_value,
                        })
                    }
                });

                self.chunk.push(BitField {
                    value: quote!(#array_size as #field_type),
                    field_type,
                    shift,
                });
            }
            ast::FieldDesc::Count { field_id, width, .. } => {
                let field_name = field_id.to_ident();
                let field_type = types::Integer::new(*width);
                if field_type.width > *width {
                    let packet_name = &self.packet_name;
                    let max_value = mask_bits(*width, "usize");
                    self.code.push(quote! {
                        if self.#field_name.len() > #max_value {
                            return Err(EncodeError::CountOverflow {
                                packet: #packet_name,
                                field: #field_id,
                                count: self.#field_name.len(),
                                maximum_count: #max_value,
                            })
                        }
                    });
                }
                self.chunk.push(BitField {
                    value: quote!(self.#field_name.len() as #field_type),
                    field_type,
                    shift,
                });
            }
            _ => todo!("{field:?}"),
        }

        self.shift += width;
        if self.shift % 8 == 0 {
            self.pack_bit_fields()
        }
    }

    fn pack_bit_fields(&mut self) {
        assert_eq!(self.shift % 8, 0);
        let chunk_type = types::Integer::new(self.shift);
        let values = self
            .chunk
            .drain(..)
            .map(|BitField { mut value, field_type, shift }| {
                if field_type.width != chunk_type.width {
                    // We will be combining values with `|`, so we
                    // need to cast them first.
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
            [] => {
                let span = format_ident!("{}", self.span);
                let count = syn::Index::from(self.shift / 8);
                self.code.push(quote! {
                    #span.put_bytes(0, #count);
                });
            }
            [value] => {
                let put = types::put_uint(self.endianness, value, self.shift, self.span);
                self.code.push(quote! {
                    #put;
                });
            }
            _ => {
                let put = types::put_uint(self.endianness, &quote!(value), self.shift, self.span);
                self.code.push(quote! {
                    let value = #(#values)|*;
                    #put;
                });
            }
        }

        self.shift = 0;
    }

    fn add_array_field(
        &mut self,
        id: &str,
        width: Option<usize>,
        padding_size: Option<usize>,
        decl: Option<&ast::Decl>,
    ) {
        let span = format_ident!("{}", self.span);
        let serialize = match width {
            Some(width) => {
                let value = quote!(*elem);
                types::put_uint(self.endianness, &value, width, self.span)
            }
            None => {
                if let Some(ast::DeclDesc::Enum { width, .. }) = decl.map(|decl| &decl.desc) {
                    let element_type = types::Integer::new(*width);
                    types::put_uint(
                        self.endianness,
                        &quote!(#element_type::from(elem)),
                        *width,
                        self.span,
                    )
                } else {
                    quote! {
                        elem.write_to(#span)?
                    }
                }
            }
        };

        let packet_name = self.packet_name;
        let name = id;
        let id = id.to_ident();

        if let Some(padding_size) = padding_size {
            let padding_octets = padding_size / 8;
            let element_width = match &width {
                Some(width) => Some(*width),
                None => self.schema.total_size(decl.unwrap().key).static_(),
            };

            let array_size = match element_width {
                Some(element_width) => {
                    let element_size = proc_macro2::Literal::usize_unsuffixed(element_width / 8);
                    quote! { self.#id.len() * #element_size }
                }
                _ => {
                    quote! { self.#id.iter().fold(0, |size, elem| size + elem.get_size()) }
                }
            };

            self.code.push(quote! {
                let array_size = #array_size;
                if array_size > #padding_octets {
                    return Err(EncodeError::SizeOverflow {
                        packet: #packet_name,
                        field: #name,
                        size: array_size,
                        maximum_size: #padding_octets,
                    })
                }
                for elem in &self.#id {
                    #serialize;
                }
                #span.put_bytes(0, #padding_octets - array_size);
            });
        } else {
            self.code.push(quote! {
                for elem in &self.#id {
                    #serialize;
                }
            });
        }
    }

    fn add_typedef_field(&mut self, id: &str, type_id: &str) {
        assert_eq!(self.shift, 0, "Typedef field does not start on an octet boundary");
        let decl = self.scope.typedef[type_id];
        if let ast::DeclDesc::Struct { parent_id: Some(_), .. } = &decl.desc {
            panic!("Derived struct used in typedef field");
        }

        let id = id.to_ident();
        let span = format_ident!("{}", self.span);

        self.code.push(match &decl.desc {
            ast::DeclDesc::Checksum { .. } => todo!(),
            ast::DeclDesc::CustomField { width: Some(width), .. } => {
                let backing_type = types::Integer::new(*width);
                let put_uint = types::put_uint(
                    self.endianness,
                    &quote! { #backing_type::from(self.#id) },
                    *width,
                    self.span,
                );
                quote! {
                    #put_uint;
                }
            }
            ast::DeclDesc::Struct { .. } => quote! {
                self.#id.write_to(#span)?;
            },
            _ => unreachable!(),
        });
    }

    fn add_payload_field(&mut self) {
        if self.shift != 0 && self.endianness == ast::EndiannessValue::BigEndian {
            panic!("Payload field does not start on an octet boundary");
        }

        let decl = self.scope.typedef[self.packet_name];
        let is_packet = matches!(&decl.desc, ast::DeclDesc::Packet { .. });

        let child_ids = self
            .scope
            .iter_children(decl)
            .map(|child| child.id().unwrap().to_ident())
            .collect::<Vec<_>>();

        let span = format_ident!("{}", self.span);
        if self.shift == 0 {
            if is_packet {
                let packet_data_child = format_ident!("{}DataChild", self.packet_name);
                self.code.push(quote! {
                    match &self.child {
                        #(#packet_data_child::#child_ids(child) => child.write_to(#span)?,)*
                        #packet_data_child::Payload(payload) => #span.put_slice(payload),
                        #packet_data_child::None => {},
                    }
                })
            } else {
                self.code.push(quote! {
                    #span.put_slice(&self.payload);
                });
            }
        } else {
            todo!("Shifted payloads");
        }
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
