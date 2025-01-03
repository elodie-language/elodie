use crate::common::Inferred;
use crate::common::node::Node;
use crate::frontend::ast::AstCompareNode;
use crate::ir::analyse::{TypeCompareNode, TypedTreeNode};
use crate::ir::analyse::pre::Pre;

impl<'a> Pre<'a> {
    pub(crate) fn compare(&mut self, node: &AstCompareNode) -> crate::ir::analyse::Result<TypedTreeNode> {
        let left = self.node(node.left.as_ref())?;
        let right = self.node(node.right.as_ref())?;
        Ok(
            TypedTreeNode::new(
                Node::Compare(TypeCompareNode {
                    left: Box::new(left),
                    operator: node.operator.clone(),
                    right: Box::new(right),
                }),
                self.span(),
                Inferred::Boolean,
            )
        )
    }
}