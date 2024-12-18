use crate::frontend::{ast, parse};
use crate::frontend::ast::Generator;
use crate::frontend::ast::node::{InterpolateStringNode, Node};
use crate::frontend::ast::node::Node::InterpolateString;

impl<'a> Generator<'a> {
    pub(crate) fn generate_interpolate_string(&mut self, node: &parse::StringInterpolationNode) -> ast::Result<Node> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(self.generate_node(node)?);
        }
        Ok(InterpolateString(InterpolateStringNode { nodes }))
    }
}