use std::ops::Deref;

use crate::{ir};
use crate::ir::compile::Compiler;
use crate::ir::{BlockNode, IfNode};
use crate::common::DefaultTypeIds;
use crate::frontend::parse;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_if(&mut self, node: &parse::IfNode) -> crate::ir::compile::Result<ir::Node> {
        // condition needs to be of type boolean --> every node has a type?!
        let condition = Box::new(self.compile_node(node.condition.deref())?);

        let mut then_body = vec![];
        for node in &node.then.nodes {
            then_body.push(self.compile_node(node.deref())?)
        }

        let otherwise = if node.otherwise.is_some() {
            let mut otherwise_body = vec![];
            for node in &node.otherwise.as_ref().unwrap().block.nodes {
                otherwise_body.push(self.compile_node(node)?)
            }
            Some(BlockNode { body: otherwise_body, return_type: DefaultTypeIds::unit() })
        } else {
            None
        };

        Ok(ir::Node::If(
            IfNode {
                condition,
                then: BlockNode { body: then_body, return_type: DefaultTypeIds::unit() },
                otherwise,
                return_type: DefaultTypeIds::unit(),
            }
        ))
    }
}