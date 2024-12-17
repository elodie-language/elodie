use std::rc::Rc;

use crate::common::DefaultTypeIds;
use crate::frontend::parse;
use crate::ir;
use crate::ir::{DeclareExternalFunctionNode, Identifier, Node};
use crate::ir::compile::Compiler;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_declare_external_function(&mut self, node: &parse::ExternalFunctionDeclarationNode) -> crate::ir::compile::Result<Node> {
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