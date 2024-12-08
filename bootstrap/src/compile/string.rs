use crate::compile::Compiler;
use crate::ir::{InterpolateStringNode, Node};
use crate::ir::Node::InterpolateString;
use crate::parse;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_interpolate_string(&mut self, node: &parse::StringInterpolationNode) -> crate::compile::Result<Node> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(self.compile_node(node)?);
        }
        Ok(InterpolateString(InterpolateStringNode { nodes }))
    }
}