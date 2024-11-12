use std::{env, io};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod parser;
mod core;
mod interpreter;
mod cli;
mod ast;
mod new_ast;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut path = PathBuf::from(args.get(1).unwrap());
    let content = load_text_from_file(path.to_str().unwrap()).unwrap();

    let ast = new_ast::parse_str(content.as_str()).unwrap();

//     let tokens = Lexer::lex(content.as_str()).unwrap();
//     let result = Parser::parse(&tokens).unwrap();
//     // println!("{result:?}");
//
//     let mut interpreter = Interpreter::new();
//     interpreter.interpret(result).unwrap();
}

fn load_text_from_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}