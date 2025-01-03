use std::rc::Rc;

use crate::common::node::Node;
use crate::common::Span;
use crate::ir::{IrCompareNode, IrTreeNode};
use crate::ir::analyse::TypeCompareNode;
use crate::ir::generate::Generator;

impl<'a> Generator<'a> {
    pub(crate) fn compare(&mut self, node: &TypeCompareNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        let left = self.node(node.left.as_ref())?;
        let right = self.node(node.right.as_ref())?;

        Ok(
            IrTreeNode::new(
                Node::Compare(IrCompareNode {
                    left: Rc::new(left),
                    operator: node.operator.clone(),
                    right: Rc::new(right),
                }),
                span.clone(),
                self.type_table.type_id_boolean(),
            )
        )
    }
}