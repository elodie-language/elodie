use std::ops::Deref;
use std::rc::Rc;

use crate::common::{Span, TypeId};
use crate::common::node::Node::If;
use crate::ir::{IrBlockNode, IrIfNode, IrTreeNode};
use crate::ir::analyse::TypeIfNode;
use crate::ir::generate::Generator;

impl<'a> Generator<'a> {
    pub(crate) fn r#if(&mut self, node: &TypeIfNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        let condition = Rc::new(self.node(node.condition.deref())?);

        let mut then_body = vec![];
        for node in &node.then.borrow().nodes {
            then_body.push(Rc::new(self.node(node.deref())?))
        }

        let otherwise = if node.otherwise.is_some() {
            let mut otherwise_body = vec![];
            for node in &node.otherwise.as_ref().unwrap().borrow().nodes {
                otherwise_body.push(Rc::new(self.node(node)?))
            }
            Some(Rc::new(IrBlockNode {
                nodes: otherwise_body.into_boxed_slice(),
            }))
        } else {
            None
        };

        Ok(IrTreeNode::new(
            If(IrIfNode {
                condition,
                then: Rc::new(IrBlockNode { nodes: then_body.into_boxed_slice() }),
                otherwise,
            }),
            span,
            TypeId::UNIT,
        )
        )
    }
}