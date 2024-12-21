use std::rc::Rc;

use crate::common::node::Node::InterpolateString;
use crate::frontend::{ast, parse};
use crate::frontend::ast::{AstInterpolateStringNode, AstTreeNode, Generator, SPAN_NOT_IMPLEMENTED};

impl<'a> Generator<'a> {
    pub(crate) fn generate_interpolate_string(
        &mut self,
        node: &parse::StringInterpolationNode,
    ) -> ast::Result<AstTreeNode> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(self.generate_node(node)?);
        }
        Ok(AstTreeNode::new(InterpolateString(AstInterpolateStringNode {
            nodes,
        }), SPAN_NOT_IMPLEMENTED.clone()))
    }
}
