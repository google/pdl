//! PDL parser and linter.

use codespan_reporting::term::{self, termcolor};
use structopt::StructOpt;

mod ast;
mod backends;
mod lint;
mod parser;
#[cfg(test)]
mod test_utils;

use crate::lint::Lintable;

#[derive(Debug)]
enum OutputFormat {
    JSON,
    Rust,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "json" => Ok(Self::JSON),
            "rust" => Ok(Self::Rust),
            _ => Err(format!("could not parse {:?}, valid option are 'json' and 'rust'.", input)),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "pdl-parser", about = "Packet Description Language parser tool.")]
struct Opt {
    /// Print tool version and exit.
    #[structopt(short, long = "--version")]
    version: bool,

    /// Generate output in this format ("json" or "rust"). The output
    /// will be printed on stdout in both cases.
    #[structopt(short, long = "--output-format", name = "FORMAT", default_value = "JSON")]
    output_format: OutputFormat,

    /// Input file.
    #[structopt(name = "FILE")]
    input_file: String,
}

fn main() -> std::process::ExitCode {
    let opt = Opt::from_args();

    if opt.version {
        println!("Packet Description Language parser version 1.0");
        return std::process::ExitCode::SUCCESS;
    }

    let mut sources = ast::SourceDatabase::new();
    match parser::parse_file(&mut sources, opt.input_file) {
        Ok(file) => {
            let lint = file.lint();
            if !lint.diagnostics.is_empty() {
                lint.print(&sources, termcolor::ColorChoice::Always)
                    .expect("Could not print lint diagnostics");
                return std::process::ExitCode::FAILURE;
            }

            match opt.output_format {
                OutputFormat::JSON => {
                    println!("{}", backends::json::generate(&file).unwrap())
                }
                OutputFormat::Rust => {
                    println!("{}", backends::rust::generate(&sources, &file))
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
