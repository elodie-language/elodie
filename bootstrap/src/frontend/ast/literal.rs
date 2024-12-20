use crate::frontend::{ast, parse};
use crate::frontend::ast::{Generator, LiteralBooleanNode, LiteralNumberNode, LiteralStringNode, Node, SPAN_NOT_IMPLEMENTED};
use crate::frontend::ast::node::AstNode;

impl<'a> Generator<'a> {
    pub(crate) fn generate_literal(&mut self, node: &parse::LiteralNode) -> ast::Result<AstNode> {
        match node {
            parse::LiteralNode::Boolean(v) => Ok(AstNode::new(Node::LiteralBoolean(LiteralBooleanNode(v.0.clone())), SPAN_NOT_IMPLEMENTED.clone())),
            parse::LiteralNode::Number(v) => Ok(AstNode::new(Node::LiteralNumber(LiteralNumberNode(v.0.clone())), SPAN_NOT_IMPLEMENTED.clone())),
            parse::LiteralNode::String(v) => Ok(AstNode::new(Node::LiteralString(LiteralStringNode(v.0.clone())), SPAN_NOT_IMPLEMENTED.clone())),
        }
    }
}
