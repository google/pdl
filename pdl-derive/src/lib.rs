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

use codespan_reporting::term;
use proc_macro2::TokenStream;
use quote::quote;
use std::env;
use std::path::Path;
use syn::parse_macro_input;

fn pdl_proc_macro(path: syn::LitStr, input: syn::ItemMod) -> TokenStream {
    // Locate the source grammar file.
    let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let Some(relative_path) =
        [Path::new(&root).join(path.value()), Path::new(&root).join("src").join(path.value())]
            .into_iter()
            .find(|path| path.exists())
    else {
        return syn::Error::new(path.span(), "error: unable to find file").to_compile_error();
    };

    // Load and parse the grammar.
    let mut sources = pdl_compiler::ast::SourceDatabase::new();
    let relative_path = relative_path.into_os_string().into_string().unwrap();
    let file = match pdl_compiler::parser::parse_file(&mut sources, &relative_path) {
        Ok(file) => file,
        Err(err) => {
            let mut buffer = termcolor::Buffer::no_color();
            term::emit(&mut buffer, &term::Config::default(), &sources, &err)
                .expect("could not emit parser diagnostics");
            return syn::Error::new(path.span(), String::from_utf8(buffer.into_inner()).unwrap())
                .to_compile_error();
        }
    };

    // Run the analyzer.
    let analyzed_file = match pdl_compiler::analyzer::analyze(&file) {
        Ok(file) => file,
        Err(diagnostics) => {
            let mut buffer = termcolor::Buffer::no_color();
            diagnostics.emit(&sources, &mut buffer).expect("could not emit analyzer diagnostics");
            return syn::Error::new(path.span(), String::from_utf8(buffer.into_inner()).unwrap())
                .to_compile_error();
        }
    };

    // Generate the pdl backend implementation.
    let parser = pdl_compiler::backends::rust::generate_tokens(&sources, &analyzed_file);
    let mod_ident = input.ident;
    let mod_attrs = input.attrs;
    let mod_vis = input.vis;
    let mod_items = input.content.map(|(_, items)| items).unwrap_or(vec![]);

    quote! {
        #(#mod_attrs)*
        #mod_vis mod #mod_ident {
            // Generate an include_bytes! statement to force a dependency
            // on the source pdl file.
            // This workaround is also used by pest, see
            // pest_generator::generator::generate_include, and for context
            // https://internals.rust-lang.org/t/pre-rfc-add-a-builtin-macro-to-indicate-build-dependency-to-file/9242.
            const _: &[u8] = include_bytes!(#relative_path);
            #parser
            #(#mod_items)*
        }
    }
}

/// The main method that's called by the proc macro
/// (a wrapper around `pest_generator::derive_parser`)
#[proc_macro_attribute]
pub fn pdl(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as syn::LitStr);
    let input = parse_macro_input!(input as syn::ItemMod);
    pdl_proc_macro(attr, input).into()
}

#[cfg(test)]
mod test {
    use super::pdl_proc_macro;
    use proc_macro2::TokenStream;
    use quote::quote;

    fn is_compile_error(input: TokenStream, message_prefix: Option<&str>) -> bool {
        match syn::parse2::<syn::Macro>(input) {
            Ok(syn::Macro {
                path: syn::Path { segments, leading_colon: Some(_) }, tokens, ..
            }) if segments.len() == 2 => {
                // Check macro path
                let segments = segments.iter().collect::<Vec<_>>();
                if segments[0].ident != "core" || segments[1].ident != "compile_error" {
                    return false;
                }

                // Check compile_error message
                match (syn::parse2::<syn::LitStr>(tokens), message_prefix) {
                    (Ok(message), Some(message_prefix)) => {
                        message.value().starts_with(message_prefix)
                    }
                    (Ok(_), None) => true,
                    (Err(_), _) => false,
                }
            }
            Ok(_) | Err(_) => false,
        }
    }

    fn make_attr(input: TokenStream) -> syn::LitStr {
        syn::parse2::<syn::LitStr>(input.into()).unwrap()
    }

    fn make_input(input: TokenStream) -> syn::ItemMod {
        syn::parse2::<syn::ItemMod>(input.into()).unwrap()
    }

    #[test]
    fn test_derive_valid() {
        assert!(!is_compile_error(
            pdl_proc_macro(
                make_attr(quote! { "src/test_valid.pdl" }),
                make_input(quote! { mod Test {} }),
            ),
            None
        ));
    }

    #[test]
    fn test_derive_file_not_found() {
        assert!(is_compile_error(
            pdl_proc_macro(
                make_attr(quote! { "src/test_not_found.pdl" }),
                make_input(quote! { mod Test {} }),
            ),
            Some("error: unable to find file")
        ));
    }

    #[test]
    fn test_derive_parser_error() {
        assert!(is_compile_error(
            pdl_proc_macro(
                make_attr(quote! { "src/test_parser_error.pdl" }),
                make_input(quote! { mod Test {} }),
            ),
            Some("error: failed to parse input file")
        ));
    }

    #[test]
    fn test_derive_analyzer_error() {
        assert!(is_compile_error(
            pdl_proc_macro(
                make_attr(quote! { "src/test_analyzer_error.pdl" }),
                make_input(quote! { mod Test {} }),
            ),
            Some("error[E")
        ));
    }
}
