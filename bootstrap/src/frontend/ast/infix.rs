use std::ops::Deref;
use std::rc::Rc;

use crate::common::PackagePath;
use crate::frontend::{ast, parse};
use crate::frontend::ast::Generator;
use crate::frontend::ast::node::{CalculateNode, CalculationOperator, CallFunctionNode, CallFunctionOfObjectNode, CallFunctionOfPackageNode, CallFunctionWithLambdaNode, CompareNode, CompareOperator, Identifier, LoadValueFromObjectNode, LoadValueFromSelfNode, NamedArgumentNode};
use crate::frontend::parse::{InfixNode, InfixOperator, Node, TypeNode};
use crate::frontend::parse::Node::Type;

impl<'a> Generator<'a> {
    pub(crate) fn generate_infix(&mut self, node: &parse::InfixNode) -> ast::Result<ast::Node> {
        let InfixNode { left, operator, right } = node;

        if left.is_type() && matches!(operator, InfixOperator::Call(_)) && right.is_tuple() {
            return self.generate_type_instantiation(node);
        }

        // function call
        if left.is_identifier() && matches!(operator, InfixOperator::Call(_)) && right.is_tuple() {
            let Node::Identifier(function_identifier) = left.deref() else { todo!() };
            let arguments = self.generate_arguments(right.as_tuple())?;
            return Ok(ast::Node::CallFunction(CallFunctionNode { function: Identifier(function_identifier.value()), arguments }));
        }

        // call function of object / self
        if left.is_infix() && matches!(left.as_infix().operator, InfixOperator::AccessProperty(_)) && matches!(operator, InfixOperator::Call(_)) {
            let ast::Node::LoadValueFromObject(LoadValueFromObjectNode { object, property }) = self.generate_access_property(left.as_infix())? else { panic!() };

            let arguments = self.generate_arguments(right.as_tuple())?;

            // FIXME add type information
            return Ok(ast::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                object: ast::Identifier::from(object),
                function: ast::Identifier::from(property),
                arguments,
            }));
        };

        // lambda call
        if let InfixOperator::LambdaCall(_) = operator {
            let left = self.generate_node(left.deref())?;
            let right = self.generate_node(right.deref())?;

            let ast::Node::CallFunction(call_function) = left else { panic!() };
            let ast::Node::Block(lambda) = right else { panic!() };

            return Ok(ast::Node::CallFunctionWithLambda(CallFunctionWithLambdaNode {
                call_function,
                lambda: Rc::new(lambda),
            }));
        }

        // call function of package
        if left.is_infix() && matches!(left.as_infix().operator, InfixOperator::AccessPackage(_)) && matches!(operator, InfixOperator::Call(_)) {
            let arguments = self.generate_arguments(node.right.as_tuple())?;

            // FIXME
            let paths = {
                if left.as_infix().left.is_infix() && matches!(left.as_infix().left.as_infix().operator, InfixOperator::AccessPackage(_)) {
                    self.package_path(left.as_infix())
                } else {
                    vec![Identifier::from(left.as_infix().left.as_identifier())]
                }
            };

            let function_identifier = left.as_infix().right.as_identifier();

            return Ok(ast::Node::CallFunctionOfPackage(CallFunctionOfPackageNode {
                package: PackagePath::from(paths.into_iter().map(|p| p.0).collect::<Vec<_>>()),
                function: Identifier(function_identifier.value()),
                arguments,
            }));
        }

        // self.variable
        if left.is_itself() && matches!(operator, InfixOperator::AccessProperty(_)) && right.is_identifier() {
            let property = right.as_identifier();
            return Ok(ast::Node::LoadValueFromSelf(LoadValueFromSelfNode {
                property: ast::Identifier::from(property),
            }));
        }

        // variable.variable
        if left.is_identifier() && matches!(operator, InfixOperator::AccessProperty(_)) && right.is_identifier() {
            // FIXME support chaining objects root.level_one.level_two..
            let object = left.as_identifier();
            let property = right.as_identifier();


            return Ok(ast::Node::LoadValueFromObject(LoadValueFromObjectNode {
                object: ast::Identifier::from(object),
                property: ast::Identifier::from(property),
            }));
        }


        if let InfixOperator::Add(_) = operator {
            let left = Box::new(self.generate_node(left.deref())?);
            let right = Box::new(self.generate_node(right.deref())?);
            return Ok(ast::Node::Calculate(CalculateNode {
                left,
                operator: CalculationOperator::Add,
                right,
            }));
        }

        if let InfixOperator::Equal(_) = operator {
            let left = Box::new(self.generate_node(left.deref())?);
            let right = Box::new(self.generate_node(right.deref())?);

            return Ok(ast::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::Equal,
                right,
            }));
        }

        if let InfixOperator::NotEqual(_) = operator {
            let left = Box::new(self.generate_node(left.deref())?);
            let right = Box::new(self.generate_node(right.deref())?);

            return Ok(ast::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::NotEqual,
                right,
            }));
        }

        if let InfixOperator::GreaterThan(_) = operator {
            let left = Box::new(self.generate_node(left.deref())?);
            let right = Box::new(self.generate_node(right.deref())?);

            return Ok(ast::Node::Compare(CompareNode {
                left,
                operator: CompareOperator::GreaterThan,
                right,
            }));
        }

        if let InfixOperator::Multiply(_) = operator {
            let left = Box::new(self.generate_node(left.deref())?);
            let right = Box::new(self.generate_node(right.deref())?);

            return Ok(ast::Node::Calculate(CalculateNode {
                left,
                operator: CalculationOperator::Multiply,
                right,
            }));
        }

        unimplemented!("{:#?}", node);
    }


    fn generate_access_property(&mut self, node: &parse::InfixNode) -> ast::Result<ast::Node> {
        let InfixNode { left, operator, right } = node;

        if let Node::Itself(_) = left.deref() {
            if let Node::Identifier(property) = right.deref() {
                return Ok(ast::Node::LoadValueFromSelf(LoadValueFromSelfNode {
                    property: ast::Identifier::from(property),
                }));
            }
        }

        let Node::Identifier(object_identifier) = left.deref() else { todo!() };

        let Node::Identifier(property) = right.deref() else { todo!() };

        return Ok(ast::Node::LoadValueFromObject(LoadValueFromObjectNode {
            object: ast::Identifier::from(object_identifier),
            property: ast::Identifier::from(property),
        }));
    }

    fn generate_type_instantiation(&mut self, node: &parse::InfixNode) -> ast::Result<ast::Node> {
        let InfixNode { left, operator, right } = node;

        let Type(TypeNode::Custom(type_node)) = left.deref() else { panic!() };
        let Node::Tuple(arguments_node) = right.deref() else { panic!() };

        let mut arguments = self.generate_named_arguments(arguments_node)?;

        return Ok(ast::Node::InstantiateType(ast::InstantiateTypeNode {
            type_name: Identifier(type_node.token.value()),
            arguments,
        }));
    }

    fn generate_arguments(&mut self, node: &parse::TupleNode) -> ast::Result<Vec<ast::Node>> {
        let mut result = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            result.push(self.generate_node(node)?)
        }
        Ok(result)
    }

    fn generate_named_arguments(&mut self, node: &parse::TupleNode) -> ast::Result<Vec<NamedArgumentNode>> {
        let mut result = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            let Node::Infix(InfixNode { left, operator, right }) = node else { panic!() };
            assert!(matches!(operator, InfixOperator::Assign(_)));
            let Node::Identifier(identifier) = left.deref() else { panic!() };
            let right = self.generate_node(right)?;
            result.push(NamedArgumentNode {
                identifier: Identifier::from(identifier),
                value: right,
            })
        }

        Ok(result)
    }


    fn package_path(&self, node: &parse::InfixNode) -> Vec<Identifier> {
        let mut paths = vec![];

        let mut current = node.left.as_infix();

        loop {
            let InfixNode { left, operator, right } = current;

            if let parse::Node::Identifier(identifier) = right.deref() {
                paths.push(Identifier::from(identifier))
            }

            if let parse::Node::Identifier(identifier) = left.deref() {
                paths.push(Identifier::from(identifier))
            }

            if !left.is_infix() {
                paths.reverse();
                return paths;
            }

            current = left.as_infix()
        }
    }
}