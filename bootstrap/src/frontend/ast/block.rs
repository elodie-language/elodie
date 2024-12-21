use std::rc::Rc;

use crate::common::node::Node;
use crate::frontend::ast::{AstBlockNode, AstTreeNode, Generator, SPAN_NOT_IMPLEMENTED};
use crate::frontend::{ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_block(&mut self, node: &parse::BlockNode) -> ast::Result<AstTreeNode> {
        let mut nodes = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            nodes.push(self.generate_node(node)?)
        }

        Ok(AstTreeNode::new(
            Node::Block(AstBlockNode { nodes }),
            SPAN_NOT_IMPLEMENTED.clone(),
        ))
    }
}
