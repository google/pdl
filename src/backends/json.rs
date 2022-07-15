//! Rust compiler backend.

use crate::ast;

/// Turn the AST into a JSON representation.
pub fn generate(file: &ast::File) -> Result<String, String> {
    serde_json::to_string_pretty(&file)
        .map_err(|err| format!("could not JSON serialize grammar: {err}"))
}
