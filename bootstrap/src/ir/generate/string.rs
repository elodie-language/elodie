use std::rc::Rc;

use crate::common::{Span, TypeId};
use crate::common::node::Node;
use crate::common::node::Node::InterpolateString;
use crate::ir::{IrInterpolateStringNode, IrTreeNode};
use crate::ir::analyse::TypeInterpolateStringNode;
use crate::ir::generate::Generator;

impl<'a> Generator<'a> {
    pub(crate) fn interpolate_string(&mut self, node: &TypeInterpolateStringNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        let mut nodes = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            nodes.push(Rc::new(self.node(node)?))
        }

        Ok(IrTreeNode::new(
            InterpolateString(IrInterpolateStringNode { nodes: nodes.into_boxed_slice() }),
            span,
            TypeId::STRING,
        ))
    }
}