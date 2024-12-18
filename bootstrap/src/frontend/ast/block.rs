use crate::frontend::{ast, parse};
use crate::frontend::ast::Generator;
use crate::frontend::ast::node::BlockNode;

impl<'a> Generator<'a> {
    pub(crate) fn generate_block(&mut self, node: &parse::BlockNode) -> ast::Result<ast::Node> {
        let mut body = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            body.push(self.generate_node(node)?)
        }

        return Ok(ast::Node::Block(BlockNode { body }));
    }
}