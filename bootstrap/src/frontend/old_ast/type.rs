use std::ops::Deref;

use crate::frontend::old_ast::node::Node::DeclareType;
use crate::frontend::old_ast::node::{DeclarePropertyNode, DeclareTypeNode, Identifier};
use crate::frontend::old_ast::{ObjectTypeNode, Generator, TypeFunctionNode};
use crate::frontend::parse::{InfixNode, InfixOperator, TypeNode};
use crate::frontend::{old_ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn declare_type(
        &mut self,
        node: &parse::TypeDeclarationNode,
    ) -> crate::frontend::old_ast::Result<old_ast::Node> {
        let mut properties = Vec::with_capacity(node.properties.nodes.len());

        for node in &node.properties.nodes {
            let parse::Node::Infix(InfixNode {
                left,
                right,
                operator,
                token,
            }) = node
            else {
                panic!()
            };
            assert!(matches!(operator, InfixOperator::TypeAscription(_)));
            let identifier = left.deref().as_identifier();
            let r#type = self.handle_type_node(right.deref().as_type())?;
            properties.push(DeclarePropertyNode {
                span: node.token().span.clone(),
                identifier: Identifier::from(identifier),
                r#type,
            })
        }

        Ok(DeclareType(DeclareTypeNode {
            span: node.token.span.clone(),
            identifier: Identifier::from(&node.identifier),
            modifiers: node.modifiers.clone(),
            properties,
        }))
    }

    pub(crate) fn handle_type_node(
        &mut self,
        node: &parse::TypeNode,
    ) -> old_ast::Result<old_ast::TypeNode> {
        match node {
            TypeNode::Boolean(t) => Ok(old_ast::TypeNode::Boolean(t.clone())),
            TypeNode::Object(node) => Ok(old_ast::TypeNode::Object(ObjectTypeNode {
                span: node.token.span.clone(),
            })),
            TypeNode::Number(t) => Ok(old_ast::TypeNode::Number(t.clone())),
            TypeNode::String(t) => Ok(old_ast::TypeNode::String(t.clone())),
            TypeNode::Function(node) => {
                let return_type = if let Some(type_node) = node.return_type.as_deref() {
                    Some(Box::new(self.handle_type_node(type_node)?))
                } else {
                    None
                };

                Ok(old_ast::TypeNode::Function(TypeFunctionNode {
                    arguments: vec![],
                    return_type,
                }))
            }
        }
    }
}
