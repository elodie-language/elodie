use crate::common::DefaultTypeIds;
use crate::frontend::parse;
use crate::ir;
use crate::ir::BlockNode;
use crate::ir::compile::Compiler;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_block(&mut self, node: &parse::BlockNode) -> crate::ir::compile::Result<ir::Node> {
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