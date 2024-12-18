use crate::frontend::{ast, parse};
use crate::frontend::ast::Generator;
use crate::frontend::ast::node::{DefineTypeNode, Identifier};

impl<'a> Generator<'a> {
    pub(crate) fn generator_define(&mut self, node: &parse::DefineDeclarationNode) -> ast::Result<ast::Node> {
        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.generator_node(node)?);
        }

        Ok(ast::Node::DefineType(DefineTypeNode {
            identifier: Identifier::from(&node.identifier),
            modifiers: node.modifiers.clone(),
            functions: compiled_body.into_iter()
                .filter_map(|n| {
                    if let ast::Node::DeclareFunction(declare_function) = n {
                        Some(declare_function)
                    } else {
                        None
                    }
                })
                .collect(),
        }))
    }
}