use std::ops::Deref;
use std::rc::Rc;

use crate::frontend::{ast, parse};
use crate::frontend::ast::{FunctionArgument, Generator, SPAN_NOT_IMPLEMENTED};
use crate::frontend::ast::DeclareExternalFunctionNode;
use crate::frontend::ast::node::{AstNode, BlockNode, DeclareFunctionNode, Identifier, Node, ReturnFromFunctionNode};
use crate::frontend::ast::node::Node::ReturnFromFunction;

impl<'a> Generator<'a> {
    pub(crate) fn generate_declare_external_function(
        &mut self,
        node: &parse::ExternalFunctionDeclarationNode,
    ) -> ast::Result<AstNode> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(self.generate_declare_function_argument(arg)?)
        }

        Ok(AstNode::new(ast::Node::DeclareExternalFunction(
            DeclareExternalFunctionNode {
                function: Identifier(node.identifier.value()),
                arguments,
                return_type: None,
            },
        ), SPAN_NOT_IMPLEMENTED.clone()))
    }


    pub(crate) fn generate_declare_function(
        &mut self,
        node: &parse::FunctionDeclarationNode,
    ) -> ast::Result<AstNode> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(self.generate_declare_function_argument(arg)?)
        }

        let mut nodes = vec![];
        for node in &node.block.nodes {
            nodes.push(self.generate_node(node)?)
        }

        let return_type = if let Some(type_node) = node.return_type.as_deref() {
            Some(self.to_ast_type(type_node))
        } else {
            None
        };

        Ok(AstNode::new(Node::DeclareFunction(DeclareFunctionNode {
            function: Identifier(node.identifier.value()),
            arguments,
            return_type,
            nodes: Rc::new(BlockNode { nodes }),
        }), SPAN_NOT_IMPLEMENTED.clone()))
    }

    pub(crate) fn generate_declare_function_argument(
        &mut self,
        node: &parse::FunctionDeclarationArgumentNode,
    ) -> ast::Result<ast::FunctionArgument> {
        let argument_type = if let Some(type_node) = node.r#type.as_deref() {
            Some(self.to_ast_type(type_node))
        } else {
            None
        };

        Ok(FunctionArgument {
            argument: Identifier(node.identifier.value()),
            argument_type,
        })
    }

    pub(crate) fn generate_function_return(
        &mut self,
        node: &parse::ReturnNode,
    ) -> ast::Result<AstNode> {
        let node = if let Some(ref node) = node.result {
            Some(Rc::new(self.generate_node(node.deref())?))
        } else {
            None
        };

        Ok(AstNode::new(ReturnFromFunction(ReturnFromFunctionNode {
            node
        }), SPAN_NOT_IMPLEMENTED.clone()))
    }
}
