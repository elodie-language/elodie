use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod parser;
mod core;
mod interpreter;
mod cli;

fn main() {
    println!("This is where it all began!");

    // let parser = Parser::new("console.log('Elodie says hi')");
    // let ast = parser.parse().unwrap();
    // println!("{ast:?}");

    let lexer = Lexer::new("3 + 5 * 2");
    let tokens = lexer.all().unwrap();

    let mut parser = Parser::new(&tokens);
    let result = parser.parse().unwrap();
}
