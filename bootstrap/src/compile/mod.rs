use crate::{compile, ir, lex, parse};
use crate::common::Context;
use crate::compile::scope::Scope;
use crate::ir::SourceFile;
use crate::lex::lex;
use crate::parse::{parse, RootNode};

mod r#let;
mod validate;
mod collect;
mod infix;
mod literal;
mod r#loop;
mod r#if;
mod identifier;
mod block;
mod function;
mod package;
mod from;
mod r#type;
mod define;
mod external;
mod string;
mod scope;

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

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub fn compile_str(ctx: &mut Context, str: &str) -> Result<SourceFile> {
    let tokens = lex(ctx, str)?;
    let root = parse(ctx, tokens)?;
    let result = compile::from(ctx, root)?;
    Ok(result)
}


pub(crate) fn from(ctx: &mut Context, node: RootNode) -> Result<SourceFile> {
    let mut compiler = Compiler::new(ctx);
    compiler.compile(node)
}

pub(crate) struct Compiler<'a> {
    ctx: &'a mut Context,
    scope: Scope,
}

impl<'a> Compiler<'a> {
    fn new(ctx: &'a mut Context) -> Self {
        let mut scope = Scope::new();

        Self {
            ctx,
            scope,
        }
    }
}

impl<'a> Compiler<'a> {

    pub(crate) fn compile(&mut self, node: RootNode) -> Result<SourceFile> {
        // 2 pass
        // populate symbol table
        // create ir

        let mut result = Vec::new();
        for node in &node.nodes {
            if !matches!(node, parse::Node::Nop) {
                result.push(self.compile_node(node)?);
            }
        }

        Ok(SourceFile { body: result })
    }

    pub(crate) fn compile_node(&mut self, node: &parse::Node) -> Result<ir::Node> {
        match node {
            parse::Node::Block(block_node) => Ok(self.compile_block(block_node)?),
            parse::Node::Break(break_node) => Ok(self.compile_break(break_node)?),
            parse::Node::Continue(continue_node) => Ok(self.compile_continue(continue_node)?),
            parse::Node::DefineDeclaration(node) => Ok(self.compile_define(node)?),
            parse::Node::From(from_node) => Ok(self.compile_from(from_node)?),
            parse::Node::ExternalFunctionDeclaration(node) => self.compile_declare_external_function(node),
            parse::Node::FunctionDeclaration(declaration_node) => Ok(self.compile_declare_function(declaration_node)?),
            parse::Node::PackageDeclaration(declaration_node) => Ok(self.compile_declare_package(declaration_node)?),
            parse::Node::Identifier(identifier_node) => Ok(self.compile_identifier(identifier_node)?),
            parse::Node::Let(let_node) => Ok(self.compile_let(let_node)?),
            parse::Node::If(if_node) => Ok(self.compile_if(if_node)?),
            parse::Node::Infix(infix_node) => Ok(self.compile_infix(infix_node)?),
            parse::Node::StringInterpolation(node) => self.compile_interpolate_string(node),
            parse::Node::Itself(node) => Ok(self.compile_self(node)?),
            parse::Node::Literal(literal_node) => Ok(self.compile_literal(literal_node)?),
            parse::Node::Loop(loop_node) => Ok(self.compile_loop(loop_node)?),
            parse::Node::Return(return_node) => Ok(self.compile_function_return(return_node)?),
            parse::Node::TypeDeclaration(node) => Ok(self.compile_declare_type(node)?),
            _ => unimplemented!("{:?}", node)
        }
    }
}