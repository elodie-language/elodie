use std::rc::Rc;

use crate::frontend::{ast, parse};
use crate::frontend::ast::Generator;
use crate::frontend::ast::node::{DeclareExternalFunctionNode, Identifier};

impl<'a> Generator<'a> {
    pub(crate) fn generator_declare_external_function(&mut self, node: &parse::ExternalFunctionDeclarationNode) -> ast::Result<ast::Node> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(Rc::new(self.generator_declare_function_argument(arg)?))
        }

        Ok(ast::Node::DeclareExternalFunction(DeclareExternalFunctionNode {
            identifier: Identifier::from(&node.identifier),
            arguments,
            return_type: None,
        }))
    }
}