use crate::frontend::{ast, parse};
use crate::frontend::ast::{BlockNode, Generator};
use crate::frontend::ast::node::{AstNode, Node, SPAN_NOT_IMPLEMENTED};

impl<'a> Generator<'a> {
    pub(crate) fn generate_block(&mut self, node: &parse::BlockNode) -> ast::Result<AstNode> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(self.generate_node(node)?)
        }

        return Ok(AstNode::new(Node::Block(BlockNode { nodes }), SPAN_NOT_IMPLEMENTED.clone()));
    }
}
