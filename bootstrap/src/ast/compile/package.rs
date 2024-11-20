use crate::ast;
use crate::ast::{DeclarePackageNode, Identifier, parse};
use crate::ast::compile::Compiler;

impl Compiler {
    pub(crate) fn compile_declare_package(&mut self, node: &parse::PackageDeclarationNode) -> crate::ast::compile::Result<ast::Node> {
        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.compile_node(node)?);
        }

        Ok(ast::Node::DeclarePackage(DeclarePackageNode {
            identifier: Identifier(node.identifier.value().to_string()),
            modifiers: node.modifiers.clone(),
            functions: compiled_body.into_iter()
                .filter_map(|n| {
                    if let ast::Node::DeclareFunction(declare_function) = n {
                        Some(declare_function) // Now directly taking ownership
                    } else {
                        None
                    }
                })
                .collect(),
        }))
    }
}