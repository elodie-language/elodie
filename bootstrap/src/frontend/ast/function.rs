use std::ops::Deref;
use std::rc::Rc;

use crate::frontend::ast::node::Node::ReturnFromFunction;
use crate::frontend::ast::node::{
    BlockNode, DeclareFunctionNode, Identifier, Node, ReturnFromFunctionNode,
};
use crate::frontend::ast::{FunctionArgumentNode, Generator};
use crate::frontend::{ast, parse};

impl<'a> Generator<'a> {
    pub(crate) fn generate_declare_function(
        &mut self,
        node: &parse::FunctionDeclarationNode,
    ) -> ast::Result<ast::Node> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(Rc::new(self.generate_declare_function_argument(arg)?))
        }

        let mut body = vec![];
        for node in &node.block.nodes {
            body.push(self.generate_node(node)?)
        }

        let return_type = if let Some(type_node) = node.return_type.as_deref() {
            Some(self.handle_type_node(type_node)?)
        } else {
            None
        };

        Ok(ast::Node::DeclareFunction(DeclareFunctionNode {
            span: node.token.span.clone(),
            identifier: Identifier::from(&node.identifier),
            arguments,
            return_type,
            body: Rc::new(BlockNode { body }),
        }))
    }

    pub(crate) fn generate_declare_function_argument(
        &mut self,
        node: &parse::FunctionDeclarationArgumentNode,
    ) -> ast::Result<ast::FunctionArgumentNode> {
        let ty = if let Some(type_node) = node.r#type.as_deref() {
            Some(self.handle_type_node(type_node)?)
        } else {
            None
        };

        Ok(FunctionArgumentNode {
            identifier: Identifier::from(&node.identifier),
            ty,
        })
    }

    pub(crate) fn generate_function_return(
        &mut self,
        node: &parse::ReturnNode,
    ) -> ast::Result<ast::Node> {
        let result = if let Some(ref node) = node.result {
            self.generate_node(node.deref())?
        } else {
            Node::Unit
        };

        Ok(ReturnFromFunction(ReturnFromFunctionNode {
            span: node.token.span.clone(),
            node: Box::new(result),
            return_type: None,
        }))
    }
}
