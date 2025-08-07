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

use crate::backends::rust::{mask_bits, types, ToIdent, ToUpperCamelCase};
use crate::{analyzer, ast};
use quote::{format_ident, quote};

/// Generate a range check for a scalar value backed to a rust type
/// that exceeds the actual size of the PDL field.
fn range_check(
    value: proc_macro2::TokenStream,
    width: usize,
    packet_name: &str,
    field_name: &str,
) -> proc_macro2::TokenStream {
    let max_value = mask_bits(width, "u64");
    quote! {
        if #value > #max_value {
            return Err(EncodeError::InvalidScalarValue {
                packet: #packet_name,
                field: #field_name,
                value: #value as u64,
                maximum_value: #max_value as u64,
            })
        }
    }
}

/// Represents the computed size of a packet,
/// compoased of constant and variable size fields.
struct RuntimeSize {
    constant: usize,
    variable: Vec<proc_macro2::TokenStream>,
}

impl RuntimeSize {
    fn payload_size() -> Self {
        RuntimeSize { constant: 0, variable: vec![quote! { self.payload.len() }] }
    }
}

impl std::ops::AddAssign<&RuntimeSize> for RuntimeSize {
    fn add_assign(&mut self, other: &RuntimeSize) {
        self.constant += other.constant;
        self.variable.extend_from_slice(&other.variable)
    }
}

