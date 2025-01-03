use crate::common::Inferred;
use crate::common::node::Node;
use crate::frontend::ast::AstBlockNode;
use crate::ir::analyse::{TypeBlockNode, TypedTreeNode};
use crate::ir::analyse::pre::Pre;

impl<'a> Pre<'a> {
    pub(crate) fn block(&mut self, node: &AstBlockNode) -> crate::ir::analyse::Result<TypedTreeNode> {
        self.scope.enter();

        let mut nodes = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            nodes.push(self.node(node)?)
        }

        self.scope.leave();

        Ok(TypedTreeNode::new(
            Node::Block(
                TypeBlockNode {
                    nodes: nodes.into_boxed_slice()
                }
            ),
            self.span(),
            Inferred::Number,
        ))
    }
}