use std::ops::Deref;

use crate::ast;
use crate::ast::{CalculateNode, CalculationOperator, CallFunctionNode, CallFunctionOfObjectNode, CompareNode, CompareOperator, parse};
use crate::ast::compiler::Compiler;
use crate::ast::parse::{InfixNode, InfixOperator, LiteralNode, Node};
use crate::ast::r#type::DefaultTypeIds;

impl Compiler {
    pub(crate) fn compile_infix(&mut self, node: &parse::InfixNode) -> crate::ast::compiler::Result<ast::Node> {
        let InfixNode { left, right, operator } = node;

        if let InfixOperator::AccessProperty(_) = operator {
            let Node::Identifier(object_identifier) = left.deref() else { todo!() };
            let Node::Infix(InfixNode { left, operator, right }) = right.deref() else { todo!() };
            let Node::Identifier(function_identifier) = left.deref() else { todo!() };
            let Node::Tuple(tuple) = right.deref() else { todo!() };

            if let Node::Identifier(identifier_node) = &tuple.nodes[0] {
                // load variable

                return Ok(ast::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                    object: ast::Identifier(object_identifier.value().to_string()),
                    function: ast::Identifier(function_identifier.value().to_string()),
                    arguments: vec![ast::Node::UseIdentifier(ast::UseIdentifierNode {
                        identifier: ast::Identifier(identifier_node.value().to_string()),
                        type_id: DefaultTypeIds::string(),
                    })],
                }));
            };

            if let Node::Literal(LiteralNode::String(value)) = &tuple.nodes[0] {
                return Ok(ast::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                    object: ast::Identifier(object_identifier.value().to_string()),
                    function: ast::Identifier(function_identifier.value().to_string()),
                    arguments: vec![ast::Node::ValueString(value.value().to_string())],
                }));
            }


            let parse::Node::Infix(InfixNode { left, right, operator }) = &tuple.nodes[0] else { panic!() };
            if let InfixOperator::Call(_) = operator {
                let ast::Node::UseIdentifier(identifier) = self.compile_node(left.deref())? else { panic!() };

                let parse::Node::Tuple(tuple_node) = right.deref() else {panic!()};
                let mut arguments = Vec::with_capacity(tuple_node.nodes.len());
                for node in &tuple_node.nodes{
                    arguments.push(self.compile_node(node)?)
                }

                let function_call = ast::Node::CallFunction(CallFunctionNode {
                    function: identifier.identifier,
                    arguments,
                });

                return Ok(ast::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                    object: ast::Identifier(object_identifier.value().to_string()),
                    function: ast::Identifier(function_identifier.value().to_string()),
                    arguments: vec![function_call],
                }));
            }

            if let InfixOperator::Add(_) = operator {
                let parse::Node::Infix(infix_node) = &tuple.nodes[0] else { panic!() };
                let arg = self.compile_infix(infix_node)?;

                return Ok(ast::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                    object: ast::Identifier(object_identifier.value().to_string()),
                    function: ast::Identifier(function_identifier.value().to_string()),
                    arguments: vec![arg],
                }));
            }

            unimplemented!();
        }

        if let InfixOperator::Call(_) = operator {
            let ast::Node::UseIdentifier(identifier) = self.compile_node(left.deref())? else { panic!() };
            // let right = Box::new(self.compile_node(right.deref())?);

            // println!("{:?}", right);

            let parse::Node::Tuple(tuple_node) = right.deref() else {panic!()};
            let mut arguments = Vec::with_capacity(tuple_node.nodes.len());
            for node in &tuple_node.nodes{
                arguments.push(self.compile_node(node)?)
            }


            return Ok(ast::Node::CallFunction(CallFunctionNode {
                function: identifier.identifier,
                arguments,
            }));
        }

        if let InfixOperator::Add(_) = operator {
            let left = Box::new(self.compile_node(left.deref())?);
            let right = Box::new(self.compile_node(right.deref())?);
            return Ok(ast::Node::Calculate(CalculateNode {
                left,
                operator: CalculationOperator::Add,
                right,
            }));
        }


        if let InfixOperator::Equal(_) = operator {
            let left = Box::new(self.compile_node(left.deref())?);
            let right = Box::new(self.compile_node(right.deref())?);

            return Ok(ast::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::Equal,
                right,
            }));
        }

        if let InfixOperator::NotEqual(_) = operator {
            let left = Box::new(self.compile_node(left.deref())?);
            let right = Box::new(self.compile_node(right.deref())?);

            return Ok(ast::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::NotEqual,
                right,
            }));
        }


        if let InfixOperator::GreaterThan(_) = operator {
            let left = Box::new(self.compile_node(left.deref())?);
            let right = Box::new(self.compile_node(right.deref())?);

            return Ok(ast::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::GreaterThan,
                right,
            }));
        }

        if let InfixOperator::Multiply(_) = operator {
            let left = Box::new(self.compile_node(left.deref())?);
            let right = Box::new(self.compile_node(right.deref())?);

            return Ok(ast::Node::Calculate(CalculateNode {
                left,
                operator: CalculationOperator::Multiply,
                right,
            }));
        }

        unimplemented!("{:?}", node);
    }
}