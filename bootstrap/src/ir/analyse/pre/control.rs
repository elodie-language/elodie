use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::common::Inferred;
use crate::common::node::Node::If;
use crate::frontend::ast::AstIfNode;
use crate::ir::analyse::{TypeBlockNode, TypedTreeNode, TypeIfNode};
use crate::ir::analyse::pre::Pre;

impl<'a> Pre<'a> {
    pub(crate) fn r#if(&mut self, node: &AstIfNode) -> crate::ir::analyse::Result<TypedTreeNode> {
        let condition = Box::new(self.node(node.condition.deref())?);

        self.scope.enter();
        let mut then_body = vec![];
        for node in &node.then.nodes {
            then_body.push(self.node(node.deref())?)
        }
        self.scope.leave();

        self.scope.enter();
        let otherwise = if node.otherwise.is_some() {
            let mut otherwise_body = vec![];
            for node in &node.otherwise.as_ref().unwrap().nodes {
                otherwise_body.push(self.node(node)?)
            }
            Some(RefCell::new(TypeBlockNode {
                nodes: otherwise_body.into_boxed_slice(),
            }))
        } else {
            None
        };
        self.scope.leave();

        Ok(TypedTreeNode::new(
            If(TypeIfNode {
                condition,
                then: RefCell::new(TypeBlockNode { nodes: then_body.into_boxed_slice() }),
                otherwise,
            }),
            self.span(),
            Inferred::Unknown,
        ))
    }
}