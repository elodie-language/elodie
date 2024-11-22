use crate::ast;
use crate::compile::Compiler;
use crate::parse::LiteralNode;

impl Compiler {
    pub(crate) fn compile_literal(&mut self, node: &LiteralNode) -> crate::compile::Result<ast::Node> {
        match node {
            LiteralNode::Number(v) => Ok(ast::Node::ValueNumber(v.value().unwrap())),
            LiteralNode::String(v) => Ok(ast::Node::ValueString(v.value().to_string())),
            LiteralNode::Boolean(v) => Ok(ast::Node::ValueBoolean(v.value()))
        }
    }
}