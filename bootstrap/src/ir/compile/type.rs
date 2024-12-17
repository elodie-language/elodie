use std::ops::Deref;

use crate::frontend::parse::{InfixNode, InfixOperator};
use crate::frontend::parse;
use crate::ir;
use crate::ir::{DeclarePropertyNode, DeclareTypeNode, Identifier};
use crate::ir::compile::Compiler;
use crate::ir::Node::DeclareType;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_declare_type(&mut self, node: &parse::TypeDeclarationNode) -> crate::ir::compile::Result<ir::Node> {
        let mut properties = Vec::with_capacity(node.properties.nodes.len());

        for node in &node.properties.nodes {
            let parse::Node::Infix(InfixNode { left, right, operator }) = node else { panic!() };
            assert!(matches!(operator, InfixOperator::TypeAscription(_)));
            let identifier = left.deref().as_identifier();
            let r#type = self.get_type_id(right.deref().as_type());
            properties.push(
                DeclarePropertyNode {
                    identifier: Identifier::from(identifier),
                    r#type,
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