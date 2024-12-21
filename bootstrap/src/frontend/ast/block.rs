use std::rc::Rc;

use crate::common::tree::{Node, TreeNode};
use crate::frontend::{ast, parse};
use crate::frontend::ast::{AstBlockNode, AstVariant, Generator, SPAN_NOT_IMPLEMENTED};

impl<'a> Generator<'a> {
    pub(crate) fn generate_block(&mut self, node: &parse::BlockNode) -> ast::Result<TreeNode<AstVariant>> {
        let mut nodes = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            nodes.push(self.generate_node(node)?)
        }

        Ok(TreeNode::new(Node::Block(Rc::new(AstBlockNode {
            nodes
        })), SPAN_NOT_IMPLEMENTED.clone()))
    }
}
