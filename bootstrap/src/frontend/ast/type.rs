use std::ops::Deref;

use crate::frontend::{ast, parse};
use crate::frontend::ast::{Generator, CustomTypeNode, TypeFunctionNode};
use crate::frontend::ast::node::{DeclarePropertyNode, DeclareTypeNode, Identifier};
use crate::frontend::ast::node::Node::DeclareType;
use crate::frontend::parse::{InfixNode, InfixOperator, TypeNode};

impl<'a> Generator<'a> {
    pub(crate) fn declare_type(&mut self, node: &parse::TypeDeclarationNode) -> crate::frontend::ast::Result<ast::Node> {
        let mut properties = Vec::with_capacity(node.properties.nodes.len());

        for node in &node.properties.nodes {
            let parse::Node::Infix(InfixNode { left, right, operator }) = node else { panic!() };
            assert!(matches!(operator, InfixOperator::TypeAscription(_)));
            let identifier = left.deref().as_identifier();
            let r#type = self.handle_type_node(right.deref().as_type())?;
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

    pub(crate) fn handle_type_node(&mut self, node: &parse::TypeNode) -> ast::Result<ast::TypeNode> {
        match node {
            TypeNode::Boolean(t) => Ok(ast::TypeNode::Boolean(t.clone())),
            TypeNode::Custom(node) => Ok(ast::TypeNode::Custom(CustomTypeNode { token: node.token.clone() })),
            TypeNode::Number(t) => Ok(ast::TypeNode::Number(t.clone())),
            TypeNode::String(t) => Ok(ast::TypeNode::String(t.clone())),
            TypeNode::Function(node) => {
                let return_type = if let Some(type_node) = node.return_type.as_deref() {
                    Some(Box::new(self.handle_type_node(type_node)?))
                } else {
                    None
                };

                Ok(ast::TypeNode::Function(TypeFunctionNode {
                    arguments: vec![],
                    return_type,
                }))
            }
        }
    }
}