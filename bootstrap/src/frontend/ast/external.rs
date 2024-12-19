use std::rc::Rc;

use crate::frontend::ast::node::{DeclareExternalFunctionNode, Identifier};
use crate::frontend::ast::Generator;
use crate::frontend::{ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_declare_external_function(
        &mut self,
        node: &parse::ExternalFunctionDeclarationNode,
    ) -> ast::Result<ast::Node> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(Rc::new(self.generate_declare_function_argument(arg)?))
        }

        Ok(ast::Node::DeclareExternalFunction(
            DeclareExternalFunctionNode {
                span: node.token.span.clone(),
                identifier: Identifier::from(&node.identifier),
                arguments,
                return_type: None,
            },
        ))
    }
}
