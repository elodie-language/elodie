use std::ops::Index;
use crate::common::Context;

use crate::frontend::lex::lex;
use crate::frontend::parse::{Node, parse};


pub mod lex;
pub mod parse;

#[derive(Debug)]
pub enum Error {
    Lexer(lex::Error),
    Parser(parse::Error),
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
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Parsed {
    pub nodes: Vec<Node>,
}

impl Index<usize> for Parsed {
    type Output = Node;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

impl From<Vec<Node>> for Parsed {
    fn from(value: Vec<Node>) -> Self {
        Self { nodes: value }
    }
}

impl Parsed {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

pub fn parse_str(ctx: &mut Context, str: &str) -> Result<Parsed> {
    let lexed = lex(ctx, str)?;
    let nodes = parse(ctx, lexed)?;
    Ok(Parsed { nodes })
}