//! PDL parser and analyzer.

use clap::Parser;
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

#[derive(Parser, Debug)]
#[clap(name = "pdl-parser", about = "Packet Description Language parser tool.")]
struct Opt {
    /// Print tool version and exit.
    #[clap(short, long = "version")]
    version: bool,

    /// Generate output in this format ("json", "rust", "rust_no_alloc", "rust_no_alloc_test"). The output
    /// will be printed on stdout in both cases.
    #[clap(short, long = "output-format", name = "FORMAT", default_value = "JSON")]
    output_format: OutputFormat,

    /// Input file.
    #[clap(name = "FILE")]
    input_file: String,
}

fn main() -> std::process::ExitCode {
    let opt = Opt::parse();

    if opt.version {
        println!("Packet Description Language parser version 1.0");
        return std::process::ExitCode::SUCCESS;
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
                    return std::process::ExitCode::FAILURE;
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
            std::process::ExitCode::SUCCESS
        }

        Err(err) => {
            let writer = termcolor::StandardStream::stderr(termcolor::ColorChoice::Always);
            let config = term::Config::default();
            term::emit(&mut writer.lock(), &config, &sources, &err).expect("Could not print error");
            std::process::ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_opt() {
        Opt::command().debug_assert();
    }
}
