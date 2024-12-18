use crate::frontend::{ast, parse};
use crate::frontend::ast::{Generator, LiteralBooleanNode};
use crate::frontend::ast::node::{LiteralNode, LiteralNumberNode, LiteralStringNode};

impl<'a> Generator<'a> {
    pub(crate) fn generator_literal(
        &mut self,
        node: &parse::LiteralNode,
    ) -> ast::Result<ast::Node> {
        match node {
            parse::LiteralNode::Number(v) => Ok(ast::Node::Literal(LiteralNode::Number(LiteralNumberNode(v.0.clone())))),
            parse::LiteralNode::String(v) => Ok(ast::Node::Literal(LiteralNode::String(LiteralStringNode(v.0.clone())))),
            parse::LiteralNode::Boolean(v) => Ok(ast::Node::Literal(LiteralNode::Boolean(LiteralBooleanNode(v.0.clone()
            )))),
        }
    }
}
