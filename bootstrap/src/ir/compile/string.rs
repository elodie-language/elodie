use crate::ir::compile::Compiler;
use crate::ir::{InterpolateStringNode, Node};
use crate::ir::Node::InterpolateString;
use crate::frontend::parse;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_interpolate_string(&mut self, node: &parse::StringInterpolationNode) -> crate::ir::compile::Result<Node> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(self.compile_node(node)?);
        }
        Ok(InterpolateString(InterpolateStringNode { nodes }))
    }
}