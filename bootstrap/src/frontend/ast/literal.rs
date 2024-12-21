use crate::common::node::Node::{LiteralBoolean, LiteralNumber, LiteralString};
use crate::frontend::ast::{
    AstLiteralBooleanNode, AstLiteralNumberNode, AstLiteralStringNode, AstTreeNode, Generator,
    SPAN_NOT_IMPLEMENTED,
};
use crate::frontend::{ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_literal(
        &mut self,
        node: &parse::LiteralNode,
    ) -> ast::Result<AstTreeNode> {
        match node {
            parse::LiteralNode::Boolean(v) => Ok(AstTreeNode::new(
                LiteralBoolean(AstLiteralBooleanNode(v.0.clone())),
                SPAN_NOT_IMPLEMENTED.clone(),
            )),
            parse::LiteralNode::Number(v) => Ok(AstTreeNode::new(
                LiteralNumber(AstLiteralNumberNode(v.0.clone())),
                SPAN_NOT_IMPLEMENTED.clone(),
            )),
            parse::LiteralNode::String(v) => Ok(AstTreeNode::new(
                LiteralString(AstLiteralStringNode(v.0.clone())),
                SPAN_NOT_IMPLEMENTED.clone(),
            )),
        }
    }
}
