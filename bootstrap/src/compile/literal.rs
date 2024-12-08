use crate::compile::Compiler;
use crate::{ir, parse};
use crate::ir::{LiteralBoolNode, LiteralNode, LiteralNumberNode, LiteralStringNode};

use crate::r#type::DefaultTypeIds;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_literal(
        &mut self,
        node: &parse::LiteralNode,
    ) -> crate::compile::Result<ir::Node> {
        match node {
            parse::LiteralNode::Number(v) => Ok(ir::Node::Literal(LiteralNode::Number(LiteralNumberNode {
                value: v.value().clone(),
                ty: DefaultTypeIds::never(),
            }))),
            parse::LiteralNode::String(v) => Ok(ir::Node::Literal(LiteralNode::String(LiteralStringNode {
                value: v.value().clone(),
                ty: DefaultTypeIds::never(),
            }))),
            parse::LiteralNode::Boolean(v) => Ok(ir::Node::Literal(LiteralNode::Bool(LiteralBoolNode {
                value: v.value().clone(),
                ty: DefaultTypeIds::never(),
            }))),
        }
    }
}
