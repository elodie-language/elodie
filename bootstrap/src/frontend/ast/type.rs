use std::ops::Deref;

use crate::frontend::{ast, parse};
use crate::frontend::ast::Compiler;
use crate::frontend::ast::node::{DeclarePropertyNode, DeclareTypeNode, Identifier};
use crate::frontend::ast::node::Node::DeclareType;
use crate::frontend::parse::{InfixNode, InfixOperator};

impl<'a> Compiler<'a> {
    pub(crate) fn compile_declare_type(&mut self, node: &parse::TypeDeclarationNode) -> crate::frontend::ast::Result<ast::Node> {
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