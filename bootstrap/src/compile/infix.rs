use std::ops::Deref;
use std::rc::Rc;

use crate::{ir, parse};
use crate::compile::Compiler;
use crate::ir::{CalculateNode, CalculationOperator, CallFunctionNode, CallFunctionOfObjectNode, CallFunctionOfPackageNode, CallFunctionWithLambdaNode, CompareNode, CompareOperator, Identifier, InstantiateTypeNode, LoadValueFromObjectNode, LoadValueFromSelfNode, LoadValueNode, NamedArgumentNode};
use crate::ir::Node::{CallFunction, CallFunctionOfObject};
use crate::parse::{InfixNode, InfixOperator, LiteralNode, Node, TypeNode};
use crate::parse::Node::Type;
use crate::r#type::{DefaultTypeIds, TypeId};

impl<'a> Compiler<'a> {
    pub(crate) fn compile_infix(&mut self, node: &parse::InfixNode) -> crate::compile::Result<ir::Node> {
        let InfixNode { left, right, operator } = node;

        if let InfixOperator::AccessPackage(_) = operator {
            let (paths, node) = self.handle_package_access(node);
            let InfixNode { left, right, operator } = node;
            if let InfixOperator::Call(_) = operator {
                let ir::Node::LoadValue(function_identifier) = self.compile_node(left.deref())? else { panic!() };

                let parse::Node::Tuple(tuple_node) = right.deref() else { panic!() };
                let mut arguments = Vec::with_capacity(tuple_node.nodes.len());
                for node in &tuple_node.nodes {
                    arguments.push(self.compile_node(node)?)
                }

                return Ok(ir::Node::CallFunctionOfPackage(CallFunctionOfPackageNode {
                    package: paths,
                    function: function_identifier.identifier.clone(),
                    arguments,
                }));
            }
        }

        if let InfixOperator::AccessProperty(_) = operator {
            if let Node::Itself(_) = left.deref() {
                if let Node::Identifier(property) = right.deref() {
                    return Ok(ir::Node::LoadValueFromSelf(LoadValueFromSelfNode {
                        property: ir::Identifier::from(property),
                    }));
                }
            }

            let Node::Identifier(object_identifier) = left.deref() else { todo!() };

            if let Node::Identifier(property) = right.deref() {
                return Ok(ir::Node::LoadValueFromObject(LoadValueFromObjectNode {
                    object: ir::Identifier::from(object_identifier),
                    property: ir::Identifier::from(property),
                }));
            }

            let Node::Infix(InfixNode { left, operator, right }) = right.deref() else { todo!() };

            // FIXME hack for list.length() == 1
            if let Node::Infix(infix_node) = left.deref() {
                let left = self.compile_infix(infix_node)?;
                let CallFunction(CallFunctionNode { function, arguments }) = left else { todo!() };
                let left = CallFunctionOfObject(CallFunctionOfObjectNode {
                    object: ir::Identifier::from(object_identifier),
                    function,
                    arguments,
                });

                return Ok(ir::Node::Compare(CompareNode {
                    left: Box::new(left),
                    operator: CompareOperator::Equal,
                    right: Box::new(self.compile_node(right)?),
                }));
            }

            let Node::Identifier(function_identifier) = left.deref() else { todo!() };


            let Node::Tuple(tuple) = right.deref() else { todo!("{right:#?}") };


            if tuple.nodes.is_empty() {
                return Ok(ir::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                    object: ir::Identifier::from(object_identifier),
                    function: ir::Identifier::from(function_identifier),
                    arguments: vec![],
                }));
            };

