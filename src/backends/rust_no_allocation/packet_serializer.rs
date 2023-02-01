use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    ast,
    backends::{
        intermediate::{ComputedValue, ComputedValueId, PacketOrStruct, Schema},
        rust_no_allocation::utils::get_integer_type,
    },
    parser,
};

fn standardize_child(id: &str) -> &str {
    match id {
        "_body_" | "_payload_" => "_child_",
        _ => id,
    }
}

pub fn generate_packet_serializer(
    id: &str,
    parent_id: Option<&str>,
    fields: &[parser::ast::Field],
    schema: &Schema,
    curr_schema: &PacketOrStruct,
    children: &HashMap<&str, Vec<&str>>,
) -> TokenStream {
    let id_ident = format_ident!("{id}Builder");

    let builder_fields = fields
        .iter()
        .filter_map(|field| {
            match &field.desc {
                ast::FieldDesc::Padding { .. }
                | ast::FieldDesc::Reserved { .. }
                | ast::FieldDesc::FixedScalar { .. }
                | ast::FieldDesc::FixedEnum { .. }
                | ast::FieldDesc::ElementSize { .. }
                | ast::FieldDesc::Count { .. }
                | ast::FieldDesc::Size { .. } => {
                    // no-op, no getter generated for this type
                    None
                }
                ast::FieldDesc::Group { .. } => unreachable!(),
                ast::FieldDesc::Checksum { .. } => {
                    unimplemented!("checksums not yet supported with this backend")
                }
                ast::FieldDesc::Body | ast::FieldDesc::Payload { .. } => {
                    let type_ident = format_ident!("{id}Child");
                    Some(("_child_", quote! { #type_ident }))
                }
                ast::FieldDesc::Array { id, width, type_id, .. } => {
                    let element_type = if let Some(width) = width {
                        get_integer_type(*width)
                    } else if let Some(type_id) = type_id {
                        if schema.enums.contains_key(type_id.as_str()) {
                            format_ident!("{type_id}")
                        } else {
                            format_ident!("{type_id}Builder")
                        }
                    } else {
                        unreachable!();
                    };
                    Some((id.as_str(), quote! { Box<[#element_type]> }))
                }
                ast::FieldDesc::Scalar { id, width } => {
                    let id_type = get_integer_type(*width);
                    Some((id.as_str(), quote! { #id_type }))
                }
                ast::FieldDesc::Typedef { id, type_id } => {
                    let type_ident = if schema.enums.contains_key(type_id.as_str()) {
                        format_ident!("{type_id}")
                    } else {
                        format_ident!("{type_id}Builder")
                    };
                    Some((id.as_str(), quote! { #type_ident }))
                }
            }
        })
        .map(|(id, typ)| {
            let id_ident = format_ident!("{id}");
            quote! { pub #id_ident: #typ }
        });

    let mut has_child = false;

    let serializer = fields.iter().map(|field| {
        match &field.desc {
            ast::FieldDesc::Checksum { .. } | ast::FieldDesc::Group { .. } => unimplemented!(),
            ast::FieldDesc::Padding { size, .. } => {
                quote! {
                    if (most_recent_array_size_in_bits > #size * 8) {
                        return Err(SerializeError::NegativePadding);
                    }
                    writer.write_bits((#size * 8 - most_recent_array_size_in_bits) as usize, || Ok(0u64))?;
                }
            },
            ast::FieldDesc::Size { field_id, width } => {
                let field_id = standardize_child(field_id);
                let field_ident = format_ident!("{field_id}");

                // if the element-size is fixed, we can directly multiply
                if let Some(ComputedValue::Constant(element_width)) = curr_schema.computed_values.get(&ComputedValueId::FieldElementSize(field_id)) {
                    return quote! {
                        writer.write_bits(
                            #width,
                            || u64::try_from(self.#field_ident.len() * #element_width).or(Err(SerializeError::IntegerConversionFailure))
                        )?;
                    }
                }

                // if the field is "countable", loop over it to sum up the size
                if curr_schema.computed_values.contains_key(&ComputedValueId::FieldCount(field_id)) {
                    return quote! {
                        writer.write_bits(#width, || {
                            let size_in_bits = self.#field_ident.iter().map(|elem| elem.size_in_bits()).fold(Ok(0), |total, next| {
                                let total: u64 = total?;
                                let next = u64::try_from(next?).or(Err(SerializeError::IntegerConversionFailure))?;
                                total.checked_add(next).ok_or(SerializeError::IntegerConversionFailure)
                            })?;
                            if size_in_bits % 8 != 0 {
                                return Err(SerializeError::AlignmentError);
                            }
                            Ok(size_in_bits / 8)
                        })?;
                    }
                }

                // otherwise, try to get the size directly
                quote! {
                    writer.write_bits(#width, || {
                        let size_in_bits: u64 = self.#field_ident.size_in_bits()?.try_into().or(Err(SerializeError::IntegerConversionFailure))?;
                        if size_in_bits % 8 != 0 {
                            return Err(SerializeError::AlignmentError);
                        }
                        Ok(size_in_bits / 8)
                    })?;
                }
            }
            ast::FieldDesc::Count { field_id, width } => {
                let field_ident = format_ident!("{field_id}");
                quote! { writer.write_bits(#width, || u64::try_from(self.#field_ident.len()).or(Err(SerializeError::IntegerConversionFailure)))?; }
            }
            ast::FieldDesc::ElementSize { field_id, width } => {
                // TODO(aryarahul) - add validation for elementsize against all the other elements
                let field_ident = format_ident!("{field_id}");
                quote! {
                    let get_element_size = || Ok(if let Some(field) = self.#field_ident.get(0) {
                        let size_in_bits = field.size_in_bits()?;
                        if size_in_bits % 8 == 0 {
                            (size_in_bits / 8) as u64
                        } else {
                            return Err(SerializeError::AlignmentError);
                        }
                    } else {
                        0
                    });
                    writer.write_bits(#width, || get_element_size() )?;
                }
            }
            ast::FieldDesc::Reserved { width, .. } => {
                quote!{ writer.write_bits(#width, || Ok(0u64))?; }
            }
            ast::FieldDesc::Scalar { width, id } => {
                let field_ident = format_ident!("{id}");
                quote! { writer.write_bits(#width, || Ok(self.#field_ident))?; }
            }
            ast::FieldDesc::FixedScalar { width, value } => {
                let width = quote! { #width };
                let value = {
                    let value = *value as u64;
                    quote! { #value }
                };
                quote!{ writer.write_bits(#width, || Ok(#value))?; }
            }
            ast::FieldDesc::FixedEnum { enum_id, tag_id } => {
                let width = {
                    let width = schema.enums[enum_id.as_str()].width;
                    quote! { #width }
                };
                let value = {
                    let enum_ident = format_ident!("{}", enum_id);
                    let tag_ident = format_ident!("{tag_id}");
                    quote! { #enum_ident::#tag_ident.value() }
                };
                quote!{ writer.write_bits(#width, || Ok(#value))?; }
            }
            ast::FieldDesc::Body | ast::FieldDesc::Payload { .. } => {
                has_child = true;
                quote! { self._child_.serialize(writer)?; }
            }
            ast::FieldDesc::Array { width, id, .. } => {
                let id_ident = format_ident!("{id}");
                if let Some(width) = width {
                    quote! {
                        for elem in self.#id_ident.iter() {
                            writer.write_bits(#width, || Ok(*elem))?;
                        }
                        let most_recent_array_size_in_bits = #width * self.#id_ident.len();
                    }
                } else {
                    quote! {
                        let mut most_recent_array_size_in_bits = 0;
                        for elem in self.#id_ident.iter() {
                            most_recent_array_size_in_bits += elem.size_in_bits()?;
                            elem.serialize(writer)?;
                        }
                     }
                }
            }
            ast::FieldDesc::Typedef { id, .. } => {
                let id_ident = format_ident!("{id}");
                quote! { self.#id_ident.serialize(writer)?; }
            }
        }
    }).collect::<Vec<_>>();

    let variant_names = children.get(id).into_iter().flatten().collect::<Vec<_>>();

    let variants = variant_names.iter().map(|name| {
        let name_ident = format_ident!("{name}");
        let variant_ident = format_ident!("{name}Builder");
        quote! { #name_ident(#variant_ident) }
    });

    let variant_serializers = variant_names.iter().map(|name| {
        let name_ident = format_ident!("{name}");
        quote! {
            Self::#name_ident(x) => {
                x.serialize(writer)?;
            }
        }
    });

    let children_enum = if has_child {
        let enum_ident = format_ident!("{id}Child");
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum #enum_ident {
                RawData(Box<[u8]>),
                #(#variants),*
            }

            impl Serializable for #enum_ident {
                fn serialize(&self, writer: &mut impl BitWriter) -> Result<(), SerializeError> {
                    match self {
                        Self::RawData(data) => {
                            for byte in data.iter() {
                                writer.write_bits(8, || Ok(*byte as u64))?;
                            }
                        },
                        #(#variant_serializers),*
                    }
                    Ok(())
                }
            }
        }
    } else {
        quote! {}
    };

    let parent_type_converter = if let Some(parent_id) = parent_id {
        let parent_enum_ident = format_ident!("{parent_id}Child");
        let variant_ident = format_ident!("{id}");
        Some(quote! {
            impl From<#id_ident> for #parent_enum_ident {
                fn from(x: #id_ident) -> Self {
                    Self::#variant_ident(x)
                }
            }
        })
    } else {
        None
    };

    let owned_packet_ident = format_ident!("Owned{id}View");

    quote! {
      #[derive(Debug, Clone, PartialEq, Eq)]
      pub struct #id_ident {
          #(#builder_fields),*
      }

      impl Builder for #id_ident {
        type OwnedPacket = #owned_packet_ident;
      }

      impl Serializable for #id_ident {
          fn serialize(&self, writer: &mut impl BitWriter) -> Result<(), SerializeError> {
            #(#serializer)*
            Ok(())
          }
      }

      #parent_type_converter

      #children_enum
    }
}