impl quote::ToTokens for RuntimeSize {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let constant = proc_macro2::Literal::usize_unsuffixed(self.constant);
        tokens.extend(match self {
            RuntimeSize { variable, .. } if variable.is_empty() => quote! { #constant },
            RuntimeSize { variable, constant: 0 } => quote! { #(#variable)+* },
            RuntimeSize { variable, .. } => quote! { #constant + #(#variable)+* },
        })
    }
}

/// Represents part of a compound bit-field.
struct BitField {
    value: proc_macro2::TokenStream,
    field_type: types::Integer,
    shift: usize,
}

struct Encoder {
    endianness: ast::EndiannessValue,
    buf: proc_macro2::Ident,
    packet_name: String,
    packet_size: RuntimeSize,
    payload_size: RuntimeSize,
    tokens: proc_macro2::TokenStream,
    bit_shift: usize,
    bit_fields: Vec<BitField>,
}

impl Encoder {
    pub fn new(
        endianness: ast::EndiannessValue,
        packet_name: &str,
        buf: proc_macro2::Ident,
        payload_size: RuntimeSize,
    ) -> Self {
        Encoder {
            buf,
            packet_name: packet_name.to_owned(),
            endianness,
            packet_size: RuntimeSize { constant: 0, variable: vec![] },
            payload_size,
            tokens: quote! {},
            bit_shift: 0,
            bit_fields: vec![],
        }
    }

    fn encode_typedef_field(
        &mut self,
        scope: &analyzer::Scope<'_>,
        schema: &analyzer::Schema,
        id: &str,
        type_id: &str,
    ) {
        assert_eq!(self.bit_shift, 0, "Typedef field does not start on an octet boundary");

        let decl = scope.typedef[type_id];
        let id = id.to_ident();
        let buf = &self.buf;

        self.tokens.extend(match &decl.desc {
            ast::DeclDesc::Checksum { .. } => todo!(),
            ast::DeclDesc::CustomField { width: Some(width), .. } => {
                let backing_type = types::Integer::new(*width);
                let put_uint = types::put_uint(
                    self.endianness,
                    &quote! { #backing_type::from(self.#id) },
                    *width,
                    &self.buf,
                );
                quote! {
                    #put_uint;
                }
            }
            ast::DeclDesc::Struct { .. } | ast::DeclDesc::CustomField { .. } => quote! {
                self.#id.encode(#buf)?;
            },
            _ => todo!("{:?}", decl),
        });

        match schema.total_size(decl.key) {
            analyzer::Size::Static(s) => self.packet_size.constant += s / 8,
            _ => self.packet_size.variable.push(quote! { self.#id.encoded_len() }),
        }
    }

    fn encode_optional_field(
        &mut self,
        scope: &analyzer::Scope<'_>,
        _schema: &analyzer::Schema,
        field: &ast::Field,
    ) {
        assert_eq!(self.bit_shift, 0, "Optional field does not start on an octet boundary");

        self.tokens.extend(match &field.desc {
            ast::FieldDesc::Scalar { id, width } => {
                let field_name = id;
                let id = id.to_ident();
                let backing_type = types::Integer::new(*width);
                let put_uint = types::put_uint(self.endianness, &quote!(*#id), *width, &self.buf);
                let range_check = (backing_type.width > *width)
                    .then(|| range_check(quote! { *#id }, *width, &self.packet_name, field_name));
                quote! {
                    if let Some(#id) = &self.#id {
                        #range_check
                        #put_uint;
                    }
                }
            }
            ast::FieldDesc::Typedef { id, type_id } => match &scope.typedef[type_id].desc {
                ast::DeclDesc::Enum { width, .. } => {
                    let id = id.to_ident();
                    let backing_type = types::Integer::new(*width);
                    let put_uint = types::put_uint(
                        self.endianness,
                        &quote!(#backing_type::from(#id)),
                        *width,
                        &self.buf,
                    );

                    quote! {
                        if let Some(#id) = &self.#id {
                            #put_uint;
                        }
                    }
                }
                ast::DeclDesc::Struct { .. } => {
                    let id = id.to_ident();
                    let buf = &self.buf;

                    quote! {
                        if let Some(#id) = &self.#id {
                            #id.encode(#buf)?;
                        }
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        });

        self.packet_size.variable.push(match &field.desc {
            ast::FieldDesc::Scalar { id, width } => {
                let id = id.to_ident();
                let size = width / 8;
                quote! { if self.#id.is_some() { #size } else { 0 } }
            }
            ast::FieldDesc::Typedef { id, type_id } => match &scope.typedef[type_id].desc {
                ast::DeclDesc::Enum { width, .. } => {
                    let id = id.to_ident();
                    let size = width / 8;
                    quote! { if self.#id.is_some() { #size } else { 0 } }
                }
                ast::DeclDesc::Struct { .. } => {
                    let id = id.to_ident();
                    let type_id = type_id.to_ident();
                    quote! {
                        &self.#id
                            .as_ref()
                            .map(#type_id::encoded_len)
                            .unwrap_or(0)
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        })
    }

    fn encode_bit_field(
        &mut self,
        scope: &analyzer::Scope<'_>,
        schema: &analyzer::Schema,
        field: &ast::Field,
    ) {
        let width = schema.field_size(field.key).static_().unwrap();
        let shift = self.bit_shift;

        match &field.desc {
            ast::FieldDesc::Flag { optional_field_ids, .. } => {
                let packet_name = &self.packet_name;
                let field_id = field.id().unwrap();
                let (optional_field_id, set_value) = &optional_field_ids[0];
                let optional_field_id = optional_field_id.to_ident();

                let cond_value_present =
                    syn::parse_str::<syn::LitInt>(&format!("{set_value}")).unwrap();
                let cond_value_absent =
                    syn::parse_str::<syn::LitInt>(&format!("{}", 1 - set_value)).unwrap();

                if optional_field_ids.len() >= 2 {
                    self.tokens.extend(quote! {
                        let mut cond_value_is_zero = false;
                        let mut cond_value_is_one = false;
                    });

                    for (optional_cond_field, value) in optional_field_ids {
                        let optional_cond_field_tok = optional_cond_field.to_ident();
                        if *value == 1 {
                            self.tokens.extend(quote! { cond_value_is_one |= self.#optional_cond_field_tok.is_some();} );
                            self.tokens.extend(quote! { cond_value_is_zero |= self.#optional_cond_field_tok.is_none();} );
                        } else {
                            self.tokens.extend(quote! { cond_value_is_one |= self.#optional_cond_field_tok.is_none();} );
                            self.tokens.extend(quote! { cond_value_is_zero |= self.#optional_cond_field_tok.is_some();} );
                        }
                    }

                    self.tokens.extend(quote! {
                        if cond_value_is_zero && cond_value_is_one {
                            return Err(EncodeError::InconsistentConditionValue {
                                packet: #packet_name,
                                field: #field_id,
                            })
                        }
                    });
                }

                self.bit_fields.push(BitField {
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
                let field_name = id;
                let field_id = id.to_ident();
                let field_type = types::Integer::new(*width);
                if field_type.width > *width {
                    self.tokens.extend(range_check(
                        quote! { self.#field_id() },
                        *width,
                        &self.packet_name,
                        field_name,
                    ));
                }
                self.bit_fields.push(BitField {
                    value: quote! { self.#field_id() },
                    field_type,
                    shift,
                });
            }
            ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                let field_type = types::Integer::new(width);
                let enum_id = enum_id.to_ident();
                let tag_id = format_ident!("{}", tag_id.to_upper_camel_case());
                self.bit_fields.push(BitField {
                    value: quote!(#field_type::from(#enum_id::#tag_id)),
                    field_type,
                    shift,
                });
            }
            ast::FieldDesc::FixedScalar { value, .. } => {
                let field_type = types::Integer::new(width);
                let value = proc_macro2::Literal::usize_unsuffixed(*value);
                self.bit_fields.push(BitField { value: quote!(#value), field_type, shift });
            }
            ast::FieldDesc::Typedef { id, .. } => {
                let id = id.to_ident();
                let field_type = types::Integer::new(width);
                self.bit_fields.push(BitField {
                    value: quote!(#field_type::from(self.#id())),
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

                let decl = scope.typedef.get(&self.packet_name).unwrap();
                let value_field = scope
                    .iter_fields(decl)
                    .find(|field| match &field.desc {
                        ast::FieldDesc::Payload { .. } => field_id == "_payload_",
                        ast::FieldDesc::Body => field_id == "_body_",
                        _ => field.id() == Some(field_id),
                    })
                    .unwrap();

                let field_name = field_id.to_ident();
                let field_type = types::Integer::new(*width);
                // TODO: size modifier

                let value_field_decl = scope.get_type_declaration(value_field);
                let array_size = match (&value_field.desc, value_field_decl.map(|decl| &decl.desc))
                {
                    (ast::FieldDesc::Payload { size_modifier: Some(size_modifier) }, _) => {
                        let size_modifier = proc_macro2::Literal::usize_unsuffixed(
                            size_modifier
                                .parse::<usize>()
                                .expect("failed to parse the size modifier"),
                        );
                        let payload_size = &self.payload_size;
                        quote! { (#payload_size + #size_modifier) }
                    }
                    (ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body, _) => {
                        let payload_size = &self.payload_size;
                        quote! { #payload_size }
                    }
                    (ast::FieldDesc::Array { width: Some(width), .. }, _)
                    | (ast::FieldDesc::Array { .. }, Some(ast::DeclDesc::Enum { width, .. })) => {
                        let size = width / 8;
                        if size == 1 {
                            quote! { self.#field_name.len() }
                        } else {
                            let size = proc_macro2::Literal::usize_unsuffixed(size);
                            quote! { (self.#field_name.len() * #size) }
                        }
                    }
                    (ast::FieldDesc::Array { .. }, _) => {
                        let field_size_name = format_ident!("{field_id}_size");
                        self.tokens.extend(quote! {
                            let #field_size_name = self.#field_name
                                .iter()
                                .map(Packet::encoded_len)
                                .sum::<usize>();
                        });
                        quote! { #field_size_name }
                    }
                    _ => panic!("Unexpected size field: {field:?}"),
                };

                self.tokens.extend(quote! {
                    if #array_size > #max_value {
                        return Err(EncodeError::SizeOverflow {
                            packet: #packet_name,
                            field: #field_id,
                            size: #array_size,
                            maximum_size: #max_value,
                        })
                    }
                });

                self.bit_fields.push(BitField {
                    value: quote!((#array_size) as #field_type),
                    field_type,
                    shift,
                });
            }
            ast::FieldDesc::ElementSize { field_id, width, .. } => {
                let field_name = field_id.to_ident();
                let field_type = types::Integer::new(*width);
                let field_element_size_name = format_ident!("{field_id}_element_size");
                let packet_name = &self.packet_name;
                let max_value = mask_bits(*width, "usize");
                self.tokens.extend(quote! {
                    let #field_element_size_name = self.#field_name
                        .get(0)
                        .map_or(0, Packet::encoded_len);

                    for (element_index, element) in self.#field_name.iter().enumerate() {
                        if element.encoded_len() != #field_element_size_name {
                            return Err(EncodeError::InvalidArrayElementSize {
                                packet: #packet_name,
                                field: #field_id,
                                size: element.encoded_len(),
                                expected_size: #field_element_size_name,
                                element_index,
                            })
                        }
                    }
                    if #field_element_size_name > #max_value {
                        return Err(EncodeError::SizeOverflow {
                            packet: #packet_name,
                            field: #field_id,
                            size: #field_element_size_name,
                            maximum_size: #max_value,
                        })
                    }
                    let #field_element_size_name = #field_element_size_name as #field_type;
                });
                self.bit_fields.push(BitField {
                    value: quote!(#field_element_size_name),
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
                    self.tokens.extend(quote! {
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
                self.bit_fields.push(BitField {
                    value: quote!(self.#field_name.len() as #field_type),
                    field_type,
                    shift,
                });
            }
            _ => todo!("{field:?}"),
        }

        self.bit_shift += width;
        if self.bit_shift % 8 == 0 {
            self.pack_bit_fields()
        }
    }

    fn pack_bit_fields(&mut self) {
        assert_eq!(self.bit_shift % 8, 0);
        let chunk_type = types::Integer::new(self.bit_shift);
        let values = self
            .bit_fields
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

        self.tokens.extend(match values.as_slice() {
            [] => {
                let buf = format_ident!("{}", self.buf);
                let count = proc_macro2::Literal::usize_unsuffixed(self.bit_shift / 8);
                quote! {
                    #buf.put_bytes(0, #count);
                }
            }
            [value] => {
                let put = types::put_uint(self.endianness, value, self.bit_shift, &self.buf);
                quote! {
                    #put;
                }
            }
            _ => {
                let put =
                    types::put_uint(self.endianness, &quote!(value), self.bit_shift, &self.buf);
                quote! {
                    let value = #(#values)|*;
                    #put;
                }
            }
        });

        self.packet_size.constant += self.bit_shift / 8;
        self.bit_shift = 0;
    }

    fn encode_array_field(
        &mut self,
        _scope: &analyzer::Scope<'_>,
        schema: &analyzer::Schema,
        id: &str,
        width: Option<usize>,
        padding_size: Option<usize>,
        decl: Option<&ast::Decl>,
    ) {
        assert_eq!(self.bit_shift, 0, "Array field does not start on an octet boundary");

        let buf = &self.buf;

        // Code to encode one array element.
        let put_element = match width {
            Some(width) => {
                let value = quote!(*elem);
                types::put_uint(self.endianness, &value, width, &self.buf)
            }
            None => {
                if let Some(ast::DeclDesc::Enum { width, .. }) = decl.map(|decl| &decl.desc) {
                    let element_type = types::Integer::new(*width);
                    types::put_uint(
                        self.endianness,
                        &quote!(#element_type::from(elem)),
                        *width,
                        &self.buf,
                    )
                } else {
                    quote! {
                        elem.encode(#buf)?
                    }
                }
            }
        };

        let packet_name = &self.packet_name;
        let field_name = id;
        let id = id.to_ident();

        let element_width = match &width {
            Some(width) => Some(*width),
            None => schema.total_size(decl.unwrap().key).static_(),
        };

        let array_size = match element_width {
            Some(8) => quote! { self.#id.len() },
            Some(element_width) => {
                let element_size = proc_macro2::Literal::usize_unsuffixed(element_width / 8);
                quote! { (self.#id.len() * #element_size) }
            }
            _ => {
                quote! {
                    self.#id
                        .iter()
                        .map(Packet::encoded_len)
                        .sum::<usize>()
                }
            }
        };

        self.tokens.extend(if let Some(padding_size) = padding_size {
            let padding_octets = padding_size / 8;
            quote! {
                let array_size = #array_size;
                if array_size > #padding_octets {
                    return Err(EncodeError::SizeOverflow {
                        packet: #packet_name,
                        field: #field_name,
                        size: array_size,
                        maximum_size: #padding_octets,
                    })
                }
                for elem in &self.#id {
                    #put_element;
                }
                #buf.put_bytes(0, #padding_octets - array_size);
            }
        } else {
            quote! {
                for elem in &self.#id {
                    #put_element;
                }
            }
        });

        if let Some(padding_size) = padding_size {
            self.packet_size.constant += padding_size / 8;
        } else {
            self.packet_size.variable.push(array_size);
        }
    }

    fn encode_field(
        &mut self,
        scope: &analyzer::Scope<'_>,
        schema: &analyzer::Schema,
        payload: &proc_macro2::TokenStream,
        field: &ast::Field,
    ) {
        match &field.desc {
            _ if field.cond.is_some() => self.encode_optional_field(scope, schema, field),
            _ if scope.is_bitfield(field) => self.encode_bit_field(scope, schema, field),
            ast::FieldDesc::Array { id, width, .. } => self.encode_array_field(
                scope,
                schema,
                id,
                *width,
                schema.padded_size(field.key),
                scope.get_type_declaration(field),
            ),
            ast::FieldDesc::Typedef { id, type_id } => {
                self.encode_typedef_field(scope, schema, id, type_id)
            }
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                self.tokens.extend(payload.clone());
                self.packet_size += &self.payload_size
            }
            // Padding field handled in serialization of associated array field.
            ast::FieldDesc::Padding { .. } => (),
            _ => todo!("Cannot yet serialize {field:?}"),
        }
    }
}

fn encode_with_parents(
    scope: &analyzer::Scope<'_>,
    schema: &analyzer::Schema,
    endianness: ast::EndiannessValue,
    buf: proc_macro2::Ident,
    decl: &ast::Decl,
    payload_size: RuntimeSize,
    payload: proc_macro2::TokenStream,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let packet_name = decl.id().unwrap();
    let mut encoder = Encoder::new(endianness, packet_name, buf.clone(), payload_size);
    for field in decl.fields() {
        encoder.encode_field(scope, schema, &payload, field);
    }

    match scope.get_parent(decl) {
        Some(parent_decl) => encode_with_parents(
            scope,
            schema,
            endianness,
            buf,
            parent_decl,
            encoder.packet_size,
            encoder.tokens,
        ),
        None => {
            let packet_size = encoder.packet_size;
            (encoder.tokens, quote! { #packet_size })
        }
    }
}

pub fn encode(
    scope: &analyzer::Scope<'_>,
    schema: &analyzer::Schema,
    endianness: ast::EndiannessValue,
    buf: proc_macro2::Ident,
    decl: &ast::Decl,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    encode_with_parents(
        scope,
        schema,
        endianness,
        buf.clone(),
        decl,
        RuntimeSize::payload_size(),
        quote! { #buf.put_slice(&self.payload); },
    )
}

pub fn encode_partial(
    scope: &analyzer::Scope<'_>,
    schema: &analyzer::Schema,
    endianness: ast::EndiannessValue,
    buf: proc_macro2::Ident,
    decl: &ast::Decl,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let parent_decl = scope.get_parent(decl).unwrap();

    let mut encoder =
        Encoder::new(endianness, decl.id().unwrap(), buf.clone(), RuntimeSize::payload_size());

    for field in decl.fields() {
        encoder.encode_field(scope, schema, &quote! { #buf.put_slice(&self.payload); }, field);
    }

    let (encode_parents, encoded_len) = encode_with_parents(
        scope,
        schema,
        endianness,
        buf,
        parent_decl,
        encoder.packet_size,
        quote! { self.encode_partial(buf)?; },
    );

    (encoder.tokens, encode_parents, encoded_len)
}
