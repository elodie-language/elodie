use crate::frontend::parse;
use crate::ir;
use crate::ir::{DefineTypeNode, Identifier};
use crate::ir::compile::Compiler;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_define(&mut self, node: &parse::DefineDeclarationNode) -> crate::ir::compile::Result<ir::Node> {
        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.compile_node(node)?);
        }

        Ok(ir::Node::DefineType(DefineTypeNode {
            identifier: Identifier::from(&node.identifier),
            modifiers: node.modifiers.clone(),
            functions: compiled_body.into_iter()
                .filter_map(|n| {
                    if let ir::Node::DeclareFunction(declare_function) = n {
                        Some(declare_function)
                    } else {
                        None
                    }
                })
                .collect(),
        }))
    }
}