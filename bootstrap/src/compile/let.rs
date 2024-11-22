use std::ops::Deref;

use crate::ast;
use crate::ast::{DeclareVariableNode, Identifier, Node};
use crate::ast::r#type::DefaultTypeIds;
use crate::compile::Compiler;
use crate::parse::LetNode;

impl Compiler {
    pub(crate) fn compile_let(&mut self, node: &LetNode) -> crate::compile::Result<ast::Node> {
        let identifier = Identifier(node.identifier.0.value().to_string());

        Ok(Node::DeclareVariable(DeclareVariableNode {
            identifier,
            value: Box::new(self.compile_node(node.node.deref())?),
            value_type: DefaultTypeIds::string(),
        }))
    }
}