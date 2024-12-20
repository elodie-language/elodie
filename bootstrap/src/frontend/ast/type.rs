use std::ops::Deref;

use crate::frontend::{ast, parse};
use crate::frontend::ast::{Ast, AstType, DefineTypeNode, Generator, Identifier, Node, SPAN_NOT_IMPLEMENTED};
use crate::frontend::ast::node::AstNode;
use crate::frontend::parse::{TypeFunctionNode, TypeNode};

impl<'a> Generator<'a> {
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
    ) -> ast::Result<AstNode> {

        let mut compiled_body = vec![];

        for node in &node.block.nodes {
            compiled_body.push(self.generate_node(node)?);
        }

        Ok(AstNode::new(Node::DefineType(DefineTypeNode {
            r#type: Identifier(node.identifier.0.clone()),
            modifiers: node.modifiers.clone(),
            functions: compiled_body
                .into_iter()
                .filter_map(|n| {
                    if let Node::DeclareFunction(declare_function) = n.node_to_owned() {
                        Some(declare_function)
                    } else {
                        None
                    }
                })
                .collect(),
        }), SPAN_NOT_IMPLEMENTED.clone()))
    }
}
