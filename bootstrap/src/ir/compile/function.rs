use std::ops::Deref;
use std::rc::Rc;

use crate::common::{BaseType, DefaultTypeIds};
use crate::frontend::parse;
use crate::ir;
use crate::ir::{BlockNode, DeclareFunctionNode, FunctionArgumentNode, Identifier, Node, ReturnFromFunctionNode};
use crate::ir::compile::Compiler;
use crate::ir::Node::ReturnFromFunction;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_declare_function(&mut self, node: &parse::FunctionDeclarationNode) -> crate::ir::compile::Result<ir::Node> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(Rc::new(self.compile_declare_function_argument(arg)?))
        }

        let mut body = vec![];
        for node in &node.block.nodes {
            body.push(self.compile_node(node)?)
        }

        let return_type = if let Some(ty) = &node.return_type {
            match ty.deref() {
                parse::TypeNode::Boolean(_) => self.ctx.type_table.get_base_type_id(&BaseType::Boolean),
                parse::TypeNode::Number(_) => self.ctx.type_table.get_base_type_id(&BaseType::Number),
                parse::TypeNode::String(_) => self.ctx.type_table.get_base_type_id(&BaseType::String),
                parse::TypeNode::Function(_) => DefaultTypeIds::never(),
                parse::TypeNode::Custom(_) => DefaultTypeIds::never(),
            }
        } else {
            DefaultTypeIds::never()
        };

        Ok(ir::Node::DeclareFunction(DeclareFunctionNode {
            identifier: Identifier::from(&node.identifier),
            arguments,
            return_type,
            body: Rc::new(BlockNode { body, return_type: DefaultTypeIds::never() }),
        }))
    }

    pub(crate) fn compile_declare_function_argument(&mut self, node: &parse::FunctionDeclarationArgumentNode) -> crate::ir::compile::Result<ir::FunctionArgumentNode> {
        Ok(FunctionArgumentNode {
            identifier: Identifier::from(&node.identifier),
            ty: DefaultTypeIds::never(),
        })
    }

    pub(crate) fn compile_function_return(&mut self, node: &parse::ReturnNode) -> crate::ir::compile::Result<ir::Node> {
        let result = if let Some(ref node) = node.result {
            self.compile_node(node.deref())?
        } else {
            Node::Unit
        };

        Ok(ReturnFromFunction(ReturnFromFunctionNode {
            node: Box::new(result),
            return_type_id: DefaultTypeIds::never(),
        }))
    }
}