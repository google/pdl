use quote::{format_ident, quote};

use crate::ast;
use crate::backends::rust::mask_bits;
use crate::backends::rust::types;

/// Like [`ast::Field::Scalar`].
#[derive(Debug, Clone)]
pub struct ScalarField {
    pub id: String,
    pub width: usize,
}

impl ScalarField {
    fn new(id: &str, width: usize) -> ScalarField {
        ScalarField { id: String::from(id), width }
    }

    fn get_ident(&self) -> proc_macro2::Ident {
        format_ident!("{}", self.id)
    }

    fn get_type(&self) -> types::Integer {
        types::Integer::new(self.width)
    }

    fn generate_decl(&self, visibility: syn::Visibility) -> proc_macro2::TokenStream {
        let field_name = self.get_ident();
        let field_type = self.get_type();
        quote! {
            #visibility #field_name: #field_type
        }
    }

    fn generate_getter(&self, packet_name: &syn::Ident) -> proc_macro2::TokenStream {
        let field_name = self.get_ident();
        let getter_name = format_ident!("get_{}", self.id);
        let field_type = self.get_type();
        quote! {
            pub fn #getter_name(&self) -> #field_type {
                self.#packet_name.as_ref().#field_name
            }
        }
    }

    fn generate_read_adjustment(
        &self,
        offset: usize,
        chunk_type: types::Integer,
    ) -> proc_macro2::TokenStream {
        let field_name = self.get_ident();
        let field_type = self.get_type();
        let mut field = quote! {
            chunk
        };
        if offset > 0 {
            let offset = syn::Index::from(offset);
            let op = syn::parse_str::<syn::BinOp>(">>").unwrap();
            field = quote! {
                (#field #op #offset)
            };
        }

        if self.width < field_type.width {
            let bit_mask = mask_bits(self.width);
            field = quote! {
                (#field & #bit_mask)
            };
        }

        if field_type.width < chunk_type.width {
            field = quote! {
                #field as #field_type;
            };
        }

        quote! {
            let #field_name = #field;
        }
    }

    fn generate_write_adjustment(
        &self,
        offset: usize,
        chunk_type: types::Integer,
    ) -> proc_macro2::TokenStream {
        let field_name = self.get_ident();
        let field_type = self.get_type();

        let mut field = quote! {
            self.#field_name
        };

        if field_type.width < chunk_type.width {
            field = quote! {
                (#field as #chunk_type)
            };
        }

        if self.width < field_type.width {
            let bit_mask = mask_bits(self.width);
            field = quote! {
                (#field & #bit_mask)
            };
        }

        if offset > 0 {
            let field_offset = syn::Index::from(offset);
            let op = syn::parse_str::<syn::BinOp>("<<").unwrap();
            field = quote! {
                (#field #op #field_offset)
            };
        }

        quote! {
            let chunk = chunk | #field;
        }
    }
}

/// Projection of [`ast::Field`] with the bits needed for the Rust
/// backend.
#[derive(Debug, Clone)]
pub enum Field {
    Scalar(ScalarField),
}

impl From<&ast::Field> for Field {
    fn from(field: &ast::Field) -> Field {
        match field {
            ast::Field::Scalar { id, width, .. } => Field::Scalar(ScalarField::new(id, *width)),
            _ => todo!("Unsupported field: {:?}", field),
        }
    }
}

impl Field {
    pub fn get_width(&self) -> usize {
        match self {
            Field::Scalar(field) => field.width,
        }
    }

    pub fn get_ident(&self) -> proc_macro2::Ident {
        match self {
            Field::Scalar(field) => field.get_ident(),
        }
    }

    pub fn generate_decl(&self, visibility: syn::Visibility) -> proc_macro2::TokenStream {
        match self {
            Field::Scalar(field) => field.generate_decl(visibility),
        }
    }

    pub fn generate_getter(&self, packet_name: &syn::Ident) -> proc_macro2::TokenStream {
        match self {
            Field::Scalar(field) => field.generate_getter(packet_name),
        }
    }

    pub fn generate_read_adjustment(
        &self,
        offset: usize,
        chunk_type: types::Integer,
    ) -> proc_macro2::TokenStream {
        match self {
            Field::Scalar(field) => field.generate_read_adjustment(offset, chunk_type),
        }
    }

    pub fn generate_write_adjustment(
        &self,
        offset: usize,
        chunk_type: types::Integer,
    ) -> proc_macro2::TokenStream {
        match self {
            Field::Scalar(field) => field.generate_write_adjustment(offset, chunk_type),
        }
    }
}
