use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod parser;
mod core;
mod interpreter;
mod cli;

fn main() {
    let lexer = Lexer::new("console.log('Elodie says hi')");
    let tokens = lexer.all().unwrap();

    let mut parser = Parser::new(&tokens);
    let result = parser.parse().unwrap();
    // println!("{result:?}");

    let interpreter = Interpreter::new();
    interpreter.interpret(result).unwrap();
}
