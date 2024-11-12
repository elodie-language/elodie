use crate::new_ast::ast::SourceFile;
use crate::new_ast::lex::Lexer;
use crate::new_ast::rewrite::Rewriter;

mod lex;
mod parse;
mod rewrite;
mod ast;

#[derive(Debug)]
pub enum Error {
    Lexer(lex::Error),
    Parser(parse::Error),
    Rewriter(rewrite::Error),
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

impl From<rewrite::Error> for Error {
    fn from(value: rewrite::Error) -> Self {
        Self::Rewriter(value)
    }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;


pub struct Parser {}

impl Parser {
    pub fn parse(str: &str) -> Result<SourceFile> {
        let tokens = Lexer::lex(str)?;
        let root = parse::Parser::parse(tokens)?;
        Ok(Rewriter::rewrite(root)?)
    }
}