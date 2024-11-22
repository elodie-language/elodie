use std::ops::Deref;

use crate::{ast, parse};
use crate::ast::{BlockNode, IfNode};
use crate::r#type::DefaultTypeIds;
use crate::compile::Compiler;

impl Compiler {
    pub(crate) fn compile_if(&mut self, node: &parse::IfNode) -> crate::compile::Result<ast::Node> {
        // condition needs to be of type boolean --> every node has a type?!
        let condition = Box::new(self.compile_node(node.condition.deref())?);

        let mut then_body = vec![];
        for node in &node.then.nodes {
            then_body.push(self.compile_node(node.deref())?)
        }

        let mut otherwise_body = vec![];
        if node.otherwise.is_some() {
            for node in &node.otherwise.as_ref().unwrap().block.nodes {
                otherwise_body.push(self.compile_node(node)?)
            }
        }

        Ok(ast::Node::If(
            IfNode {
                condition,
                then: BlockNode { body: then_body, return_type: DefaultTypeIds::unit() },
                otherwise: BlockNode { body: otherwise_body, return_type: DefaultTypeIds::unit() },
                return_type: DefaultTypeIds::unit(),
            }
        ))
    }
}