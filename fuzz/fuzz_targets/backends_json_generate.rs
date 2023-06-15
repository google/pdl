#![no_main]

use libfuzzer_sys::fuzz_target;
use pdl_compiler::{ast, backends, parser};

// Fuzz pdl_compiler::backends::json::generate.
fuzz_target!(|source: String| {
    let mut sources = ast::SourceDatabase::new();
    let Ok(file) = parser::parse_inline(&mut sources, String::from("input.pdl"), source) else {
        return;
    };
    let _ = backends::json::generate(&file);
});
