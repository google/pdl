use quote::{format_ident, quote};

use crate::ast;
use crate::backends::rust::types;

/// Like [`ast::Field::Scalar`].
#[derive(Debug, Clone)]
pub struct ScalarField {
    id: String,
    width: usize,
}

impl ScalarField {
    fn new(id: &str, width: usize) -> ScalarField {
        ScalarField { id: String::from(id), width }
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_ident(&self) -> proc_macro2::Ident {
        format_ident!("{}", self.id)
    }

    fn generate_decl(&self, visibility: syn::Visibility) -> proc_macro2::TokenStream {
        let field_name = self.get_ident();
        let field_type = types::Integer::new(self.width);
        quote! {
            #visibility #field_name: #field_type
        }
    }

    fn generate_getter(&self, packet_name: &syn::Ident) -> proc_macro2::TokenStream {
        let field_name = self.get_ident();
        let getter_name = format_ident!("get_{}", self.id);
        let field_type = types::Integer::new(self.width);
        quote! {
            pub fn #getter_name(&self) -> #field_type {
                self.#packet_name.as_ref().#field_name
            }
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
            Field::Scalar(field) => field.get_width(),
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
}
