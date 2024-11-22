use std::ops::Deref;

use crate::ast;
use crate::ast::{DeclarePropertyNode, DeclareTypeNode, Identifier, parse};
use crate::ast::compile::Compiler;
use crate::ast::Node::DeclareType;
use crate::parse::{InfixNode, InfixOperator};
use crate::ast::r#type::DefaultTypeIds;

impl Compiler {
    pub(crate) fn compile_declare_type(&mut self, node: &parse::TypeDeclarationNode) -> crate::ast::compile::Result<ast::Node> {
        let mut properties = Vec::with_capacity(node.properties.nodes.len());

        for node in &node.properties.nodes {
            let parse::Node::Infix(InfixNode { left, right, operator }) = node else { panic!() };
            assert!(matches!(operator, InfixOperator::TypeAscription(_)));
            let identifier = left.deref().as_identifier();
            let r#type = right.deref().as_type();
            properties.push(
                DeclarePropertyNode {
                    identifier: Identifier(identifier.value().to_string()),
                    r#type: DefaultTypeIds::never(),
                }
            )
        }

        Ok(DeclareType(DeclareTypeNode {
            identifier: Identifier(node.identifier.value().to_string()),
            modifiers: node.modifiers.clone(),
            properties,
        }))
    }
}