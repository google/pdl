#![no_main]

use libfuzzer_sys::fuzz_target;
use pdl_compiler::{analyzer, ast, backends, parser};

// Fuzz pdl_compiler::backends::rust::generate.
fuzz_target!(|source: String| {
    let mut sources = ast::SourceDatabase::new();
    let Ok(file) = parser::parse_inline(&mut sources, String::from("input.pdl"), source) else {
        return;
    };
    let Ok(analyzed_file) = analyzer::analyze(&file) else {
        return
    };
    let _ = backends::rust::generate(&sources, &analyzed_file);
});
