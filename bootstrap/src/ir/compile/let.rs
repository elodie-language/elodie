use std::ops::Deref;

use crate::common::DefaultTypeIds;
use crate::frontend::parse;
use crate::ir;
use crate::ir::{DeclareVariableNode, Identifier, Node};
use crate::ir::compile::Compiler;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_let(&mut self, node: &parse::DeclareVariableNode) -> crate::ir::compile::Result<ir::Node> {
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