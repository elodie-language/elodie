use std::ops::Deref;

use crate::{ast, parse};
use crate::ast::{DeclarePropertyNode, DeclareTypeNode, Identifier};
use crate::ast::Node::DeclareType;
use crate::compile::Compiler;
use crate::parse::{InfixNode, InfixOperator};
use crate::r#type::DefaultTypeIds;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_declare_type(&mut self, node: &parse::TypeDeclarationNode) -> crate::compile::Result<ast::Node> {
        let mut properties = Vec::with_capacity(node.properties.nodes.len());

        for node in &node.properties.nodes {
            let parse::Node::Infix(InfixNode { left, right, operator }) = node else { panic!() };
            assert!(matches!(operator, InfixOperator::TypeAscription(_)));
            let identifier = left.deref().as_identifier();
            let r#type = right.deref().as_type();
            properties.push(
                DeclarePropertyNode {
                    identifier: Identifier::from(identifier),
                    r#type: DefaultTypeIds::never(),
                }
            )
        }

        Ok(DeclareType(DeclareTypeNode {
            identifier: Identifier::from(&node.identifier),
            modifiers: node.modifiers.clone(),
            properties,
        }))
    }
}