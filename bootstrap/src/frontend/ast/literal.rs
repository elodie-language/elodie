use std::rc::Rc;

use crate::common::tree::Node::{LiteralBoolean, LiteralNumber, LiteralString};
use crate::common::tree::TreeNode;
use crate::frontend::{ast, parse};
use crate::frontend::ast::{AstLiteralBooleanNode, AstLiteralNumberNode, AstLiteralStringNode, AstVariant, Generator, SPAN_NOT_IMPLEMENTED};

impl<'a> Generator<'a> {
    pub(crate) fn generate_literal(&mut self, node: &parse::LiteralNode) -> ast::Result<TreeNode<AstVariant>> {
        match node {
            parse::LiteralNode::Boolean(v) => Ok(TreeNode::new(LiteralBoolean(Rc::new(AstLiteralBooleanNode(v.0.clone()))), SPAN_NOT_IMPLEMENTED.clone())),
            parse::LiteralNode::Number(v) => Ok(TreeNode::new(LiteralNumber(Rc::new(AstLiteralNumberNode(v.0.clone()))), SPAN_NOT_IMPLEMENTED.clone())),
            parse::LiteralNode::String(v) => Ok(TreeNode::new(LiteralString(Rc::new(AstLiteralStringNode(v.0.clone()))), SPAN_NOT_IMPLEMENTED.clone())),
        }
    }
}
