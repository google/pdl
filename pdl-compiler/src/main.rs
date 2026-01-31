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

//! PDL parser and analyzer.

use argh::FromArgs;
use codespan_reporting::term::{self, termcolor};

use pdl_compiler::{analyzer, ast, backends, parser};

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OutputFormat {
    Java,
    JSON,
    Rust,
    RustLegacy,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "json" => Ok(Self::JSON),
            "rust" => Ok(Self::Rust),
            "java" => Ok(Self::Java),
            "rust_legacy" => Ok(Self::RustLegacy),
            _ => Err(format!(
                "could not parse {input:?}, valid option are 'json', 'rust', 'rust_legacy'."
            )),
        }
    }
}

#[derive(FromArgs, Debug)]
/// PDL analyzer and generator.
struct Opt {
    #[argh(switch)]
    /// print tool version and exit.
    version: bool,

    #[argh(option, default = "OutputFormat::JSON")]
    /// generate output in this format ("json", "rust", "java", "rust_legacy",).
    /// The output will be printed on stdout in all cases.
    /// The input file is the source PDL file.
    output_format: OutputFormat,

    #[argh(option)]
    /// generate tests for the selected output format from the provided file.
    /// This file must point to a JSON formatted file with a list of test vectors.
    /// When this option is provided, the input file must point to the source PDL file
    /// from which the vectors were generated.
    /// Valid for the output formats "rust", "java", "rust_legacy".
    test_file: Option<String>,

    #[argh(positional)]
    /// input files.
    input_file: Option<String>,

    #[argh(option)]
    /// exclude declarations from the generated output.
    exclude_declaration: Vec<String>,

    #[argh(option)]
    /// custom_field import paths.
    /// For the rust backend this is a path e.g. "module::CustomField" or "super::CustomField".
    custom_field: Vec<String>,

    #[cfg(feature = "java")]
    #[argh(option)]
    /// directory where generated files should go. This only works when 'output_format' is 'java'.
    /// If omitted, the generated code will be printed to stdout.
    output_dir: Option<String>,

    #[cfg(feature = "java")]
    #[argh(option)]
    /// java package to contain the generated classes.
    java_package: Option<String>,
}

/// Remove declarations listed in the input filter.
fn filter_declarations(file: ast::File, exclude_declarations: &[String]) -> ast::File {
    ast::File {
        declarations: file
            .declarations
            .into_iter()
            .filter(|decl| {
                decl.id().map(|id| !exclude_declarations.contains(&id.to_owned())).unwrap_or(true)
            })
            .collect(),
        ..file
    }
}

fn generate_backend(opt: &Opt, input_file: &str) -> Result<(), String> {
    let mut sources = ast::SourceDatabase::new();
    match parser::parse_file(&mut sources, input_file) {
        Ok(file) => {
            let file = filter_declarations(file, &opt.exclude_declaration);
            let analyzed_file = match analyzer::analyze(&file) {
                Ok(file) => file,
                Err(diagnostics) => {
                    diagnostics
                        .emit(
                            &sources,
                            &mut termcolor::StandardStream::stderr(termcolor::ColorChoice::Always)
                                .lock(),
                        )
                        .expect("Could not print analyzer diagnostics");
                    return Err(String::from("Analysis failed"));
                }
            };

            match opt.output_format {
                OutputFormat::JSON => {
                    println!("{}", backends::json::generate(&file).unwrap());
                    Ok(())
                }
                OutputFormat::Rust => {
                    println!(
                        "{}",
                        backends::rust::generate(&sources, &analyzed_file, &opt.custom_field)
                    );
                    Ok(())
                }
                #[cfg(feature = "java")]
                OutputFormat::Java => {
                    let output_dir = opt.output_dir.as_ref().ok_or(String::from(
                        "'--output-dir' is required for '--output-format java'",
                    ))?;
                    let package = opt
                        .java_package
                        .as_ref()
                        .ok_or("'--java-package' is required for '--output-format java'")?;

                    backends::java::generate(
                        &sources,
                        &analyzed_file,
                        &opt.custom_field,
                        std::path::Path::new(output_dir),
                        package,
                    )
                }
                #[cfg(not(feature = "java"))]
                OutputFormat::Java => {
                    Err(String::from("For Java support, please recompile with the 'java' feature"))
                }
                OutputFormat::RustLegacy => {
                    println!("{}", backends::rust_legacy::generate(&sources, &analyzed_file));
                    Ok(())
                }
            }
        }

        Err(err) => {
            let writer = termcolor::StandardStream::stderr(termcolor::ColorChoice::Always);
            let config = term::Config::default();
            term::emit_to_write_style(&mut writer.lock(), &config, &sources, &err)
                .expect("Could not print error");
            Err(String::from("Error while parsing input"))
        }
    }
}

fn generate_tests(opt: &Opt, test_file: &str, _input_file: &str) -> Result<(), String> {
    match opt.output_format {
        OutputFormat::Rust => {
            println!("{}", backends::rust::test::generate_tests(test_file)?);
            Ok(())
        }
        OutputFormat::RustLegacy => {
            println!("{}", backends::rust_legacy::test::generate_tests(test_file)?);
            Ok(())
        }
        #[cfg(feature = "java")]
        OutputFormat::Java => {
            let output_dir = opt
                .output_dir
                .as_ref()
                .ok_or(String::from("'--output-dir' is required for '--output-format java'"))?;
            let package = opt
                .java_package
                .as_ref()
                .ok_or("'--java-package' is required for '--output-format java'")?;

            backends::java::test::generate_tests(
                test_file,
                std::path::Path::new(output_dir),
                package.clone(),
                _input_file,
                &opt.exclude_declaration,
            )
        }
        _ => Err(format!(
            "Canonical tests cannot be generated for the format {:?}",
            opt.output_format
        )),
    }
}

fn main() -> Result<(), String> {
    let opt: Opt = argh::from_env();

    if opt.version {
        println!("pdlc {}\nCopyright (C) 2026 Google LLC", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let Some(input_file) = opt.input_file.as_ref() else {
        return Err("No input file is specified".to_owned());
    };

    if let Some(test_file) = opt.test_file.as_ref() {
        generate_tests(&opt, test_file, input_file)?
    } else {
        generate_backend(&opt, input_file)?
    }

    Ok(())
}
