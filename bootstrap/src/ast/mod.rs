pub use crate::ast::ast::*;
use crate::ast::lex::lex;
use crate::ast::parse::parse;

mod ast;
mod compile;
mod lex;
mod parse;
pub mod r#type;
mod modifier;

#[derive(Debug)]
pub enum Error {
    Lexer(lex::Error),
    Parser(parse::Error),
    Compiler(compile::Error),
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

impl From<compile::Error> for Error {
    fn from(value: compile::Error) -> Self {
        Self::Compiler(value)
    }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;


pub fn parse_str(str: &str) -> Result<SourceFile> {
    let tokens = lex(str)?;
    let root = parse(tokens)?;
    Ok(compile::from(root)?)
}
