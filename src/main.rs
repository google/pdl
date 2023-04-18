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

mod analyzer;
mod ast;
mod backends;
mod lint;
mod parser;
#[cfg(test)]
mod test_utils;
mod utils;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OutputFormat {
    JSON,
    Rust,
    RustNoAlloc,
    RustNoAllocTest,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "json" => Ok(Self::JSON),
            "rust" => Ok(Self::Rust),
            "rust_no_alloc" => Ok(Self::RustNoAlloc),
            "rust_no_alloc_test" => Ok(Self::RustNoAllocTest),
            _ => Err(format!("could not parse {:?}, valid option are 'json', 'rust', 'rust_no_alloc', and 'rust_no_alloc_test'.", input)),
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
    /// generate output in this format ("json", "rust", "rust_no_alloc", "rust_no_alloc_test"). The output
    /// will be printed on stdout in both cases.
    output_format: OutputFormat,

    #[argh(positional)]
    /// input file.
    input_file: String,
}

fn main() -> Result<(), String> {
    let opt: Opt = argh::from_env();

    if opt.version {
        println!("Packet Description Language parser version 1.0");
        return Ok(());
    }

    let mut sources = ast::SourceDatabase::new();
    match parser::parse_file(&mut sources, opt.input_file) {
        Ok(file) => {
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
                    println!("{}", backends::json::generate(&file).unwrap())
                }
                OutputFormat::Rust => {
                    println!("{}", backends::rust::generate(&sources, &analyzed_file))
                }
                OutputFormat::RustNoAlloc => {
                    let schema = backends::intermediate::generate(&file).unwrap();
                    println!("{}", backends::rust_no_allocation::generate(&file, &schema).unwrap())
                }
                OutputFormat::RustNoAllocTest => {
                    println!(
                        "{}",
                        backends::rust_no_allocation::test::generate_test_file().unwrap()
                    )
                }
            }
            Ok(())
        }

        Err(err) => {
            let writer = termcolor::StandardStream::stderr(termcolor::ColorChoice::Always);
            let config = term::Config::default();
            term::emit(&mut writer.lock(), &config, &sources, &err).expect("Could not print error");
            Err(String::from("Error while parsing input"))
        }
    }
}
