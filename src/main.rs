//! PDL parser and linter.

extern crate codespan_reporting;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;
extern crate serde_json;
extern crate structopt;

use codespan_reporting::term;
use codespan_reporting::term::termcolor;
use structopt::StructOpt;

mod ast;
mod lint;
mod parser;

use crate::lint::Lintable;

#[derive(Debug, StructOpt)]
#[structopt(name = "pdl-parser", about = "Packet Description Language parser tool.")]
struct Opt {
    /// Print tool version and exit.
    #[structopt(short, long = "--version")]
    version: bool,

    /// Input file.
    #[structopt(name = "FILE")]
    input_file: String,
}

fn main() {
    let opt = Opt::from_args();

    if opt.version {
        println!("Packet Description Language parser version 1.0");
        return;
    }

    let mut sources = ast::SourceDatabase::new();
    match parser::parse_file(&mut sources, opt.input_file) {
        Ok(grammar) => {
            let _ = grammar.lint().print(&sources, termcolor::ColorChoice::Always);
            println!("{}", serde_json::to_string_pretty(&grammar).unwrap())
        }
        Err(err) => {
            let writer = termcolor::StandardStream::stderr(termcolor::ColorChoice::Always);
            let config = term::Config::default();
            _ = term::emit(&mut writer.lock(), &config, &sources, &err);
        }
    }
}
