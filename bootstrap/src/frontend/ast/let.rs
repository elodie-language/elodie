use std::ops::Deref;

use crate::common::DefaultTypeIds;
use crate::frontend::{ast, parse};
use crate::frontend::ast::Compiler;
use crate::frontend::ast::node::{DeclareVariableNode, Identifier, Node};

impl<'a> Compiler<'a> {
    pub(crate) fn compile_let(&mut self, node: &parse::VariableDeclarationNode) -> ast::Result<ast::Node> {
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