use crate::frontend::{ast, parse};
use crate::frontend::ast::{Generator, SPAN_NOT_IMPLEMENTED};
use crate::frontend::ast::node::{AstNode, InterpolateStringNode};
use crate::frontend::ast::node::Node::InterpolateString;

impl<'a> Generator<'a> {
    pub(crate) fn generate_interpolate_string(
        &mut self,
        node: &parse::StringInterpolationNode,
    ) -> ast::Result<AstNode> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(self.generate_node(node)?);
        }
        Ok(AstNode::new(InterpolateString(InterpolateStringNode {
            nodes,
        }), SPAN_NOT_IMPLEMENTED.clone()))
    }
}
