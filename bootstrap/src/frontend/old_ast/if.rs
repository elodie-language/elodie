use std::ops::Deref;

use crate::frontend::old_ast::node::{BlockNode, IfNode};
use crate::frontend::old_ast::Generator;
use crate::frontend::{old_ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_if(&mut self, node: &parse::IfNode) -> old_ast::Result<old_ast::Node> {
        // condition needs to be of type boolean --> every node has a type?!
        let condition = Box::new(self.generate_node(node.condition.deref())?);

        let mut then_body = vec![];
        for node in &node.then.nodes {
            then_body.push(self.generate_node(node.deref())?)
        }

        let otherwise = if node.otherwise.is_some() {
            let mut otherwise_body = vec![];
            for node in &node.otherwise.as_ref().unwrap().block.nodes {
                otherwise_body.push(self.generate_node(node)?)
            }
            Some(BlockNode {
                body: otherwise_body,
            })
        } else {
            None
        };

        Ok(old_ast::Node::If(IfNode {
            span: node.token.span.clone(),
            condition,
            then: BlockNode { body: then_body },
            otherwise,
        }))
    }
}
