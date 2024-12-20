use std::ops::Index;

use crate::frontend::ast::node::*;
pub use crate::frontend::context::Context;
use crate::frontend::lex::lex;
use crate::frontend::new_ast::node::AstNode;
use crate::frontend::parse::parse;

pub mod new_ast;
pub mod ast;
pub mod lex;
pub mod parse;
pub mod modifier;
pub mod context;

#[derive(Debug)]
pub enum Error {
    Lexer(lex::Error),
    Parser(parse::Error),
    Ast(ast::Error),
    NewAst(new_ast::Error),
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

impl From<ast::Error> for Error {
    fn from(value: ast::Error) -> Self {
        Self::Ast(value)
    }
}

impl From<new_ast::Error> for Error {
    fn from(value: new_ast::Error) -> Self {
        Self::NewAst(value)
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Node>,
}

impl Index<usize> for Ast {
    type Output = Node;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

impl From<Vec<Node>> for Ast {
    fn from(value: Vec<Node>) -> Self {
        Self { nodes: value }
    }
}

impl Ast {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

pub fn ast_from_str(ctx: &mut Context, str: &str) -> Result<Ast> {
    let lexed = lex(ctx, str)?;
    let nodes = parse(ctx, lexed)?;
    Ok(ast::from(ctx, nodes)?)
}

#[derive(Debug)]
pub struct NewAst {
    pub nodes: Vec<AstNode>,
}

impl Index<usize> for NewAst {
    type Output = AstNode;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

impl From<Vec<AstNode>> for NewAst {
    fn from(value: Vec<AstNode>) -> Self {
        Self { nodes: value }
    }
}

impl NewAst {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

pub fn new_ast_from_str(ctx: &mut Context, str: &str) -> Result<NewAst> {
    let lexed = lex(ctx, str)?;
    let nodes = parse(ctx, lexed)?;
    Ok(new_ast::from(ctx, nodes)?)
}
