use crate::common::Context;
use crate::frontend::{Ast, parse};
pub use crate::frontend::ast::node::*;

mod r#let;
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
pub mod node;

#[derive(Debug)]
pub enum Error {}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;


pub(crate) fn from(ctx: &mut Context, nodes: Vec<parse::Node>) -> Result<Ast> {
    let mut compiler = Generator::new(ctx);
    compiler.compile(nodes)
}

pub(crate) struct Generator<'a> {
    ctx: &'a mut Context,
    // scope: Scope,
}

impl<'a> Generator<'a> {
    fn new(ctx: &'a mut Context) -> Self {
        // let mut scope = Scope::new();

        Self {
            ctx,
            // scope,
        }
    }
}

impl<'a> Generator<'a> {
    pub(crate) fn compile(&mut self, nodes: Vec<parse::Node>) -> Result<Ast> {
        // 2 pass
        // populate symbol table
        // create ir

        let mut result = Vec::new();
        for node in &nodes {
            if !matches!(node, parse::Node::Nop) {
                result.push(self.generator_node(node)?);
            }
        }

        Ok(Ast { nodes: result })
    }

    pub(crate) fn generator_node(&mut self, node: &parse::Node) -> Result<node::Node> {
        match node {
            parse::Node::Block(block_node) => Ok(self.generator_block(block_node)?),
            parse::Node::Break(break_node) => Ok(self.generator_break(break_node)?),
            parse::Node::Continue(continue_node) => Ok(self.generator_continue(continue_node)?),
            parse::Node::DefineDeclaration(node) => Ok(self.generator_define(node)?),
            parse::Node::From(from_node) => Ok(self.generator_from(from_node)?),
            parse::Node::ExternalFunctionDeclaration(node) => self.generator_declare_external_function(node),
            parse::Node::FunctionDeclaration(declaration_node) => Ok(self.generator_declare_function(declaration_node)?),
            parse::Node::PackageDeclaration(declaration_node) => Ok(self.generator_declare_package(declaration_node)?),
            parse::Node::Identifier(identifier_node) => Ok(self.generator_identifier(identifier_node)?),
            parse::Node::VariableDeclaration(let_node) => Ok(self.generator_let(let_node)?),
            parse::Node::If(if_node) => Ok(self.generator_if(if_node)?),
            parse::Node::Infix(infix_node) => Ok(self.generator_infix(infix_node)?),
            parse::Node::StringInterpolation(node) => self.generator_interpolate_string(node),
            parse::Node::Itself(node) => Ok(self.generator_self(node)?),
            parse::Node::Literal(literal_node) => Ok(self.generator_literal(literal_node)?),
            parse::Node::Loop(loop_node) => Ok(self.generator_loop(loop_node)?),
            parse::Node::Return(return_node) => Ok(self.generator_function_return(return_node)?),
            parse::Node::TypeDeclaration(node) => Ok(self.declare_type(node)?),
            _ => unimplemented!("{:?}", node)
        }
    }
}
