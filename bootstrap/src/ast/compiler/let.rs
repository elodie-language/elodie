use std::ops::Deref;

use crate::ast::{ast, DeclareVariableNode, Identifier, Node};
use crate::ast::compiler::Compiler;
use crate::ast::parse::LetNode;
use crate::ast::r#type::DefaultTypeIds;

impl Compiler {
    pub(crate) fn compile_let(&mut self, node: &LetNode) -> crate::ast::compiler::Result<ast::Node> {
        let identifier = Identifier(node.identifier.0.value().to_string());

        Ok(Node::DeclareVariable(DeclareVariableNode {
            identifier,
            value: Box::new(self.compile_node(node.node.deref())?),
            value_type: DefaultTypeIds::string(),
        }))
    }
}