use std::rc::Rc;

use crate::frontend::{ast, parse};
use crate::frontend::ast::Compiler;
use crate::frontend::ast::node::{DeclareExternalFunctionNode, Identifier};

impl<'a> Compiler<'a> {
    pub(crate) fn compile_declare_external_function(&mut self, node: &parse::ExternalFunctionDeclarationNode) -> ast::Result<ast::Node> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(Rc::new(self.compile_declare_function_argument(arg)?))
        }

        Ok(ast::Node::DeclareExternalFunction(DeclareExternalFunctionNode {
            identifier: Identifier::from(&node.identifier),
            arguments,
            return_type: None,
        }))
    }
}