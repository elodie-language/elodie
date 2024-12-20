use crate::frontend::old_ast::node::BlockNode;
use crate::frontend::old_ast::Generator;
use crate::frontend::{old_ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_block(&mut self, node: &parse::BlockNode) -> old_ast::Result<old_ast::Node> {
        let mut body = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            body.push(self.generate_node(node)?)
        }

        return Ok(old_ast::Node::Block(BlockNode { body }));
    }
}
