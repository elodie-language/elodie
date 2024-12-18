use std::ops::Deref;
use std::rc::Rc;

use crate::frontend::{ast, parse};
use crate::frontend::ast::{Generator, FunctionArgumentNode};
use crate::frontend::ast::node::{BlockNode, DeclareFunctionNode, Identifier, Node, ReturnFromFunctionNode};
use crate::frontend::ast::node::Node::ReturnFromFunction;

impl<'a> Generator<'a> {
    pub(crate) fn generator_declare_function(&mut self, node: &parse::FunctionDeclarationNode) -> ast::Result<ast::Node> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(Rc::new(self.generator_declare_function_argument(arg)?))
        }

        let mut body = vec![];
        for node in &node.block.nodes {
            body.push(self.generator_node(node)?)
        }

        let return_type = if let Some(type_node) = node.return_type.as_deref() {
            Some(self.handle_type_node(type_node)?)
        } else {
            None
        };

        Ok(ast::Node::DeclareFunction(DeclareFunctionNode {
            identifier: Identifier::from(&node.identifier),
            arguments,
            return_type,
            body: Rc::new(BlockNode { body }),
        }))
    }

    pub(crate) fn generator_declare_function_argument(&mut self, node: &parse::FunctionDeclarationArgumentNode) -> ast::Result<ast::FunctionArgumentNode> {
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

    pub(crate) fn generator_function_return(&mut self, node: &parse::ReturnNode) -> ast::Result<ast::Node> {
        let result = if let Some(ref node) = node.result {
            self.generator_node(node.deref())?
        } else {
            Node::Unit
        };

        Ok(ReturnFromFunction(ReturnFromFunctionNode {
            node: Box::new(result),
            return_type: None,
        }))
    }
}