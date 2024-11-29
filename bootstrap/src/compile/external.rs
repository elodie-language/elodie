use std::rc::Rc;

use crate::{ir, parse};
use crate::compile::Compiler;
use crate::ir::{DeclareExternalFunctionNode, Identifier, Node};
use crate::r#type::DefaultTypeIds;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_declare_external_function(&mut self, node: &parse::ExternalFunctionDeclarationNode) -> crate::compile::Result<Node> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(Rc::new(self.compile_declare_function_argument(arg)?))
        }

        Ok(ir::Node::DeclareExternalFunction(DeclareExternalFunctionNode {
            identifier: Identifier::from(&node.identifier),
            arguments,
            return_type: DefaultTypeIds::never(),
        }))
    }
}