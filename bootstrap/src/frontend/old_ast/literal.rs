use crate::frontend::old_ast::node::{LiteralNode, LiteralNumberNode, LiteralStringNode};
use crate::frontend::old_ast::{Generator, LiteralBooleanNode};
use crate::frontend::{old_ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_literal(&mut self, node: &parse::LiteralNode) -> old_ast::Result<old_ast::Node> {
        match node {
            parse::LiteralNode::Number(v) => Ok(old_ast::Node::Literal(LiteralNode::Number(
                LiteralNumberNode(v.0.clone()),
            ))),
            parse::LiteralNode::String(v) => Ok(old_ast::Node::Literal(LiteralNode::String(
                LiteralStringNode(v.0.clone()),
            ))),
            parse::LiteralNode::Boolean(v) => Ok(old_ast::Node::Literal(LiteralNode::Boolean(
                LiteralBooleanNode(v.0.clone()),
            ))),
        }
    }
}
