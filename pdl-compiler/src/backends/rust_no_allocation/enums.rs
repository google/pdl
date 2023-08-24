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

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use crate::ast;

use super::utils::get_integer_type;

pub fn generate_enum(id: &str, tags: &[ast::Tag], width: usize) -> TokenStream {
    let id_ident = format_ident!("{id}");
    let tag_ids = tags.iter().map(|tag| format_ident!("{}", tag.id())).collect::<Vec<_>>();
    let tag_values = tags
        .iter()
        .map(|tag| Literal::u64_unsuffixed(tag.value().unwrap() as u64))
        .collect::<Vec<_>>();
    let backing_ident = get_integer_type(width);

    quote! {
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        pub enum #id_ident {
            #(#tag_ids),*
        }

        impl #id_ident {
            pub fn new(value: #backing_ident) -> Result<Self, ParseError> {
                match value {
                    #(#tag_values => Ok(Self::#tag_ids)),*,
                    _ => Err(ParseError::InvalidEnumValue),
                }
            }

            pub fn value(&self) -> #backing_ident {
                match self {
                    #(Self::#tag_ids => #tag_values),*,
                }
            }

            fn try_parse(buf: BitSlice) -> Result<Self, ParseError> {
                let value = buf.slice(#width)?.try_parse()?;
                match value {
                    #(#tag_values => Ok(Self::#tag_ids)),*,
                    _ => Err(ParseError::InvalidEnumValue),
                }
            }
        }

        impl Serializable for #id_ident {
            fn serialize(&self, writer: &mut impl BitWriter) -> Result<(), SerializeError> {
                writer.write_bits(#width, || Ok(self.value()));
                Ok(())
            }
        }

        impl From<#id_ident> for #backing_ident {
            fn from(x: #id_ident) -> #backing_ident {
                x.value()
            }
        }

        impl TryFrom<#backing_ident> for #id_ident {
            type Error = ParseError;

            fn try_from(value: #backing_ident) -> Result<Self, ParseError> {
                Self::new(value)
            }
        }
    }
}
