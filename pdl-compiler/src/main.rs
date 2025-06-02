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

use std::path::Path;

use argh::FromArgs;
use codespan_reporting::term::{self, termcolor};

use pdl_compiler::{analyzer, ast, backends, parser};

#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OutputFormat {
    JSON,
    Rust,
    Java,
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
                "could not parse {:?}, valid option are 'json', 'rust', 'rust_legacy'.",
                input
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

    #[argh(switch)]
    /// generate tests for the selected output format.
    /// Valid for the output formats "rust", "java", "rust_legacy".
    /// The input file must point to a JSON formatterd file with the list of
    /// test vectors.
    tests: bool,

    #[argh(positional)]
    /// input file.
    input_file: String,

    #[argh(option)]
    /// exclude declarations from the generated output.
    exclude_declaration: Vec<String>,

    #[argh(option)]
    /// custom_field import paths.
    /// For the rust backend this is a path e.g. "module::CustomField" or "super::CustomField".
    custom_field: Vec<String>,

    #[argh(option)]
    /// directory where generated files should go.
    /// If omitted, the generated code will be printed to stdout when supported
    /// by the provided output format.
    output_dir: Option<String>,

    #[argh(option)]
    /// java package to contain the generated classes.
    java_package: Option<String>,

    #[argh(option)]
    /// file containing the PDL declarations under test. This must be provded when the 'test' flag
    /// is set and 'output_format' is 'java' to retreive type information for the packets under
    /// test.
    pdl_file_under_test: Option<String>,
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

fn generate_backend(opt: &Opt) -> Result<(), String> {
    let mut sources = ast::SourceDatabase::new();
    match parser::parse_file(&mut sources, &opt.input_file) {
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
                        Path::new(output_dir),
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
            term::emit(&mut writer.lock(), &config, &sources, &err).expect("Could not print error");
            Err(String::from("Error while parsing input"))
        }
    }
}

fn generate_tests(opt: &Opt) -> Result<(), String> {
    match opt.output_format {
        OutputFormat::Rust => {
            println!("{}", backends::rust::test::generate_tests(&opt.input_file)?);
            Ok(())
        }
        OutputFormat::RustLegacy => {
            println!("{}", backends::rust_legacy::test::generate_tests(&opt.input_file)?);
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
            let pdl_file_under_test = opt
                .pdl_file_under_test
                .as_ref()
                .ok_or("'--pdl-file-under-test' is requred for when generating tests with '--output-format java'")?;

            backends::java::test::generate_tests(
                &opt.input_file,
                Path::new(output_dir),
                package.clone(),
                pdl_file_under_test,
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
        println!("Packet Description Language parser version 1.0");
        return Ok(());
    }

    if opt.tests {
        generate_tests(&opt)
    } else {
        generate_backend(&opt)
    }
}
