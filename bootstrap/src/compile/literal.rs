use crate::compile::Compiler;
use crate::ir;
use crate::ir::{LiteralBoolNode, LiteralNumberNode, LiteralStringNode};
use crate::parse::LiteralNode;
use crate::r#type::DefaultTypeIds;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_literal(
        &mut self,
        node: &LiteralNode,
    ) -> crate::compile::Result<ir::Node> {
        match node {
            LiteralNode::Number(v) => Ok(ir::Node::LiteralNumber(LiteralNumberNode {
                value: v.value(),
                type_id: DefaultTypeIds::never(),
            })),
            LiteralNode::String(v) => Ok(ir::Node::LiteralString(LiteralStringNode {
                value: v.value(),
                type_id: DefaultTypeIds::never(),
            })),
            LiteralNode::Boolean(v) => Ok(ir::Node::LiteralBoolean(LiteralBoolNode {
                value: v.value(),
                type_id: DefaultTypeIds::never(),
            })),
        }
    }
}
