use std::ops::Deref;

use crate::ir::compile::Compiler;
use crate::ir;
use crate::ir::{DeclareVariableNode, Identifier, Node};
use crate::frontend::parse::LetNode;
use crate::common::DefaultTypeIds;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_let(&mut self, node: &LetNode) -> crate::ir::compile::Result<ir::Node> {
        let identifier = Identifier::from(&node.identifier);
        let value = self.compile_node(node.node.deref())?;

        let value_type = if let Node::Literal(node) = &value {
            node.ty()
        } else {
            DefaultTypeIds::never()
        };

        self.scope.insert_identifier(identifier.clone(), value_type);

        Ok(Node::DeclareVariable(DeclareVariableNode {
            identifier,
            value: Box::new(value),
            value_type,
        }))
    }
}