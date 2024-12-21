use std::ops::Deref;
use std::rc::Rc;
use crate::common::tree::Node::{DeclareType, DefineType};
use crate::common::tree::{Node, Tree, TreeNode};

use crate::frontend::{ast, parse};
use crate::frontend::ast::{AstType, Generator, AstIdentifier, SPAN_NOT_IMPLEMENTED, TypeVariable, AstDeclareTypeNode, AstDefineTypeNode};
use crate::frontend::ast::node::{AstVariant};
use crate::frontend::parse::{InfixNode, InfixOperator, TypeFunctionNode, TypeNode};

impl<'a> Generator<'a> {

    pub(crate) fn generate_declare_type(
        &mut self,
        node: &parse::TypeDeclarationNode,
    ) -> ast::Result<TreeNode<AstVariant>> {

        let mut variables = Vec::with_capacity(node.properties.nodes.len());
        for node in &node.properties.nodes {
            let parse::Node::Infix(InfixNode { left, right, operator, token, }) = node else { panic!() };
            assert!(matches!(operator, InfixOperator::TypeAscription(_)));
            let identifier = left.deref().as_identifier();
            let r#type = self.to_ast_type(right.deref().as_type());
            variables.push(TypeVariable {
                variable: AstIdentifier(identifier.value()),
                r#type,
            })
        }

        Ok(TreeNode::new(DeclareType(Rc::new(AstDeclareTypeNode {
            r#type: AstIdentifier(node.identifier.value()),
            modifiers: node.modifiers.clone(),
            variables,
        })), SPAN_NOT_IMPLEMENTED.clone()))
    }


    pub(crate) fn to_ast_type(&self, node: &parse::TypeNode) -> AstType {
        match node {
            TypeNode::Boolean(_) => AstType::Boolean,
            TypeNode::Object(_) => AstType::Object,
            TypeNode::Number(_) => AstType::Number,
            TypeNode::String(_) => AstType::String,
            TypeNode::Function(TypeFunctionNode { arguments, return_type, .. }) => AstType::Function {
                arguments: arguments.iter().map(|a| Box::new(self.to_ast_type(a.r#type.deref()))).collect::<Vec<_>>(),
                return_type: return_type.as_ref().map(|r| Box::new(self.to_ast_type(r.deref()))),
            }
        }
    }

    pub(crate) fn generate_define_type(
        &mut self,
        node: &parse::DefineDeclarationNode,
    ) -> ast::Result<TreeNode<AstVariant>> {
        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.generate_node(node)?);
        }

        // Ok(TreeNode::new(DefineType(Rc::new(AstDefineTypeNode {
        //     r#type: AstIdentifier(node.identifier.value()),
        //     modifiers: node.modifiers.clone(),
        //     functions: compiled_body
        //         .into_iter()
        //         .filter_map(|n| {
        //             if let Node::DeclareFunction(declare_function) = n.node_to_owned() {
        //                 Some(declare_function)
        //             } else {
        //                 None
        //             }
        //         })
        //         .collect(),
        // })), SPAN_NOT_IMPLEMENTED.clone()))
        todo!()
    }
}
