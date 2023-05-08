mod parser;
mod ast;
mod codegen;
mod patterns;

use std::path::PathBuf;
use std::fs;
use std::io::{stdin, Read};
use clap::Parser;

#[derive(Parser)]
#[command(author, version)]
/// Supercharging WebAssembly to JavaScript Compiler
struct Cli {
    #[arg(short, long, value_name = "FILE", help = "Path to the wasm module. Will read from stdin if ommited.")]
    path: Option<PathBuf>
}

fn main() {
    let cli = Cli::parse();

    let src = match cli.path.as_deref() {
        Some(path) => fs::read(path).expect("Could not read file from path!"),
        None => stdin().bytes().collect::<Result<Vec<u8>, std::io::Error>>().expect("Could not read file from stdin!"),
    };

    let ast = match parser::parse_wasm(src.as_slice()) {
        Ok(ast) => ast,
        Err(error) => panic!("Module is invalid! Error: {}", error.message()),
    };

    codegen::generate_js(ast);
}
