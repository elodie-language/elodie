use crate::{ir, parse};
use crate::ir::BlockNode;
use crate::r#type::DefaultTypeIds;
use crate::compile::Compiler;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_block(&mut self, node: &parse::BlockNode) -> crate::compile::Result<ir::Node> {
        let mut body = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            body.push(self.compile_node(node)?)
        }

        return Ok(ir::Node::Block(BlockNode {
            body,
            return_type: DefaultTypeIds::never(),
        }));
    }
}