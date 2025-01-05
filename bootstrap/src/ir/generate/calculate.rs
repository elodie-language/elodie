use std::rc::Rc;

use crate::common::{Span, TypeId};
use crate::common::node::Node;
use crate::ir::{IrCalculateNode, IrTreeNode};
use crate::ir::analyse::TypeCalculateNode;
use crate::ir::generate::Generator;

impl<'a> Generator<'a> {
    pub(crate) fn calculate(&mut self, node: &TypeCalculateNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        let left = self.node(node.left.as_ref())?;
        let right = self.node(node.right.as_ref())?;

        Ok(
            IrTreeNode::new(
                Node::Calculate(IrCalculateNode {
                    left: Rc::new(left),
                    operator: node.operator.clone(),
                    right: Rc::new(right),
                }),
                span.clone(),
                TypeId::BOOLEAN,
            )
        )
    }
}