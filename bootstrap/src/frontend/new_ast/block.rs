use crate::frontend::{new_ast, parse};
use crate::frontend::new_ast::Generator;
use crate::frontend::new_ast::node::{AstNode, Node, SPAN_NOT_IMPLEMENTED};

impl<'a> Generator<'a> {
    pub(crate) fn generate_block(&mut self, node: &parse::BlockNode) -> new_ast::Result<AstNode> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(self.generate_node(node)?)
        }

        return Ok(AstNode::new(Node::Block { nodes }, SPAN_NOT_IMPLEMENTED.clone()));
    }
}