            if let Node::Identifier(identifier_node) = &tuple.nodes[0] {
                // load variable

                return Ok(ir::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                    object: ir::Identifier::from(object_identifier),
                    function: ir::Identifier::from(function_identifier),
                    arguments: vec![ir::Node::LoadValue(ir::LoadValueNode {
                        identifier: ir::Identifier::from(identifier_node),
                        type_id: DefaultTypeIds::string(),
                    })],
                }));
            };

            let mut args = Vec::new();
            let mut hit = false;

            for arg in &tuple.nodes {
                if let Node::Itself(_) = arg {
                    hit = true;
                    args.push(ir::Node::LoadValue(LoadValueNode { identifier: Identifier(self.ctx.string_cache.insert("self")), type_id: TypeId(99) }));
                } else if let Node::Literal(LiteralNode::String(value)) = arg {
                    hit = true;
                    args.push(ir::Node::ValueString(self.ctx.get_str(value.value()).to_string()));
                } else if let Node::Literal(LiteralNode::Number(value)) = arg {
                    hit = true;
                    args.push(ir::Node::ValueNumber(self.ctx.get_str(value.value()).parse().unwrap()));
                } else if let Node::Identifier(ident) = arg {
                    hit = true;
                    args.push(ir::Node::LoadValue(LoadValueNode { identifier: Identifier(ident.value()), type_id: TypeId(99) }))
                } else {
                    unimplemented!("{arg:#?}")
                }
            }

            if hit {
                return Ok(ir::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                    object: ir::Identifier::from(object_identifier),
                    function: ir::Identifier::from(function_identifier),
                    arguments: args,
                }));
            }


            let parse::Node::Infix(InfixNode { left, right, operator }) = &tuple.nodes[0] else { panic!() };
            if let InfixOperator::Call(_) = operator {
                let ir::Node::LoadValue(identifier) = self.compile_node(left.deref())? else { panic!() };

                let parse::Node::Tuple(tuple_node) = right.deref() else { panic!() };
                let mut arguments = Vec::with_capacity(tuple_node.nodes.len());
                for node in &tuple_node.nodes {
                    arguments.push(self.compile_node(node)?)
                }

                let function_call = ir::Node::CallFunction(CallFunctionNode {
                    function: identifier.identifier,
                    arguments,
                });

                return Ok(ir::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                    object: ir::Identifier::from(object_identifier),
                    function: ir::Identifier::from(function_identifier),
                    arguments: vec![function_call],
                }));
            }

            if let InfixOperator::Add(_) = operator {
                let parse::Node::Infix(infix_node) = &tuple.nodes[0] else { panic!() };
                let arg = self.compile_infix(infix_node)?;

                return Ok(ir::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                    object: ir::Identifier::from(object_identifier),
                    function: ir::Identifier::from(function_identifier),
                    arguments: vec![arg],
                }));
            }

            unimplemented!();
        }

        if let InfixOperator::Call(_) = operator {

            // type instantiation
            if let Type(TypeNode::Custom(custom_node)) = left.deref() {
                let parse::Node::Tuple(tuple_node) = right.deref() else { panic!() };
                let mut arguments = Vec::with_capacity(tuple_node.nodes.len());
                for node in &tuple_node.nodes {
                    let parse::Node::Infix(InfixNode { left, operator, right }) = node else { panic!() };
                    assert!(matches!(operator, InfixOperator::Assign(_)));
                    let parse::Node::Identifier(identifier) = left.deref() else { panic!() };
                    let right = self.compile_node(right)?;
                    arguments.push(NamedArgumentNode {
                        identifier: Identifier::from(identifier),
                        value: right,
                    })
                }
                return Ok(ir::Node::InstantiateType(InstantiateTypeNode {
                    type_id: TypeId(23),
                    type_name: Identifier(custom_node.token.value()),
                    arguments,
                }));
            };


            let ir::Node::LoadValue(identifier) = self.compile_node(left.deref())? else { panic!() };
            let parse::Node::Tuple(tuple_node) = right.deref() else { panic!() };
            let mut arguments = Vec::with_capacity(tuple_node.nodes.len());
            for node in &tuple_node.nodes {
                arguments.push(self.compile_node(node)?)
            }


            return Ok(ir::Node::CallFunction(CallFunctionNode {
                function: identifier.identifier,
                arguments,
            }));
        }

        if let InfixOperator::Add(_) = operator {
            let left = Box::new(self.compile_node(left.deref())?);
            let right = Box::new(self.compile_node(right.deref())?);
            return Ok(ir::Node::Calculate(CalculateNode {
                left,
                operator: CalculationOperator::Add,
                right,
            }));
        }


        if let InfixOperator::Equal(_) = operator {
            let left = Box::new(self.compile_node(left.deref())?);
            let right = Box::new(self.compile_node(right.deref())?);

            return Ok(ir::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::Equal,
                right,
            }));
        }

        if let InfixOperator::NotEqual(_) = operator {
            let left = Box::new(self.compile_node(left.deref())?);
            let right = Box::new(self.compile_node(right.deref())?);

            return Ok(ir::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::NotEqual,
                right,
            }));
        }


        if let InfixOperator::GreaterThan(_) = operator {
            let left = Box::new(self.compile_node(left.deref())?);
            let right = Box::new(self.compile_node(right.deref())?);

            return Ok(ir::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::GreaterThan,
                right,
            }));
        }

        if let InfixOperator::Multiply(_) = operator {
            let left = Box::new(self.compile_node(left.deref())?);
            let right = Box::new(self.compile_node(right.deref())?);

            return Ok(ir::Node::Calculate(CalculateNode {
                left,
                operator: CalculationOperator::Multiply,
                right,
            }));
        }

        if let InfixOperator::LambdaCall(_) = operator {
            let left = self.compile_node(left.deref())?;
            let right = self.compile_node(right.deref())?;

            let ir::Node::CallFunction(call_function) = left else { panic!() };
            let ir::Node::Block(lambda) = right else { panic!() };

            return Ok(ir::Node::CallFunctionWithLambda(CallFunctionWithLambdaNode {
                call_function,
                lambda: Rc::new(lambda),
            }));
        }

        unimplemented!("{:?}", node);
    }

    fn handle_package_access<'b>(&self, node: &'b parse::InfixNode) -> (Vec<Identifier>, &'b InfixNode) {
        let mut paths = vec![];
        let mut current = node;

        loop {
            if !matches!(current.right.deref(), Node::Infix(_)) {
                paths.reverse();
                return (paths, current);
            }

            let Node::Identifier(package_identifier) = current.left.deref() else { todo!() };
            paths.push(Identifier::from(package_identifier));

            let Node::Infix(right) = &current.right.deref() else { panic!() };

            if !matches!(current.operator,InfixOperator::AccessPackage(_)) {
                paths.reverse();
                return (paths, right);
            }

            current = right
        }
    }
}