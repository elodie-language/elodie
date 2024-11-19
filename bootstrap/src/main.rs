use std::{env, io};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::runner::Runner;

mod common;
mod cli;
mod ast;
mod runner;

fn main() {
    let args: Vec<String> = env::args().collect();

    // load modules

    let mut path = PathBuf::from(args.get(1).unwrap());
    let content = load_text_from_file(path.to_str().unwrap()).unwrap();

    let source_file = ast::parse_str(content.as_str()).unwrap();

    let mut runner = Runner::new();
    runner.run(source_file).unwrap();
}

fn load_text_from_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}