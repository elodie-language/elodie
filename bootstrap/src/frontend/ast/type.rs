use std::ops::Deref;

use crate::common::node::Node;
use crate::common::node::Node::{DeclareType, DefineType};
use crate::frontend::ast::{
    AstDeclareTypeNode, AstDefineTypeNode, AstIdentifier, AstTreeNode, AstType, Generator,
    TypeVariable, SPAN_NOT_IMPLEMENTED,
};
use crate::frontend::parse::{InfixNode, InfixOperator, TypeFunctionNode, TypeNode};
use crate::frontend::{ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_declare_type(
        &mut self,
        node: &parse::TypeDeclarationNode,
    ) -> ast::Result<AstTreeNode> {
        let mut variables = Vec::with_capacity(node.properties.nodes.len());
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
            let r#type = self.to_ast_type(right.deref().as_type());
            variables.push(TypeVariable {
                variable: AstIdentifier(identifier.value()),
                r#type,
            })
        }

        Ok(AstTreeNode::new(
            DeclareType(AstDeclareTypeNode {
                r#type: AstIdentifier(node.identifier.value()),
                modifiers: node.modifiers.clone(),
                variables,
            }),
            SPAN_NOT_IMPLEMENTED.clone(),
        ))
    }

    pub(crate) fn to_ast_type(&self, node: &parse::TypeNode) -> AstType {
        match node {
            TypeNode::Boolean(_) => AstType::Boolean,
            TypeNode::Number(_) => AstType::Number,
            TypeNode::String(_) => AstType::String,
            TypeNode::Function(TypeFunctionNode {
                token,
                arguments,
                return_type,
                ..
            }) => AstType::Function {
                function: AstIdentifier(token.value),
                arguments: arguments
                    .iter()
                    .map(|a| self.to_ast_type(a.r#type.deref()))
                    .collect::<Vec<_>>(),

                return_type: return_type
                    .as_ref()
                    .map(|r| Box::new(self.to_ast_type(r.deref()))),
            },
            TypeNode::Type(token) => AstType::Type {
                r#type: AstIdentifier(token.value())
            },
        }
    }

    pub(crate) fn generate_define_type(
        &mut self,
        node: &parse::DefineDeclarationNode,
    ) -> ast::Result<AstTreeNode> {
        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.generate_node(node)?);
        }

        Ok(AstTreeNode::new(
            DefineType(AstDefineTypeNode {
                r#type: AstIdentifier(node.identifier.value()),
                modifiers: node.modifiers.clone(),
                functions: compiled_body
                    .into_iter()
                    .filter_map(|n| {
                        if let Node::DeclareFunction(declare_function) = n.node_to_owned() {
                            Some(declare_function.clone())
                        } else {
                            None
                        }
                    })
                    .collect(),
            }),
            SPAN_NOT_IMPLEMENTED.clone(),
        ))
    }
}
