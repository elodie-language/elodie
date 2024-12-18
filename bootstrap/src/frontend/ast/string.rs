use crate::frontend::{ast, parse};
use crate::frontend::ast::Compiler;
use crate::frontend::ast::node::{InterpolateStringNode, Node};
use crate::frontend::ast::node::Node::InterpolateString;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_interpolate_string(&mut self, node: &parse::StringInterpolationNode) -> ast::Result<Node> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(self.compile_node(node)?);
        }
        Ok(InterpolateString(InterpolateStringNode { nodes }))
    }
}