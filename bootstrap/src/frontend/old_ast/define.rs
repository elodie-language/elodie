use crate::frontend::old_ast::node::{DefineTypeNode, Identifier};
use crate::frontend::old_ast::Generator;
use crate::frontend::{old_ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_define(
        &mut self,
        node: &parse::DefineDeclarationNode,
    ) -> old_ast::Result<old_ast::Node> {
        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.generate_node(node)?);
        }

        Ok(old_ast::Node::DefineType(DefineTypeNode {
            span: node.token.span.clone(),
            identifier: Identifier::from(&node.identifier),
            modifiers: node.modifiers.clone(),
            functions: compiled_body
                .into_iter()
                .filter_map(|n| {
                    if let old_ast::Node::DeclareFunction(declare_function) = n {
                        Some(declare_function)
                    } else {
                        None
                    }
                })
                .collect(),
        }))
    }
}
