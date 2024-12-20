use std::ops::Deref;
use std::rc::Rc;

use crate::common::PackagePath;
use crate::frontend::{ast, parse};
use crate::frontend::ast::{AccessVariableOfObjectNode, AccessVariableOfSelfNode,  Generator, NamedArgument, SPAN_NOT_IMPLEMENTED};
use crate::frontend::ast::node::{Ast, AstNode, CalculateNode, CalculationOperator, CallFunctionNode, CallFunctionOfObjectNode, CallFunctionOfPackageNode, CallFunctionWithLambdaNode, CompareNode, CompareOperator, Identifier};
use crate::frontend::parse::{InfixNode, InfixOperator, Node, TypeNode};
use crate::frontend::parse::Node::Type;

impl<'a> Generator<'a> {
    pub(crate) fn generate_infix(&mut self, node: &parse::InfixNode) -> ast::Result<AstNode> {
        let InfixNode {
            left,
            operator,
            right,
            token,
        } = node;

        if left.is_type() && matches!(operator, InfixOperator::Call(_)) && right.is_tuple() {
            return self.generate_type_instantiation(node);
        }

        // function call
        if left.is_identifier() && matches!(operator, InfixOperator::Call(_)) && right.is_tuple() {
            let Node::Identifier(function_identifier) = left.deref() else {
                todo!()
            };
            let arguments = self.generate_arguments(right.as_tuple())?;
            return Ok(AstNode::new(ast::Node::CallFunction(CallFunctionNode {
                function: Identifier(function_identifier.0.value),
                arguments,
            }), SPAN_NOT_IMPLEMENTED.clone()));
        }

        // call function of object / self
        if left.is_infix()
            && matches!(left.as_infix().operator, InfixOperator::AccessProperty(_))
            && matches!(operator, InfixOperator::Call(_))
        {
            let ast::Node::AccessVariableOfObject(AccessVariableOfObjectNode {
                                                      object,
                                                      variable,
                                                  }) = self.generate_access_variable(left.as_infix())?.node_to_owned()
                else {
                    panic!()
                };

            let arguments = self.generate_arguments(right.as_tuple())?;

            // FIXME add type information
            return Ok(AstNode::new(ast::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                object: ast::Identifier(object.0.clone()),
                function: ast::Identifier(variable.0.clone()),
                arguments,
            }), SPAN_NOT_IMPLEMENTED.clone()));
        };

        // lambda call
        if let InfixOperator::LambdaCall(_) = operator {
            let left = self.generate_node(left.deref())?;
            let right = self.generate_node(right.deref())?;

            let ast::Node::CallFunction(call_function) = left.node_to_owned() else {
                panic!()
            };
            let ast::Node::Block(lambda) = right.node_to_owned() else {
                panic!()
            };

            return Ok(AstNode::new(ast::Node::CallFunctionWithLambda(
                CallFunctionWithLambdaNode {
                    function: call_function.function,
                    lambda: Rc::new(lambda),
                    arguments: call_function.arguments,
                },
            ), SPAN_NOT_IMPLEMENTED.clone()));
        }

        // call function of package
        if left.is_infix()
            && matches!(left.as_infix().operator, InfixOperator::AccessPackage(_))
            && matches!(operator, InfixOperator::Call(_))
        {
            let arguments = self.generate_arguments(node.right.as_tuple())?;

            // FIXME
            let paths = {
                if left.as_infix().left.is_infix()
                    && matches!(
                        left.as_infix().left.as_infix().operator,
                        InfixOperator::AccessPackage(_)
                    )
                {
                    self.package_path(left.as_infix())
                } else {
                    vec![Identifier(left.as_infix().left.as_identifier().0.value)]
                }
            };

            let function_identifier = left.as_infix().right.as_identifier();

            return Ok(AstNode::new(ast::Node::CallFunctionOfPackage(
                CallFunctionOfPackageNode {
                    package: PackagePath::from(
                        paths.into_iter().map(|p| p.0).collect::<Vec<_>>(),
                    ),
                    function: Identifier(function_identifier.value()),
                    arguments,
                },
            ), SPAN_NOT_IMPLEMENTED.clone()));
        }

        // self.variable
        if left.is_itself()
            && matches!(operator, InfixOperator::AccessProperty(_))
            && right.is_identifier()
        {
            let variable = right.as_identifier();
            return Ok(AstNode::new(ast::Node::AccessVariableOfSelf(AccessVariableOfSelfNode {
                variable: ast::Identifier(variable.value()),
            }), SPAN_NOT_IMPLEMENTED.clone()));
        }

        // variable.variable
        if left.is_identifier()
            && matches!(operator, InfixOperator::AccessProperty(_))
            && right.is_identifier()
        {
            // FIXME support chaining objects root.level_one.level_two..
            let object = left.as_identifier();
            let variable = right.as_identifier();

            return Ok(AstNode::new(ast::Node::AccessVariableOfObject(AccessVariableOfObjectNode {
                object: ast::Identifier(object.value()),
                variable: ast::Identifier(variable.value()),
            }), SPAN_NOT_IMPLEMENTED.clone()));
        }

        if let InfixOperator::Add(_) = operator {
            let left = Rc::new(self.generate_node(left.deref())?);
            let right = Rc::new(self.generate_node(right.deref())?);
            return Ok(AstNode::new(ast::Node::Calculate(CalculateNode {
                left,
                operator: CalculationOperator::Add,
                right,
            }), SPAN_NOT_IMPLEMENTED.clone()));
        }

        if let InfixOperator::Equal(_) = operator {
            let left = Rc::new(self.generate_node(left.deref())?);
            let right = Rc::new(self.generate_node(right.deref())?);

            return Ok(AstNode::new(ast::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::Equal,
                right,
            }), SPAN_NOT_IMPLEMENTED.clone()));
        }

        if let InfixOperator::NotEqual(_) = operator {
            let left = Rc::new(self.generate_node(left.deref())?);
            let right = Rc::new(self.generate_node(right.deref())?);

            return Ok(AstNode::new(ast::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::NotEqual,
                right,
            }), SPAN_NOT_IMPLEMENTED.clone()));
        }

        if let InfixOperator::GreaterThan(_) = operator {
            let left = Rc::new(self.generate_node(left.deref())?);
            let right = Rc::new(self.generate_node(right.deref())?);

            return Ok(AstNode::new(ast::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::GreaterThan,
                right,
            }), SPAN_NOT_IMPLEMENTED.clone()));
        }

        if let InfixOperator::Multiply(_) = operator {
            let left = Rc::new(self.generate_node(left.deref())?);
            let right = Rc::new(self.generate_node(right.deref())?);

            return Ok(AstNode::new(ast::Node::Calculate(CalculateNode {
                left,
                operator: CalculationOperator::Multiply,
                right,
            }), SPAN_NOT_IMPLEMENTED.clone()));
        }

        unimplemented!("{:#?}", node);
    }

    fn generate_access_variable(&mut self, node: &parse::InfixNode) -> ast::Result<AstNode> {
        let InfixNode {
            left,
            operator,
            right,
            token,
        } = node;

        if let Node::Itself(_) = left.deref() {
            if let Node::Identifier(variable) = right.deref() {
                return Ok(AstNode::new(ast::Node::AccessVariableOfSelf(AccessVariableOfSelfNode {
                    variable: Identifier(variable.0.value),
                }), SPAN_NOT_IMPLEMENTED.clone()));
            }
        }

        let Node::Identifier(object_identifier) = left.deref() else {
            todo!()
        };

        let Node::Identifier(variable) = right.deref() else {
            todo!()
        };

        return Ok(AstNode::new(ast::Node::AccessVariableOfObject(AccessVariableOfObjectNode {
            object: ast::Identifier(object_identifier.0.value),
            variable: ast::Identifier(variable.0.value),
        }), SPAN_NOT_IMPLEMENTED.clone()));
    }

    fn generate_type_instantiation(&mut self, node: &parse::InfixNode) -> ast::Result<AstNode> {
        let InfixNode {
            left,
            operator,
            right,
            token,
        } = node;

        let Type(TypeNode::Object(type_node)) = left.deref() else {
            panic!()
        };
        let Node::Tuple(arguments_node) = right.deref() else {
            panic!()
        };

        let mut arguments = self.generate_named_arguments(arguments_node)?;

        return Ok(AstNode::new(ast::Node::InstantiateType(ast::InstantiateTypeNode {
            r#type: Identifier(type_node.token.value),
            arguments,
        }), SPAN_NOT_IMPLEMENTED.clone()));
    }

    fn generate_arguments(&mut self, node: &parse::TupleNode) -> ast::Result<Vec<AstNode>> {
        let mut result = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            result.push(self.generate_node(node)?)
        }
        Ok(result)
    }

    fn generate_named_arguments(
        &mut self,
        node: &parse::TupleNode,
    ) -> ast::Result<Vec<NamedArgument<AstNode>>> {
        let mut result = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            let Node::Infix(InfixNode {
                                left,
                                operator,
                                right,
                                token,
                            }) = node
                else {
                    panic!()
                };
            assert!(matches!(operator, InfixOperator::Assign(_)));
            let Node::Identifier(identifier) = left.deref() else {
                panic!()
            };
            let right = self.generate_node(right)?;
            result.push(NamedArgument {
                identifier: Identifier(identifier.0.value),
                value: right,
            })
        }

        Ok(result)
    }

    fn package_path(&self, node: &parse::InfixNode) -> Vec<Identifier> {
        let mut paths = vec![];

        let mut current = node.left.as_infix();

        loop {
            let InfixNode {
                left,
                operator,
                right,
                token,
            } = current;

            if let parse::Node::Identifier(identifier) = right.deref() {
                paths.push(Identifier(identifier.0.value))
            }

            if let parse::Node::Identifier(identifier) = left.deref() {
                paths.push(Identifier(identifier.0.value))
            }

            if !left.is_infix() {
                paths.reverse();
                return paths;
            }

            current = left.as_infix()
        }
    }
}
