pub mod ast;
pub mod cli;
pub mod compiler;
pub mod lexer;
pub mod parser;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    compiler::compile(&args);
}
