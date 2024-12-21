use crate::common::tree::TreeNode;
use crate::frontend::{Ast, Context, parse};
pub use crate::frontend::ast::node::*;

mod block;
mod control;
mod function;
mod identifier;
mod infix;
mod literal;
pub(crate) mod node;
mod package;
mod string;
mod r#type;
mod variable;


#[derive(Debug)]
pub enum Error {}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub(crate) fn from(ctx: &mut Context, nodes: Vec<parse::Node>) -> Result<Ast> {
    let mut compiler = Generator::new(ctx);
    compiler.generate(nodes)
}

pub(crate) struct Generator<'a> {
    ctx: &'a mut Context,
}

impl<'a> Generator<'a> {
    fn new(ctx: &'a mut Context) -> Self {
        Self { ctx }
    }
}

impl<'a> Generator<'a> {
    pub(crate) fn generate(&mut self, nodes: Vec<parse::Node>) -> Result<Ast> {
        let mut result = Vec::new();
        for node in &nodes {
            if !matches!(node, parse::Node::Nop) {
                result.push(self.generate_node(node)?);
            }
        }

        Ok(Ast { nodes: result })
    }

    pub(crate) fn generate_node(&mut self, node: &parse::Node) -> Result<TreeNode<AstVariant>> {
        match node {
            parse::Node::Block(block_node) => Ok(self.generate_block(block_node)?),
            parse::Node::Break(break_node) => Ok(self.generate_break(break_node)?),
            parse::Node::Continue(continue_node) => Ok(self.generate_continue(continue_node)?),
            parse::Node::DefineDeclaration(node) => Ok(self.generate_define_type(node)?),
            parse::Node::From(from_node) => Ok(self.generate_from(from_node)?),
            parse::Node::ExternalFunctionDeclaration(node) => {
                self.generate_declare_external_function(node)
            }
            parse::Node::FunctionDeclaration(declaration_node) => {
                Ok(self.generate_declare_function(declaration_node)?)
            }
            parse::Node::PackageDeclaration(declaration_node) => {
                Ok(self.generate_declare_package(declaration_node)?)
            }
            parse::Node::Identifier(identifier_node) => {
                Ok(self.generate_identifier(identifier_node)?)
            }
            parse::Node::If(if_node) => Ok(self.generate_if(if_node)?),
            parse::Node::Infix(infix_node) => Ok(self.generate_infix(infix_node)?),
            parse::Node::StringInterpolation(node) => self.generate_interpolate_string(node),
            parse::Node::Itself(node) => Ok(self.generate_self(node)?),
            parse::Node::Literal(literal_node) => Ok(self.generate_literal(literal_node)?),
            parse::Node::Loop(loop_node) => Ok(self.generate_loop(loop_node)?),
            parse::Node::Return(return_node) => Ok(self.generate_function_return(return_node)?),
            parse::Node::TypeDeclaration(node) => Ok(self.generate_declare_type(node)?),
            parse::Node::VariableDeclaration(let_node) => Ok(self.generate_declare_variable(let_node)?),
            _ => unimplemented!("{:?}", node),
        }
    }
}
