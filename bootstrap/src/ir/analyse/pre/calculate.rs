use crate::common::node::Node;
use crate::frontend::ast::AstCalculateNode;
use crate::ir::analyse::{TypeCalculateNode, TypedTreeNode};
use crate::ir::analyse::pre::Pre;

impl<'a> Pre<'a> {
    pub(crate) fn calculate(&mut self, node: &AstCalculateNode) -> crate::ir::analyse::Result<TypedTreeNode> {
        let left = self.node(node.left.as_ref())?;
        let right = self.node(node.right.as_ref())?;

        let inferred = left.inferred.clone();

        Ok(
            TypedTreeNode::new(
                Node::Calculate(TypeCalculateNode {
                    left: Box::new(left),
                    operator: node.operator.clone(),
                    right: Box::new(right),
                }),
                self.span(),
                inferred,
            )
        )
    }
}