use quote::format_ident;

use crate::ast;

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
}
