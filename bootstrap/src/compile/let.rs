use std::ops::Deref;

use crate::ir;
use crate::ir::{DeclareVariableNode, Identifier, Node};
use crate::compile::Compiler;
use crate::parse::LetNode;
use crate::r#type::DefaultTypeIds;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_let(&mut self, node: &LetNode) -> crate::compile::Result<ir::Node> {
        let identifier = Identifier::from(&node.identifier);

        Ok(Node::DeclareVariable(DeclareVariableNode {
            identifier,
            value: Box::new(self.compile_node(node.node.deref())?),
            value_type: DefaultTypeIds::string(),
        }))
    }
}