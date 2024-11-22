use std::{env, io};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::run::Runner;

mod common;
mod cli;
mod ast;
mod run;
mod lex;
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut runner = Runner::new();

    let std_content = load_library_file("std/index.elx").unwrap();
    let std_file = ast::parse_str(std_content.as_str()).unwrap();

    runner.run(std_file).unwrap();

    let mut path = PathBuf::from(args.get(1).unwrap());
    let content = load_text_from_file(path.to_str().unwrap()).unwrap();
    let source_file = ast::parse_str(content.as_str()).unwrap();

    runner.run(source_file).unwrap();
}

fn load_text_from_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn load_library_file(filename: &str) -> io::Result<String> {
    let manifest_dir = "/home/ddymke/repo/elodie/src/lib/";
    let file_path = PathBuf::from(manifest_dir).join(filename);

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
