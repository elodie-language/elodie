use std::rc::Rc;

use crate::common::tree::Node::InterpolateString;
use crate::common::tree::TreeNode;
use crate::frontend::{ast, parse};
use crate::frontend::ast::{AstInterpolateStringNode, AstVariant, Generator, SPAN_NOT_IMPLEMENTED};

impl<'a> Generator<'a> {
    pub(crate) fn generate_interpolate_string(
        &mut self,
        node: &parse::StringInterpolationNode,
    ) -> ast::Result<TreeNode<AstVariant>> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(self.generate_node(node)?);
        }
        Ok(TreeNode::new(InterpolateString(Rc::new(AstInterpolateStringNode {
            nodes,
        })), SPAN_NOT_IMPLEMENTED.clone()))
    }
}
