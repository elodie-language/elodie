use std::ops::Deref;
use std::rc::Rc;

use crate::{ir, parse};
use crate::ir::{BlockNode, DeclareFunctionNode, FunctionArgumentNode, Identifier, Node, ReturnFromFunctionNode};
use crate::ir::Node::ReturnFromFunction;
use crate::compile::Compiler;
use crate::parse::{TypeFundamentalNode, TypeNode};
use crate::parse::LiteralNode::Boolean;
use crate::r#type::{BaseType, DefaultTypeIds};

impl<'a> Compiler<'a> {

    pub(crate) fn compile_declare_function(&mut self, node: &parse::FunctionDeclarationNode) -> crate::compile::Result<ir::Node> {
        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(Rc::new(self.compile_declare_function_argument(arg)?))
        }

        let mut body = vec![];
        for node in &node.block.nodes {
            body.push(self.compile_node(node)?)
        }

        let return_type = if let Some(ty) = &node.return_type{
            match ty.deref(){
                TypeNode::Fundamental(inner) => {
                    match inner {
                        TypeFundamentalNode::Boolean(_) => self.ctx.type_table.get_base_type_id(&BaseType::Boolean),
                        TypeFundamentalNode::Number(_) => self.ctx.type_table.get_base_type_id(&BaseType::Number),
                        TypeFundamentalNode::String(_) => self.ctx.type_table.get_base_type_id(&BaseType::String)
                    }
                }
                TypeNode::Function(_) => DefaultTypeIds::never(),
                TypeNode::Custom(_) => DefaultTypeIds::never(),
            }
        }else{
            DefaultTypeIds::never()
        };

        Ok(ir::Node::DeclareFunction(DeclareFunctionNode {
            identifier: Identifier::from(&node.identifier),
            arguments,
            return_type,
            body: Rc::new(BlockNode { body, return_type: DefaultTypeIds::never() }),
        }))
    }

    pub(crate) fn compile_declare_function_argument(&mut self, node: &parse::FunctionDeclarationArgumentNode) -> crate::compile::Result<ir::FunctionArgumentNode> {
        Ok(FunctionArgumentNode {
            identifier: Identifier::from(&node.identifier),
            ty: DefaultTypeIds::never(),
        })
    }

    pub(crate) fn compile_function_return(&mut self, node: &parse::ReturnNode) -> crate::compile::Result<ir::Node> {
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