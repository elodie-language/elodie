use std::rc::Rc;

use crate::common::{Span, TypeId};
use crate::common::node::Node::Block;
use crate::ir::{IrBlockNode, IrTreeNode};
use crate::ir::analyse::TypeBlockNode;
use crate::ir::generate::Generator;

impl<'a> Generator<'a> {
    pub(crate) fn block(&mut self, node: &TypeBlockNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        let mut nodes = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            nodes.push(Rc::new(self.node(node)?));
        }

        Ok(IrTreeNode::new(
            Block(IrBlockNode {
                nodes: nodes.into_boxed_slice()
            }),
            span,
            TypeId::UNIT,
        ))
    }
}