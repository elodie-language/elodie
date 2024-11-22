use crate::ast;
use crate::compile::Compiler;
use crate::parse::LiteralNode;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_literal(&mut self, node: &LiteralNode) -> crate::compile::Result<ast::Node> {
        match node {
            LiteralNode::Number(v) => Ok(ast::Node::ValueNumber(
                self.ctx.get_str(v.value()).parse().unwrap()
            )),
            LiteralNode::String(v) => Ok(ast::Node::ValueString(
                self.ctx.get_str(v.value()).to_string()
            )),
            LiteralNode::Boolean(v) => Ok(ast::Node::ValueBoolean(v.value()))
        }
    }
}