use crate::core::token::{Token, TokenKind};
use crate::new_ast::ast::SourceFile;

mod parse;
mod rewrite;
mod ast;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfFile,
    UnexpectedToken {
        expected: TokenKind,
        got: Token,
    },
    UnsupportedToken(Token),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub struct Parser<'a> {
    tokens: &'a [Token],
}

impl<'a> Parser<'a> {
    pub fn parse(tokens: &'a [Token]) -> Result<SourceFile> {
        todo!()
    }
}