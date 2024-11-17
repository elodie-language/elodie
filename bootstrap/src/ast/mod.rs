pub use crate::ast::ast::SourceFile;
use crate::ast::lex::lex;
use crate::ast::parse::parse;

mod ast;
mod compiler;
mod lex;
mod parse;
mod token;

#[derive(Debug)]
pub enum Error {
    Lexer(lex::Error),
    Parser(parse::Error),
    Compiler(compiler::Error),
}

impl From<lex::Error> for Error {
    fn from(value: lex::Error) -> Self {
        Self::Lexer(value)
    }
}

impl From<parse::Error> for Error {
    fn from(value: parse::Error) -> Self {
        Self::Parser(value)
    }
}

impl From<compiler::Error> for Error {
    fn from(value: compiler::Error) -> Self {
        Self::Compiler(value)
    }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;


pub fn parse_str(str: &str) -> Result<SourceFile> {
    let tokens = lex(str)?;
    let root = parse(tokens)?;
    Ok(compiler::from(root)?)
}
