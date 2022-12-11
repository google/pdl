use crate::backends::rust::types;
use crate::parser::ast as parser_ast;
use quote::{format_ident, quote};

pub struct FieldDeclarations {
    code: Vec<proc_macro2::TokenStream>,
}

impl FieldDeclarations {
    pub fn new() -> FieldDeclarations {
        FieldDeclarations { code: Vec::new() }
    }

    pub fn add(&mut self, field: &parser_ast::Field) {
        let id = match field.id() {
            Some(id) => format_ident!("{id}"),
            None => return, // No id => field not stored.
        };

        let field_type = types::rust_type(field);
        self.code.push(quote! {
            #id: #field_type,
        })
    }
}

impl quote::ToTokens for FieldDeclarations {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let code = &self.code;
        tokens.extend(quote! {
            #(#code)*
        });
    }
}
