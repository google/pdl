//! Rust compiler backend.

use crate::parser;

/// Turn the AST into a JSON representation.
pub fn generate(file: &parser::ast::File) -> Result<String, String> {
    serde_json::to_string_pretty(&file)
        .map_err(|err| format!("could not JSON serialize grammar: {err}"))
}
