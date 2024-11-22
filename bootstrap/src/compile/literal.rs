use crate::ir;
use crate::compile::Compiler;
use crate::parse::LiteralNode;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_literal(&mut self, node: &LiteralNode) -> crate::compile::Result<ir::Node> {
        match node {
            LiteralNode::Number(v) => Ok(ir::Node::ValueNumber(
                self.ctx.get_str(v.value()).parse().unwrap()
            )),
            LiteralNode::String(v) => Ok(ir::Node::ValueString(
                self.ctx.get_str(v.value()).to_string()
            )),
            LiteralNode::Boolean(v) => Ok(ir::Node::ValueBoolean(v.value()))
        }
    }
}