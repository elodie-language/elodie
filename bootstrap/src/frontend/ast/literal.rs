use crate::common::BaseType;
use crate::frontend::{ast, parse};
use crate::frontend::ast::Compiler;
use crate::frontend::ast::node::{LiteralBoolNode, LiteralNode, LiteralNumberNode, LiteralStringNode};

impl<'a> Compiler<'a> {
    pub(crate) fn compile_literal(
        &mut self,
        node: &parse::LiteralNode,
    ) -> ast::Result<ast::Node> {
        match node {
            parse::LiteralNode::Number(v) => Ok(ast::Node::Literal(LiteralNode::Number(LiteralNumberNode {
                value: v.value().clone(),
                ty: self.ctx.type_table.get_base_type_id(&BaseType::Number),
            }))),
            parse::LiteralNode::String(v) => Ok(ast::Node::Literal(LiteralNode::String(LiteralStringNode {
                value: v.value().clone(),
                ty: self.ctx.type_table.get_base_type_id(&BaseType::String),
            }))),
            parse::LiteralNode::Boolean(v) => Ok(ast::Node::Literal(LiteralNode::Bool(LiteralBoolNode {
                value: v.value().clone(),
                ty: self.ctx.type_table.get_base_type_id(&BaseType::Boolean),
            }))),
        }
    }
}
