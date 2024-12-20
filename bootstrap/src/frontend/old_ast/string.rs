use crate::frontend::old_ast::node::Node::InterpolateString;
use crate::frontend::old_ast::node::{InterpolateStringNode, Node};
use crate::frontend::old_ast::Generator;
use crate::frontend::{old_ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_interpolate_string(
        &mut self,
        node: &parse::StringInterpolationNode,
    ) -> old_ast::Result<Node> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(self.generate_node(node)?);
        }
        Ok(InterpolateString(InterpolateStringNode {
            span: node.token.span.clone(),
            nodes,
        }))
    }
}
