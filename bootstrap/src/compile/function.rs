use std::ops::Deref;
use std::rc::Rc;

use crate::{ast, parse};
use crate::ast::{BlockNode, DeclareFunctionNode, FunctionArgumentNode, Identifier, Node, ReturnFromFunctionNode};
use crate::ast::Node::ReturnFromFunction;
use crate::compile::Compiler;
use crate::r#type::DefaultTypeIds;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_declare_function(&mut self, node: &parse::FunctionDeclarationNode) -> crate::compile::Result<ast::Node> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(Rc::new(self.compile_declare_function_argument(arg)?))
        }

        let mut body = vec![];
        for node in &node.block.nodes {
            body.push(self.compile_node(node)?)
        }

        Ok(ast::Node::DeclareFunction(DeclareFunctionNode {
            identifier: Identifier::from(&node.identifier),
            arguments,
            return_type: DefaultTypeIds::never(),
            body: Rc::new(BlockNode { body, return_type: DefaultTypeIds::never() }),
        }))
    }

    pub(crate) fn compile_declare_function_argument(&mut self, node: &parse::FunctionDeclarationArgumentNode) -> crate::compile::Result<ast::FunctionArgumentNode> {
        Ok(FunctionArgumentNode {
            identifier: Identifier::from(&node.identifier),
            type_id: DefaultTypeIds::never(),
        })
    }

    pub(crate) fn compile_function_return(&mut self, node: &parse::ReturnNode) -> crate::compile::Result<ast::Node> {
        let result = if let Some(ref node) = node.result {
            self.compile_node(node.deref())?
        } else {
            Node::ValueUnit
        };

        Ok(ReturnFromFunction(ReturnFromFunctionNode {
            node: Box::new(result),
            return_type_id: DefaultTypeIds::never(),
        }))
    }
}