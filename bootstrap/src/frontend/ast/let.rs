use std::ops::Deref;
use std::rc::Rc;

use crate::frontend::ast::node::{DeclareVariableNode, Identifier, Node};
use crate::frontend::ast::Generator;
use crate::frontend::{ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_let(
        &mut self,
        node: &parse::VariableDeclarationNode,
    ) -> ast::Result<ast::Node> {
        let identifier = Identifier::from(&node.identifier);
        let value = self.generate_node(node.node.deref())?;

        // self.scope.insert_identifier(identifier.clone());

        let value_type = if let Some(type_node) = node.r#type.as_ref() {
            Some(self.handle_type_node(type_node)?)
        } else {
            None
        };

        Ok(Node::DeclareVariable(DeclareVariableNode {
            span: node.token.span.clone(),
            identifier,
            value: Rc::new(value),
            value_type,
        }))
    }
}
