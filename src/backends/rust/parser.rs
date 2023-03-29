use crate::analyzer::ast as analyzer_ast;
use crate::backends::rust::{
    constraint_to_value, find_constrained_parent_fields, mask_bits, types, ToUpperCamelCase,
};
use crate::{ast, lint};
use quote::{format_ident, quote};
use std::collections::{BTreeSet, HashMap};

fn size_field_ident(id: &str) -> proc_macro2::Ident {
    format_ident!("{}_size", id.trim_matches('_'))
}

/// A single bit-field.
struct BitField<'a> {
    shift: usize, // The shift to apply to this field.
    field: &'a analyzer_ast::Field,
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

    pub fn add(&mut self, field: &'a analyzer_ast::Field) {
        match &field.desc {
            _ if self.scope.is_bitfield(field) => self.add_bit_field(field),
            ast::FieldDesc::Padding { .. } => todo!("Padding fields are not supported"),
            ast::FieldDesc::Array { id, width, type_id, size, .. } => self.add_array_field(
                id,
                *width,
                type_id.as_deref(),
                *size,
                self.scope.get_field_declaration(field),
            ),
            ast::FieldDesc::Typedef { id, type_id } => self.add_typedef_field(id, type_id),
            ast::FieldDesc::Payload { size_modifier, .. } => {
                self.add_payload_field(size_modifier.as_deref())
            }
            ast::FieldDesc::Body { .. } => self.add_payload_field(None),
            _ => todo!("{field:?}"),
        }
    }

    fn add_bit_field(&mut self, field: &'a analyzer_ast::Field) {
        self.chunk.push(BitField { shift: self.shift, field });
        self.shift += self.scope.get_field_width(field, false).unwrap();
        if self.shift % 8 != 0 {
            return;
        }

        let size = self.shift / 8;
        let end_offset = self.offset + size;

        let wanted = proc_macro2::Literal::usize_unsuffixed(size);
        self.check_size(&quote!(#wanted));

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

            let width = self.scope.get_field_width(field, false).unwrap();
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

            self.code.push(match &field.desc {
                ast::FieldDesc::Scalar { id, .. } => {
                    let id = format_ident!("{id}");
                    quote! {
                        let #id = #v;
                    }
                }
                ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                    let enum_id = format_ident!("{enum_id}");
                    let tag_id = format_ident!("{}", tag_id.to_upper_camel_case());
                    quote! {
                        if #v != #enum_id::#tag_id as #value_type {
                            return Err(Error::InvalidFixedValue {
                                expected: #enum_id::#tag_id as u64,
                                actual: #v as u64,
                            });
                        }
                    }
                }
                ast::FieldDesc::FixedScalar { value, .. } => {
                    let value = proc_macro2::Literal::usize_unsuffixed(*value);
                    quote! {
                        if #v != #value {
                            return Err(Error::InvalidFixedValue {
                                expected: #value,
                                actual: #v as u64,
                            });
                        }
                    }
                }
                ast::FieldDesc::Typedef { id, type_id } => {
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
                ast::FieldDesc::Reserved { .. } => {
                    if single_value {
                        let span = self.span;
                        let size = proc_macro2::Literal::usize_unsuffixed(size);
                        quote! {
                            #span.get_mut().advance(#size);
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

    fn packet_scope(&self) -> Option<&lint::PacketScope> {
        self.scope.scopes.get(self.scope.typedef.get(self.packet_name)?)
    }

    fn find_count_field(&self, id: &str) -> Option<proc_macro2::Ident> {
        match self.packet_scope()?.get_array_size_field(id)?.desc {
            ast::FieldDesc::Count { .. } => Some(format_ident!("{id}_count")),
            _ => None,
        }
    }

    fn find_size_field(&self, id: &str) -> Option<proc_macro2::Ident> {
        match self.packet_scope()?.get_array_size_field(id)?.desc {
            ast::FieldDesc::Size { .. } => Some(size_field_ident(id)),
            _ => None,
        }
    }

    fn payload_field_offset_from_end(&self) -> Option<usize> {
        let packet_scope = self.packet_scope().unwrap();
        let mut fields = packet_scope.iter_fields();
        fields.find(|f| {
            matches!(f.desc, ast::FieldDesc::Body { .. } | ast::FieldDesc::Payload { .. })
        })?;

        let mut offset = 0;
        for field in fields {
            if let Some(width) = self.scope.get_field_width(field, false) {
                offset += width;
            } else {
                return None;
            }
        }

        Some(offset)
    }

    fn check_size(&mut self, wanted: &proc_macro2::TokenStream) {
        let packet_name = &self.packet_name;
        let span = self.span;
        self.code.push(quote! {
            if #span.get().remaining() < #wanted {
                return Err(Error::InvalidLengthError {
                    obj: #packet_name.to_string(),
                    wanted: #wanted,
                    got: #span.get().remaining(),
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
        decl: Option<&analyzer_ast::Decl>,
    ) {
        enum ElementWidth {
            Static(usize), // Static size in bytes.
            Unknown,
        }
        let element_width = match width.or_else(|| self.scope.get_decl_width(decl?, false)) {
            Some(w) => {
                assert_eq!(w % 8, 0, "Array element size ({w}) is not a multiple of 8");
                ElementWidth::Static(w / 8)
            }
            None => ElementWidth::Unknown,
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

        // TODO padded_size

        let id = format_ident!("{id}");
        let span = self.span;

        let parse_element = self.parse_array_element(self.span, width, type_id, decl);
        match (element_width, &array_shape) {
            (ElementWidth::Unknown, ArrayShape::SizeField(size_field)) => {
                // The element width is not known, but the array full
                // octet size is known by size field. Parse elements
                // item by item as a vector.
                self.check_size(&quote!(#size_field));
                let parse_element =
                    self.parse_array_element(&format_ident!("head"), width, type_id, decl);
                self.code.push(quote! {
                    let (head, tail) = #span.get().split_at(#size_field);
                    let mut head = &mut Cell::new(head);
                    #span.replace(tail);
                    let mut #id = Vec::new();
                    while !head.get().is_empty() {
                        #id.push(#parse_element?);
                    }
                });
            }
            (ElementWidth::Unknown, ArrayShape::Static(count)) => {
                // The element width is not known, but the array
                // element count is known statically. Parse elements
                // item by item as an array.
                let count = syn::Index::from(*count);
                self.code.push(quote! {
                    // TODO(mgeisler): use
                    // https://doc.rust-lang.org/std/array/fn.try_from_fn.html
                    // when stabilized.
                    let #id = [0; #count].map(|_| #parse_element.unwrap());
                });
            }
            (ElementWidth::Unknown, ArrayShape::CountField(count_field)) => {
                // The element width is not known, but the array
                // element count is known by the count field. Parse
                // elements item by item as a vector.
                self.code.push(quote! {
                    let #id = (0..#count_field)
                        .map(|_| #parse_element)
                        .collect::<Result<Vec<_>>>()?;
                });
            }
            (ElementWidth::Unknown, ArrayShape::Unknown) => {
                // Neither the count not size is known, parse elements
                // until the end of the span.
                self.code.push(quote! {
                    let mut #id = Vec::new();
                    while !#span.get().is_empty() {
                        #id.push(#parse_element?);
                    }
                });
            }
            (ElementWidth::Static(element_width), ArrayShape::Static(count)) => {
                // The element width is known, and the array element
                // count is known statically.
                let count = syn::Index::from(*count);
                // This creates a nicely formatted size.
                let array_size = if element_width == 1 {
                    quote!(#count)
                } else {
                    let element_width = syn::Index::from(element_width);
                    quote!(#count * #element_width)
                };
                self.check_size(&array_size);
                self.code.push(quote! {
                    // TODO(mgeisler): use
                    // https://doc.rust-lang.org/std/array/fn.try_from_fn.html
                    // when stabilized.
                    let #id = [0; #count].map(|_| #parse_element.unwrap());
                });
            }
            (ElementWidth::Static(_), ArrayShape::CountField(count_field)) => {
                // The element width is known, and the array element
                // count is known dynamically by the count field.
                self.check_size(&quote!(#count_field));
                self.code.push(quote! {
                    let #id = (0..#count_field)
                        .map(|_| #parse_element)
                        .collect::<Result<Vec<_>>>()?;
                });
            }
            (ElementWidth::Static(element_width), ArrayShape::SizeField(_))
            | (ElementWidth::Static(element_width), ArrayShape::Unknown) => {
                // The element width is known, and the array full size
                // is known by size field, or unknown (in which case
                // it is the remaining span length).
                let array_size = if let ArrayShape::SizeField(size_field) = &array_shape {
                    self.check_size(&quote!(#size_field));
                    quote!(#size_field)
                } else {
                    quote!(#span.get().remaining())
                };
                let count_field = format_ident!("{id}_count");
                let array_count = if element_width != 1 {
                    let element_width = syn::Index::from(element_width);
                    self.code.push(quote! {
                        if #array_size % #element_width != 0 {
                            return Err(Error::InvalidArraySize {
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

                self.code.push(quote! {
                    let mut #id = Vec::with_capacity(#array_count);
                    for _ in 0..#array_count {
                        #id.push(#parse_element?);
                    }
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
        if let ast::DeclDesc::Struct { parent_id: Some(_), .. } = &decl.desc {
            panic!("Derived struct used in typedef field");
        }

        let span = self.span;
        let id = format_ident!("{id}");
        let type_id = format_ident!("{type_id}");

        match self.scope.get_decl_width(decl, true) {
            None => self.code.push(quote! {
                let #id = #type_id::parse_inner(&mut #span)?;
            }),
            Some(width) => {
                assert_eq!(width % 8, 0, "Typedef field type size is not a multiple of 8");
                let width = syn::Index::from(width / 8);
                self.code.push(if let ast::DeclDesc::Checksum { .. } = &decl.desc {
                    // TODO: handle checksum fields.
                    quote! {
                        #span.get_mut().advance(#width);
                    }
                } else {
                    quote! {
                        let (head, tail) = #span.get().split_at(#width);
                        #span.replace(tail);
                        let #id = #type_id::parse(head)?;
                    }
                });
            }
        }
    }

    /// Parse body and payload fields.
    fn add_payload_field(&mut self, size_modifier: Option<&str>) {
        let span = self.span;
        let packet_scope = self.packet_scope().unwrap();
        let payload_size_field = packet_scope.get_payload_size_field();
        let offset_from_end = self.payload_field_offset_from_end();

        if size_modifier.is_some() {
            todo!(
                "Unsupported size modifier for {packet}: {size_modifier:?}",
                packet = self.packet_name
            );
        }

        if self.shift != 0 {
            if payload_size_field.is_some() {
                panic!("Unexpected payload size for non byte aligned payload");
            }

            //let rounded_size = self.shift / 8 + if self.shift % 8 == 0 { 0 } else { 1 };
            //let padding_bits = 8 * rounded_size - self.shift;
            //let reserved_field =
            //    ast::Field::Reserved { loc: ast::SourceRange::default(), width: padding_bits };
            //TODO: self.add_bit_field(&reserved_field); --
            // reserved_field does not live long enough.

            // TODO: consume span of rounded size
        } else {
            // TODO: consume span
        }

        if let Some(ast::FieldDesc::Size { field_id, .. }) = &payload_size_field.map(|f| &f.desc) {
            // The payload or body has a known size. Consume the
            // payload and update the span in case fields are placed
            // after the payload.
            let size_field = size_field_ident(field_id);
            self.check_size(&quote!(#size_field ));
            self.code.push(quote! {
                let payload = &#span.get()[..#size_field];
                #span.get_mut().advance(#size_field);
            });
        } else if offset_from_end == Some(0) {
            // The payload or body is the last field of a packet,
            // consume the remaining span.
            self.code.push(quote! {
                let payload = #span.get();
                #span.get_mut().advance(payload.len());
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
            let offset_from_end = syn::Index::from(offset_from_end / 8);
            self.check_size(&quote!(#offset_from_end));
            self.code.push(quote! {
                let payload = &#span.get()[..#span.get().len() - #offset_from_end];
                #span.get_mut().advance(payload.len());
            });
        }

        let decl = self.scope.typedef[self.packet_name];
        if let ast::DeclDesc::Struct { .. } = &decl.desc {
            self.code.push(quote! {
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
        decl: Option<&analyzer_ast::Decl>,
    ) -> proc_macro2::TokenStream {
        if let Some(width) = width {
            let get_uint = types::get_uint(self.endianness, width, span);
            return quote! {
                Ok::<_, Error>(#get_uint)
            };
        }

        if let Some(ast::DeclDesc::Enum { id, width, .. }) = decl.map(|decl| &decl.desc) {
            let element_type = types::Integer::new(*width);
            let get_uint = types::get_uint(self.endianness, *width, span);
            let type_id = format_ident!("{id}");
            let from_u = format_ident!("from_u{}", element_type.width);
            let packet_name = &self.packet_name;
            return quote! {
                #type_id::#from_u(#get_uint).ok_or_else(|| Error::InvalidEnumValueError {
                    obj: #packet_name.to_string(),
                    field: String::new(), // TODO(mgeisler): fill out or remove
                    value: 0,
                    type_: #id.to_string(),
                })
            };
        }

        let type_id = format_ident!("{}", type_id.unwrap());
        quote! {
            #type_id::parse_inner(#span)
        }
    }

    pub fn done(&mut self) {
        let decl = self.scope.typedef[self.packet_name];
        if let ast::DeclDesc::Struct { .. } = &decl.desc {
            return; // Structs don't parse the child structs recursively.
        }

        let packet_scope = &self.scope.scopes[&decl];
        let children = self.scope.iter_children(self.packet_name).collect::<Vec<_>>();
        if children.is_empty() && packet_scope.get_payload_field().is_none() {
            return;
        }

        let child_ids = children
            .iter()
            .map(|child| format_ident!("{}", child.id().unwrap()))
            .collect::<Vec<_>>();
        let child_ids_data = child_ids.iter().map(|ident| format_ident!("{ident}Data"));

        // Set of field names (sorted by name).
        let mut constrained_fields = BTreeSet::new();
        // Maps (child name, field name) -> value.
        let mut constraint_values = HashMap::new();

        for child in children.iter() {
            match &child.desc {
                ast::DeclDesc::Packet { id, constraints, .. }
                | ast::DeclDesc::Struct { id, constraints, .. } => {
                    for constraint in constraints.iter() {
                        constrained_fields.insert(&constraint.id);
                        constraint_values.insert(
                            (id.as_str(), &constraint.id),
                            constraint_to_value(packet_scope, constraint),
                        );
                    }
                }
                _ => unreachable!("Invalid child: {child:?}"),
            }
        }

        let wildcard = quote!(_);
        let match_values = children.iter().map(|child| {
            let child_id = child.id().unwrap();
            let values = constrained_fields.iter().map(|field_name| {
                constraint_values.get(&(child_id, field_name)).unwrap_or(&wildcard)
            });
            quote! {
                (#(#values),*)
            }
        });
        let constrained_field_idents =
            constrained_fields.iter().map(|field| format_ident!("{field}"));
        let child_parse_args = children.iter().map(|child| {
            let fields = find_constrained_parent_fields(self.scope, child.id().unwrap())
                .map(|field| format_ident!("{}", field.id().unwrap()));
            quote!(#(, #fields)*)
        });
        let packet_data_child = format_ident!("{}DataChild", self.packet_name);
        self.code.push(quote! {
            let child = match (#(#constrained_field_idents),*) {
                #(#match_values if #child_ids_data::conforms(&payload) => {
                    let mut cell = Cell::new(payload);
                    let child_data = #child_ids_data::parse_inner(&mut cell #child_parse_args)?;
                    // TODO(mgeisler): communicate back to user if !cell.get().is_empty()?
                    #packet_data_child::#child_ids(Arc::new(child_data))
                }),*
                _ if !payload.is_empty() => {
                    #packet_data_child::Payload(Bytes::copy_from_slice(payload))
                }
                _ => #packet_data_child::None,
            };
        });
    }
}

impl quote::ToTokens for FieldParser<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let code = &self.code;
        tokens.extend(quote! {
            #(#code)*
        });
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
    pub fn parse_str(text: &str) -> analyzer_ast::File {
        let mut db = ast::SourceDatabase::new();
        let file =
            parse_inline(&mut db, String::from("stdin"), String::from(text)).expect("parse error");
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
        let scope = lint::Scope::new(&file);
        let span = format_ident!("bytes");
        let parser = FieldParser::new(&scope, file.endianness.value, "P", &span);
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
        let scope = lint::Scope::new(&file);
        let span = format_ident!("bytes");
        let parser = FieldParser::new(&scope, file.endianness.value, "P", &span);
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
        let scope = lint::Scope::new(&file);
        let span = format_ident!("bytes");
        let parser = FieldParser::new(&scope, file.endianness.value, "P", &span);
        assert_eq!(parser.find_size_field("c"), Some(format_ident!("c_size")));
        assert_eq!(parser.find_count_field("c"), None);
    }
}
