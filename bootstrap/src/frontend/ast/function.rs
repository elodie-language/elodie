use std::ops::Deref;
use std::rc::Rc;

use crate::common::node::Node;
use crate::common::node::Node::ReturnFromFunction;
use crate::frontend::{ast, parse};
use crate::frontend::ast::{AstBlockNode, AstDeclareExternalFunctionNode, AstDeclareFunctionNode, AstFunctionArgument, AstIdentifier, AstReturnFromFunctionNode, AstTreeNode, Generator, SPAN_NOT_IMPLEMENTED};

impl<'a> Generator<'a> {
    pub(crate) fn generate_declare_external_function(
        &mut self,
        node: &parse::ExternalFunctionDeclarationNode,
    ) -> ast::Result<AstTreeNode> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(self.generate_declare_function_argument(arg)?)
        }

        Ok(AstTreeNode::new(Node::DeclareExternalFunction(
            AstDeclareExternalFunctionNode {
                function: AstIdentifier(node.identifier.value()),
                arguments,
                return_type: None,
            },
        ), SPAN_NOT_IMPLEMENTED.clone()))
    }


    pub(crate) fn generate_declare_function(
        &mut self,
        node: &parse::FunctionDeclarationNode,
    ) -> ast::Result<AstTreeNode> {
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

        Ok(AstTreeNode::new(Node::DeclareFunction(AstDeclareFunctionNode {
            function: AstIdentifier(node.identifier.value()),
            arguments,
            return_type,
            nodes: Rc::new(AstBlockNode { nodes }),
        }), SPAN_NOT_IMPLEMENTED.clone()))
    }

    pub(crate) fn generate_declare_function_argument(
        &mut self,
        node: &parse::FunctionDeclarationArgumentNode,
    ) -> ast::Result<ast::AstFunctionArgument> {
        let argument_type = if let Some(type_node) = node.r#type.as_deref() {
            Some(self.to_ast_type(type_node))
        } else {
            None
        };

        Ok(AstFunctionArgument {
            argument: AstIdentifier(node.identifier.value()),
            argument_type,
        })
    }

    pub(crate) fn generate_function_return(
        &mut self,
        node: &parse::ReturnNode,
    ) -> ast::Result<AstTreeNode> {
        let node = if let Some(ref node) = node.result {
            Some(Rc::new(self.generate_node(node.deref())?))
        } else {
            None
        };

        Ok(AstTreeNode::new(ReturnFromFunction(AstReturnFromFunctionNode {
            node
        }), SPAN_NOT_IMPLEMENTED.clone()))
    }
}
