use std::{env, io};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod parser;
mod core;
mod interpreter;
mod cli;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut path = PathBuf::from(args.get(1).unwrap());

    let content = load_text_from_file(path.to_str().unwrap()).unwrap();

    let lexer = Lexer::new(content.as_str());
    let tokens = lexer.all().unwrap();

    let mut parser = Parser::new(&tokens);
    let result = parser.parse().unwrap();
    // println!("{result:?}");

    let interpreter = Interpreter::new();
    interpreter.interpret(result).unwrap();
}

fn load_text_from_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}