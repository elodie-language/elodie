use crate::common::BaseType;
use crate::frontend::parse;
use crate::ir;
use crate::ir::{LiteralBoolNode, LiteralNode, LiteralNumberNode, LiteralStringNode};
use crate::ir::compile::Compiler;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_literal(
        &mut self,
        node: &parse::LiteralNode,
    ) -> crate::ir::compile::Result<ir::Node> {
        match node {
            parse::LiteralNode::Number(v) => Ok(ir::Node::Literal(LiteralNode::Number(LiteralNumberNode {
                value: v.value().clone(),
                ty: self.ctx.type_table.get_base_type_id(&BaseType::Number),
            }))),
            parse::LiteralNode::String(v) => Ok(ir::Node::Literal(LiteralNode::String(LiteralStringNode {
                value: v.value().clone(),
                ty: self.ctx.type_table.get_base_type_id(&BaseType::String),
            }))),
            parse::LiteralNode::Boolean(v) => Ok(ir::Node::Literal(LiteralNode::Bool(LiteralBoolNode {
                value: v.value().clone(),
                ty: self.ctx.type_table.get_base_type_id(&BaseType::Boolean),
            }))),
        }
    }
}
