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

fn size_field_ident(id: &str) -> proc_macro2::Ident {
    format_ident!("{}_size", id.trim_matches('_'))
}

/// A single bit-field.
struct BitField<'a> {
    shift: usize, // The shift to apply to this field.
    field: &'a ast::Field,
}

pub struct FieldParser<'a> {
    scope: &'a analyzer::Scope<'a>,
    schema: &'a analyzer::Schema,
    endianness: ast::EndiannessValue,
    decl: &'a ast::Decl,
    packet_name: &'a str,
    span: &'a proc_macro2::Ident,
    chunk: Vec<BitField<'a>>,
    tokens: proc_macro2::TokenStream,
    shift: usize,
    offset: usize,
}

impl<'a> FieldParser<'a> {
    pub fn new(
        scope: &'a analyzer::Scope<'a>,
        schema: &'a analyzer::Schema,
        endianness: ast::EndiannessValue,
        packet_name: &'a str,
        span: &'a proc_macro2::Ident,
    ) -> FieldParser<'a> {
        FieldParser {
            scope,
            schema,
            endianness,
            decl: scope.typedef[packet_name],
            packet_name,
            span,
            chunk: Vec::new(),
            tokens: quote! {},
            shift: 0,
            offset: 0,
        }
    }

    pub fn add(&mut self, field: &'a ast::Field) {
        match &field.desc {
            _ if field.cond.is_some() => self.add_optional_field(field),
            _ if self.scope.is_bitfield(field) => self.add_bit_field(field),
            ast::FieldDesc::Padding { .. } => (),
            ast::FieldDesc::Array { id, width, type_id, size, .. } => self.add_array_field(
                id,
                *width,
                type_id.as_deref(),
                *size,
                self.schema.padded_size(field.key),
                self.scope.get_type_declaration(field),
            ),
            ast::FieldDesc::Typedef { id, type_id } => self.add_typedef_field(id, type_id),
            ast::FieldDesc::Payload { size_modifier, .. } => {
                self.add_payload_field(size_modifier.as_deref())
            }
            ast::FieldDesc::Body => self.add_payload_field(None),
            _ => todo!("{field:?}"),
        }
    }

    fn add_optional_field(&mut self, field: &'a ast::Field) {
        let cond_id = field.cond.as_ref().unwrap().id.to_ident();
        let cond_value = syn::parse_str::<syn::LitInt>(&format!(
            "{}",
            field.cond.as_ref().unwrap().value.unwrap()
        ))
        .unwrap();

        self.tokens.extend(match &field.desc {
            ast::FieldDesc::Scalar { id, width } => {
                let id = id.to_ident();
                let value = types::get_uint(self.endianness, *width, self.span);
                quote! {
                    let #id = (#cond_id == #cond_value).then(|| #value);
                }
            }
            ast::FieldDesc::Typedef { id, type_id } => match &self.scope.typedef[type_id].desc {
                ast::DeclDesc::Enum { width, .. } => {
                    let name = id;
                    let type_name = type_id;
                    let id = id.to_ident();
                    let type_id = type_id.to_ident();
                    let decl_id = &self.packet_name;
                    let value = types::get_uint(self.endianness, *width, self.span);
                    quote! {
                        let #id = (#cond_id == #cond_value)
                            .then(||
                                #type_id::try_from(#value).map_err(|unknown_val| {
                                    DecodeError::InvalidEnumValueError {
                                        obj: #decl_id,
                                        field: #name,
                                        value: unknown_val as u64,
                                        type_: #type_name,
                                    }
                                }))
                            .transpose()?;
                    }
                }
                ast::DeclDesc::Struct { .. } => {
                    let id = id.to_ident();
                    let type_id = type_id.to_ident();
                    let span = self.span;
                    quote! {
                        let #id = (#cond_id == #cond_value)
                            .then(|| #type_id::decode_mut(&mut #span))
                            .transpose()?;
                    }
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        })
    }

    fn add_bit_field(&mut self, field: &'a ast::Field) {
        self.chunk.push(BitField { shift: self.shift, field });
        self.shift += self.schema.field_size(field.key).static_().unwrap();
        if self.shift % 8 != 0 {
            return;
        }

        let size = self.shift / 8;
        let end_offset = self.offset + size;

        let wanted = proc_macro2::Literal::usize_unsuffixed(size);
        self.check_size(self.span, &quote!(#wanted));

        let chunk_type = types::Integer::new(self.shift);
        // TODO(mgeisler): generate Rust variable names which cannot
        // conflict with PDL field names. An option would be to start
        // Rust variable names with `_`, but that has a special
        // semantic in Rust.
        let chunk_name = format_ident!("chunk");

        let get = types::get_uint(self.endianness, self.shift, self.span);
        if self.chunk.len() > 1 {
            // Multiple values: we read into a local variable.
            self.tokens.extend(quote! {
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

            let width = self.schema.field_size(field.key).static_().unwrap();
            let value_type = types::Integer::new(width);
            if !single_value && width < value_type.width {
                // Mask value if we grabbed more than `width` and if
                // `as #value_type` doesn't already do the masking.
                let mask = mask_bits(width, "u64");
                v = quote! { (#v & #mask) };
            }

            if value_type.width < chunk_type.width {
                v = quote! { #v as #value_type };
            }

            self.tokens.extend(match &field.desc {
                ast::FieldDesc::Scalar { id, .. }
                | ast::FieldDesc::Flag { id, .. } => {
                    let id = id.to_ident();
                    quote! {
                        let #id = #v;
                    }
                }
                ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                    let enum_id = enum_id.to_ident();
                    let tag_id = tag_id.to_upper_camel_case().to_ident();
                    quote! {
                        let fixed_value = #v;
                        if fixed_value != #value_type::from(#enum_id::#tag_id)  {
                            return Err(DecodeError::InvalidFixedValue {
                                expected: #value_type::from(#enum_id::#tag_id) as u64,
                                actual: fixed_value as u64,
                            });
                        }
                    }
                }
                ast::FieldDesc::FixedScalar { value, .. } => {
                    let value = proc_macro2::Literal::usize_unsuffixed(*value);
                    quote! {
                        let fixed_value = #v;
                        if fixed_value != #value {
                            return Err(DecodeError::InvalidFixedValue {
                                expected: #value,
                                actual: fixed_value as u64,
                            });
                        }
                    }
                }
                ast::FieldDesc::Typedef { id, type_id } => {
                    let field_name = id;
                    let type_name = type_id;
                    let packet_name = &self.packet_name;
                    let id = id.to_ident();
                    let type_id = type_id.to_ident();
                    quote! {
                        let #id = #type_id::try_from(#v).map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                            obj: #packet_name,
                            field: #field_name,
                            value: unknown_val as u64,
                            type_: #type_name,
                        })?;
                    }
                }
                ast::FieldDesc::Reserved { .. } => {
                    if single_value {
                        let span = self.span;
                        let size = proc_macro2::Literal::usize_unsuffixed(size);
                        quote! {
                            #span.advance(#size);
                        }
                    } else {
                        //  Otherwise we don't need anything: we will
                        //  have advanced past the reserved field when
                        //  reading the chunk above.
                        quote! {}
                    }
                }
                ast::FieldDesc::Size { field_id, .. } => {
                    let id = size_field_ident(field_id);
                    quote! {
                        let #id = #v as usize;
                    }
                }
                ast::FieldDesc::ElementSize { field_id, .. } => {
                    let id = format_ident!("{field_id}_element_size");
                    quote! {
                        let #id = #v as usize;
                    }
                }
                ast::FieldDesc::Count { field_id, .. } => {
                    let id = format_ident!("{field_id}_count");
                    quote! {
                        let #id = #v as usize;
                    }
                }
                _ => todo!(),
            });
        }

        self.offset = end_offset;
        self.shift = 0;
    }

    fn find_count_field(&self, id: &str) -> Option<proc_macro2::Ident> {
        match self.decl.array_size(id)?.desc {
            ast::FieldDesc::Count { .. } => Some(format_ident!("{id}_count")),
            _ => None,
        }
    }

    fn find_size_field(&self, id: &str) -> Option<proc_macro2::Ident> {
        match self.decl.array_size(id)?.desc {
            ast::FieldDesc::Size { .. } => Some(size_field_ident(id)),
            _ => None,
        }
    }

    fn find_element_size_field(&self, id: &str) -> Option<proc_macro2::Ident> {
        self.decl.fields().find_map(|field| match &field.desc {
            ast::FieldDesc::ElementSize { field_id, .. } if field_id == id => {
                Some(format_ident!("{id}_element_size"))
            }
            _ => None,
        })
    }

    fn payload_field_offset_from_end(&self) -> Option<usize> {
        let decl = self.scope.typedef[self.packet_name];
        let mut fields = decl.fields();
        fields.find(|f| matches!(f.desc, ast::FieldDesc::Body | ast::FieldDesc::Payload { .. }))?;

        let mut offset = 0;
        for field in fields {
            if let Some(width) =
                self.schema.padded_size(field.key).or(self.schema.field_size(field.key).static_())
            {
                offset += width;
            } else {
                return None;
            }
        }

        Some(offset)
    }

    fn check_size(&mut self, span: &proc_macro2::Ident, wanted: &proc_macro2::TokenStream) {
        let packet_name = &self.packet_name;
        self.tokens.extend(quote! {
            if #span.remaining() < #wanted {
                return Err(DecodeError::InvalidLengthError {
                    obj: #packet_name,
                    wanted: #wanted,
                    got: #span.remaining(),
                });
            }
        });
    }

    fn add_array_field(
        &mut self,
        id: &str,
        // `width`: the width in bits of the array elements (if Some).
        width: Option<usize>,
        // `type_id`: the enum type of the array elements (if Some).
        // Mutually exclusive with `width`.
        type_id: Option<&str>,
        // `size`: the size of the array in number of elements (if
        // known). If None, the array is a Vec with a dynamic size.
        size: Option<usize>,
        padding_size: Option<usize>,
        decl: Option<&ast::Decl>,
    ) {
        enum ElementWidth {
            Static(usize),               // Static size in bytes.
            Dynamic(proc_macro2::Ident), // Dynamic size in bytes.
            Unknown,
        }
        let element_width = if let Some(w) =
            width.or_else(|| self.schema.total_size(decl.unwrap().key).static_())
        {
            assert_eq!(w % 8, 0, "Array element size ({w}) is not a multiple of 8");
            ElementWidth::Static(w / 8)
        } else if let Some(element_size_field) = self.find_element_size_field(id) {
            ElementWidth::Dynamic(element_size_field)
        } else {
            ElementWidth::Unknown
        };

        // The "shape" of the array, i.e., the number of elements
        // given via a static count, a count field, a size field, or
        // unknown.
        enum ArrayShape {
            Static(usize),                  // Static count
            CountField(proc_macro2::Ident), // Count based on count field
            SizeField(proc_macro2::Ident),  // Count based on size and field
            Unknown,                        // Variable count based on remaining bytes
        }
        let array_shape = if let Some(count) = size {
            ArrayShape::Static(count)
        } else if let Some(count_field) = self.find_count_field(id) {
            ArrayShape::CountField(count_field)
        } else if let Some(size_field) = self.find_size_field(id) {
            ArrayShape::SizeField(size_field)
        } else {
            ArrayShape::Unknown
        };

        // TODO size modifier

        let span = match padding_size {
            Some(padding_size) => {
                let span = self.span;
                let padding_octets = padding_size / 8;
                self.check_size(span, &quote!(#padding_octets));
                self.tokens.extend(quote! {
                    let (mut head, tail) = #span.split_at(#padding_octets);
                    #span = tail;
                });
                format_ident!("head")
            }
            None => self.span.clone(),
        };

        let field_name = id;
        let packet_name = self.packet_name;
        let id = id.to_ident();

        let parse_element = self.parse_array_element(&span, width, type_id, decl);
        match (element_width, &array_shape) {
            (ElementWidth::Unknown, ArrayShape::SizeField(size_field)) => {
                // The element width is not known, but the array full
                // octet size is known by size field. Parse elements
                // item by item as a vector.
                self.check_size(&span, &quote!(#size_field));
                let parse_element =
                    self.parse_array_element(&format_ident!("head"), width, type_id, decl);
                self.tokens.extend(quote! {
                    let (mut head, tail) = #span.split_at(#size_field);
                    #span = tail;
                    let mut #id = Vec::new();
                    while !head.is_empty() {
                        #id.push(#parse_element?);
                    }
                });
            }
            (ElementWidth::Unknown, ArrayShape::Static(count)) => {
                // The element width is not known, but the array
                // element count is known statically. Parse elements
                // item by item as an array.
                let count = proc_macro2::Literal::usize_unsuffixed(*count);
                self.tokens.extend(quote! {
                    // TODO(mgeisler): use
                    // https://doc.rust-lang.org/std/array/fn.try_from_fn.html
                    // when stabilized.
                    let mut #id = Vec::with_capacity(#count);
                    for _ in 0..#count {
                        #id.push(#parse_element?)
                    }
                    let #id = #id
                        .try_into()
                        .map_err(|_| DecodeError::InvalidPacketError)?;
                });
            }
            (ElementWidth::Unknown, ArrayShape::CountField(count_field)) => {
                // The element width is not known, but the array
                // element count is known by the count field. Parse
                // elements item by item as a vector.
                self.tokens.extend(quote! {
                    let #id = (0..#count_field)
                        .map(|_| #parse_element)
                        .collect::<Result<Vec<_>, DecodeError>>()?;
                });
            }
            (ElementWidth::Unknown, ArrayShape::Unknown) => {
                // Neither the count not size is known, parse elements
                // until the end of the span.
                self.tokens.extend(quote! {
                    let mut #id = Vec::new();
                    while !#span.is_empty() {
                        #id.push(#parse_element?);
                    }
                });
            }
            (ElementWidth::Static(element_width), ArrayShape::Static(count)) => {
                // The element width is known, and the array element
                // count is known statically.
                let count = proc_macro2::Literal::usize_unsuffixed(*count);
                // This creates a nicely formatted size.
                let array_size = if element_width == 1 {
                    quote!(#count)
                } else {
                    let element_width = proc_macro2::Literal::usize_unsuffixed(element_width);
                    quote!(#count * #element_width)
                };
                self.check_size(&span, &quote! { #array_size });
                self.tokens.extend(quote! {
                    // TODO(mgeisler): use
                    // https://doc.rust-lang.org/std/array/fn.try_from_fn.html
                    // when stabilized.
                    let mut #id = Vec::with_capacity(#count);
                    for _ in 0..#count {
                        #id.push(#parse_element?)
                    }
                    let #id = #id
                        .try_into()
                        .map_err(|_| DecodeError::InvalidPacketError)?;
                });
            }
            (ElementWidth::Static(element_width), ArrayShape::CountField(count_field)) => {
                // The element width is known, and the array element
                // count is known dynamically by the count field.
                self.check_size(&span, &quote!(#count_field * #element_width));
                self.tokens.extend(quote! {
                    let #id = (0..#count_field)
                        .map(|_| #parse_element)
                        .collect::<Result<Vec<_>, DecodeError>>()?;
                });
            }
            (ElementWidth::Static(element_width), ArrayShape::SizeField(_))
            | (ElementWidth::Static(element_width), ArrayShape::Unknown) => {
                // The element width is known, and the array full size
                // is known by size field, or unknown (in which case
                // it is the remaining span length).
                let array_size = if let ArrayShape::SizeField(size_field) = &array_shape {
                    self.check_size(&span, &quote!(#size_field));
                    quote!(#size_field)
                } else {
                    quote!(#span.remaining())
                };
                let count_field = format_ident!("{id}_count");
                let array_count = if element_width != 1 {
                    let element_width = proc_macro2::Literal::usize_unsuffixed(element_width);
                    self.tokens.extend(quote! {
                        if #array_size % #element_width != 0 {
                            return Err(DecodeError::InvalidArraySize {
                                array: #array_size,
                                element: #element_width,
                            });
                        }
                        let #count_field = #array_size / #element_width;
                    });
                    quote!(#count_field)
                } else {
                    array_size
                };

                self.tokens.extend(quote! {
                    let mut #id = Vec::with_capacity(#array_count);
                    for _ in 0..#array_count {
                        #id.push(#parse_element?);
                    }
                });
            }
            (ElementWidth::Dynamic(element_size_field), ArrayShape::Static(count)) => {
                // The element width is known, and the array element
                // count is known statically.
                let array_size = if *count == 1 {
                    quote!(#element_size_field)
                } else {
                    quote!(#count * #element_size_field)
                };

                self.check_size(&span, &array_size);

                let parse_element =
                    self.parse_array_element(&format_ident!("chunk"), width, type_id, decl);

                self.tokens.extend(quote! {
                    // TODO: use
                    // https://doc.rust-lang.org/std/array/fn.try_from_fn.html
                    // when stabilized.
                    let #id = #span.chunks(#element_size_field)
                        .take(#count)
                        .map(|mut chunk| #parse_element.and_then(|value| {
                            if chunk.is_empty() {
                                Ok(value)
                            } else {
                                Err(DecodeError::TrailingBytesInArray {
                                    obj: #packet_name,
                                    field: #field_name,
                                })
                            }
                         }))
                        .collect::<Result<Vec<_>, DecodeError>>()?;
                    #span = &#span[#array_size..];
                    let #id = #id
                        .try_into()
                        .map_err(|_| DecodeError::InvalidPacketError)?;
                });
            }
            (ElementWidth::Dynamic(element_size_field), ArrayShape::CountField(count_field)) => {
                // The element width is known, and the array element
                // count is known dynamically by the count field.
                self.check_size(&span, &quote!(#count_field * #element_size_field));

                let parse_element =
                    self.parse_array_element(&format_ident!("chunk"), width, type_id, decl);

                self.tokens.extend(quote! {
                    let #id = #span.chunks(#element_size_field)
                        .take(#count_field)
                        .map(|mut chunk| #parse_element.and_then(|value| {
                            if chunk.is_empty() {
                                Ok(value)
                            } else {
                                Err(DecodeError::TrailingBytesInArray {
                                    obj: #packet_name,
                                    field: #field_name,
                                })
                            }
                         }))
                        .collect::<Result<Vec<_>, DecodeError>>()?;
                    #span = &#span[(#element_size_field * #count_field)..];
                });
            }
            (ElementWidth::Dynamic(element_size_field), ArrayShape::SizeField(_))
            | (ElementWidth::Dynamic(element_size_field), ArrayShape::Unknown) => {
                // The element width is known, and the array full size
                // is known by size field, or unknown (in which case
                // it is the remaining span length).
                let array_size = if let ArrayShape::SizeField(size_field) = &array_shape {
                    self.check_size(&span, &quote!(#size_field));
                    quote!(#size_field)
                } else {
                    quote!(#span.remaining())
                };
                self.tokens.extend(quote! {
                    if #array_size % #element_size_field != 0 {
                        return Err(DecodeError::InvalidArraySize {
                            array: #array_size,
                            element: #element_size_field,
                        });
                    }
                });

                let parse_element =
                    self.parse_array_element(&format_ident!("chunk"), width, type_id, decl);

                self.tokens.extend(quote! {
                    let #id = #span.chunks(#element_size_field)
                        .take(#array_size / #element_size_field)
                        .map(|mut chunk| #parse_element.and_then(|value| {
                            if chunk.is_empty() {
                                Ok(value)
                            } else {
                                Err(DecodeError::TrailingBytesInArray {
                                    obj: #packet_name,
                                    field: #field_name,
                                })
                            }
                         }))
                        .collect::<Result<Vec<_>, DecodeError>>()?;
                    #span = &#span[#array_size..];
                });
            }
        }
    }

    /// Parse typedef fields.
    ///
    /// This is only for non-enum fields: enums are parsed via
    /// add_bit_field.
    fn add_typedef_field(&mut self, id: &str, type_id: &str) {
        assert_eq!(self.shift, 0, "Typedef field does not start on an octet boundary");

        let decl = self.scope.typedef[type_id];
        let span = self.span;
        let id = id.to_ident();
        let type_id = type_id.to_ident();

        self.tokens.extend(match self.schema.total_size(decl.key) {
            analyzer::Size::Unknown | analyzer::Size::Dynamic => quote! {
                let (#id, mut #span) = #type_id::decode(#span)?;
            },
            analyzer::Size::Static(width) => {
                assert_eq!(width % 8, 0, "Typedef field type size is not a multiple of 8");
                match &decl.desc {
                    ast::DeclDesc::Checksum { .. } => todo!(),
                    ast::DeclDesc::CustomField { .. } if [8, 16, 32, 64].contains(&width) => {
                        let get_uint = types::get_uint(self.endianness, width, span);
                        quote! {
                            let #id = #get_uint.into();
                        }
                    }
                    ast::DeclDesc::CustomField { .. } => {
                        let get_uint = types::get_uint(self.endianness, width, span);
                        quote! {
                            let #id = (#get_uint)
                                .try_into()
                                .unwrap(); // Value is masked and conversion must succeed.
                        }
                    }
                    ast::DeclDesc::Struct { .. } => {
                        quote! {
                            let (#id, mut #span) = #type_id::decode(#span)?;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        });
    }

    /// Parse body and payload fields.
    fn add_payload_field(&mut self, size_modifier: Option<&str>) {
        let span = self.span;
        let payload_size_field = self.decl.payload_size();
        let offset_from_end = self.payload_field_offset_from_end();

        if self.shift != 0 {
            todo!("Unexpected non byte aligned payload");
        }

        if let Some(ast::FieldDesc::Size { field_id, .. }) = &payload_size_field.map(|f| &f.desc) {
            // The payload or body has a known size. Consume the
            // payload and update the span in case fields are placed
            // after the payload.
            let size_field = size_field_ident(field_id);
            if let Some(size_modifier) = size_modifier {
                let size_modifier = proc_macro2::Literal::usize_unsuffixed(
                    size_modifier.parse::<usize>().expect("failed to parse the size modifier"),
                );
                let packet_name = &self.packet_name;
                // Push code to check that the size is greater than the size
                // modifier. Required to safely substract the modifier from the
                // size.
                self.tokens.extend(quote! {
                    if #size_field < #size_modifier {
                        return Err(DecodeError::InvalidLengthError {
                            obj: #packet_name,
                            wanted: #size_modifier,
                            got: #size_field,
                        });
                    }
                    let #size_field = #size_field - #size_modifier;
                });
            }
            self.check_size(self.span, &quote!(#size_field ));
            self.tokens.extend(quote! {
                let payload = #span[..#size_field].to_vec();
                #span.advance(#size_field);
            });
        } else if offset_from_end == Some(0) {
            // The payload or body is the last field of a packet,
            // consume the remaining span.
            self.tokens.extend(quote! {
                let payload = #span.to_vec();
                #span.advance(payload.len());
            });
        } else if let Some(offset_from_end) = offset_from_end {
            // The payload or body is followed by fields of static
            // size. Consume the span that is not reserved for the
            // following fields.
            assert_eq!(
                offset_from_end % 8,
                0,
                "Payload field offset from end of packet is not a multiple of 8"
            );
            let offset_from_end = proc_macro2::Literal::usize_unsuffixed(offset_from_end / 8);
            self.check_size(self.span, &quote!(#offset_from_end));
            self.tokens.extend(quote! {
                let payload = #span[..#span.len() - #offset_from_end].to_vec();
                #span.advance(payload.len());
            });
        }

        let decl = self.scope.typedef[self.packet_name];
        if let ast::DeclDesc::Struct { .. } = &decl.desc {
            self.tokens.extend(quote! {
                let payload = Vec::from(payload);
            });
        }
    }

    /// Parse a single array field element from `span`.
    fn parse_array_element(
        &self,
        span: &proc_macro2::Ident,
        width: Option<usize>,
        type_id: Option<&str>,
        decl: Option<&ast::Decl>,
    ) -> proc_macro2::TokenStream {
        if let Some(width) = width {
            let get_uint = types::get_uint(self.endianness, width, span);
            return quote! {
                Ok::<_, DecodeError>(#get_uint)
            };
        }

        if let Some(ast::DeclDesc::Enum { id, width, .. }) = decl.map(|decl| &decl.desc) {
            let get_uint = types::get_uint(self.endianness, *width, span);
            let type_id = id.to_ident();
            let packet_name = &self.packet_name;
            return quote! {
                #type_id::try_from(#get_uint).map_err(|unknown_val| DecodeError::InvalidEnumValueError {
                    obj: #packet_name,
                    field: "", // TODO(mgeisler): fill out or remove
                    value: unknown_val as u64,
                    type_: #id,
                })
            };
        }

        let type_id = type_id.unwrap().to_ident();
        quote! {
            #type_id::decode_mut(&mut #span)
        }
    }
}

impl quote::ToTokens for FieldParser<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.tokens.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer;
    use crate::ast;
    use crate::parser::parse_inline;

    /// Parse a string fragment as a PDL file.
    ///
    /// # Panics
    ///
    /// Panics on parse errors.
    pub fn parse_str(text: &str) -> ast::File {
        let mut db = ast::SourceDatabase::new();
        let file = parse_inline(&mut db, "stdin", String::from(text)).expect("parse error");
        analyzer::analyze(&file).expect("analyzer error")
    }

    #[test]
    fn test_find_fields_static() {
        let code = "
              little_endian_packets
              packet P {
                a: 24[3],
              }
            ";
        let file = parse_str(code);
        let scope = analyzer::Scope::new(&file).unwrap();
        let schema = analyzer::Schema::new(&file);
        let span = format_ident!("bytes");
        let parser = FieldParser::new(&scope, &schema, file.endianness.value, "P", &span);
        assert_eq!(parser.find_size_field("a"), None);
        assert_eq!(parser.find_count_field("a"), None);
    }

    #[test]
    fn test_find_fields_dynamic_count() {
        let code = "
              little_endian_packets
              packet P {
                _count_(b): 24,
                b: 16[],
              }
            ";
        let file = parse_str(code);
        let scope = analyzer::Scope::new(&file).unwrap();
        let schema = analyzer::Schema::new(&file);
        let span = format_ident!("bytes");
        let parser = FieldParser::new(&scope, &schema, file.endianness.value, "P", &span);
        assert_eq!(parser.find_size_field("b"), None);
        assert_eq!(parser.find_count_field("b"), Some(format_ident!("b_count")));
    }

    #[test]
    fn test_find_fields_dynamic_size() {
        let code = "
              little_endian_packets
              packet P {
                _size_(c): 8,
                c: 24[],
              }
            ";
        let file = parse_str(code);
        let scope = analyzer::Scope::new(&file).unwrap();
        let schema = analyzer::Schema::new(&file);
        let span = format_ident!("bytes");
        let parser = FieldParser::new(&scope, &schema, file.endianness.value, "P", &span);
        assert_eq!(parser.find_size_field("c"), Some(format_ident!("c_size")));
        assert_eq!(parser.find_count_field("c"), None);
    }
}
