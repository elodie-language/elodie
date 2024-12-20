use std::rc::Rc;

use crate::frontend::old_ast::node::{DeclareExternalFunctionNode, Identifier};
use crate::frontend::old_ast::Generator;
use crate::frontend::{old_ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_declare_external_function(
        &mut self,
        node: &parse::ExternalFunctionDeclarationNode,
    ) -> old_ast::Result<old_ast::Node> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(Rc::new(self.generate_declare_function_argument(arg)?))
        }

        Ok(old_ast::Node::DeclareExternalFunction(
            DeclareExternalFunctionNode {
                span: node.token.span.clone(),
                identifier: Identifier::from(&node.identifier),
                arguments,
                return_type: None,
            },
        ))
    }
}
