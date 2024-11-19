use std::ops::Deref;

use crate::ast;
use crate::ast::{CalculateNode, CalculationOperator, CallFunctionOfObjectNode, CompareNode, CompareOperator, parse};
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
                    arguments: vec![ast::Node::LoadVariable(ast::LoadVariableNode {
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