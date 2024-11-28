use std::ops::Deref;
use std::rc::Rc;

use crate::{ir, parse};
use crate::common::PackagePath;
use crate::compile::Compiler;
use crate::ir::{CalculateNode, CalculationOperator, CallFunctionNode, CallFunctionOfObjectNode, CallFunctionOfPackageNode, CallFunctionWithLambdaNode, CompareNode, CompareOperator, Identifier, InstantiateTypeNode, LoadValueFromObjectNode, LoadValueFromSelfNode, NamedArgumentNode};
use crate::parse::{InfixNode, InfixOperator, Node, TypeNode};
use crate::parse::Node::Type;
use crate::r#type::TypeId;

impl<'a> Compiler<'a> {
    pub(crate) fn compile_infix(&mut self, node: &parse::InfixNode) -> crate::compile::Result<ir::Node> {
        let InfixNode { left, operator, right } = node;

        if left.is_type() && matches!(operator, InfixOperator::Call(_)) && right.is_tuple() {
            return self.compile_type_instantiation(node);
        }

        // function call
        if left.is_identifier() && matches!(operator, InfixOperator::Call(_)) && right.is_tuple() {
            let Node::Identifier(function_identifier) = left.deref() else { todo!() };
            let arguments = self.compile_arguments(right.as_tuple())?;
            return Ok(ir::Node::CallFunction(CallFunctionNode { function: Identifier(function_identifier.value()), arguments }));
        }

        // call function of object / self
        if left.is_infix() && matches!(left.as_infix().operator, InfixOperator::AccessProperty(_)) && matches!(operator, InfixOperator::Call(_)) {
            let ir::Node::LoadValueFromObject(LoadValueFromObjectNode { object, property }) = self.compile_access_property(left.as_infix())? else { panic!() };

            let arguments = self.compile_arguments(right.as_tuple())?;

            return Ok(ir::Node::CallFunctionOfObject(CallFunctionOfObjectNode {
                object: ir::Identifier::from(object),
                function: ir::Identifier::from(property),
                arguments,
            }));
        };

        // lambda call
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

        // call function of package
        if left.is_infix() && matches!(left.as_infix().operator, InfixOperator::AccessPackage(_)) && matches!(operator, InfixOperator::Call(_)) {
            let arguments = self.compile_arguments(node.right.as_tuple())?;

            // FIXME
            let paths = {
                if left.as_infix().left.is_infix() && matches!(left.as_infix().left.as_infix().operator, InfixOperator::AccessPackage(_)){
                    self.package_path(left.as_infix())
                }else{
                    vec![Identifier::from(left.as_infix().left.as_identifier())]
                }
            };

            let function_identifier = left.as_infix().right.as_identifier();

            return Ok(ir::Node::CallFunctionOfPackage(CallFunctionOfPackageNode {
                package: PackagePath::from(paths.into_iter().map(|p|p.0).collect::<Vec<_>>()),
                function: Identifier(function_identifier.value()),
                arguments,
            }));
        }

        // self.variable
        if left.is_itself() && matches!(operator, InfixOperator::AccessProperty(_)) && right.is_identifier() {
            let property = right.as_identifier();
            return Ok(ir::Node::LoadValueFromSelf(LoadValueFromSelfNode {
                property: ir::Identifier::from(property),
            }));
        }

        // variable.variable
        if left.is_identifier() && matches!(operator, InfixOperator::AccessProperty(_)) && right.is_identifier() {
            // FIXME support chaining objects root.level_one.level_two..
            let object = left.as_identifier();
            let property = right.as_identifier();
            return Ok(ir::Node::LoadValueFromObject(LoadValueFromObjectNode {
                object: ir::Identifier::from(object),
                property: ir::Identifier::from(property),
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

        unimplemented!("{:#?}", node);
    }


    fn compile_access_property(&mut self, node: &parse::InfixNode) -> crate::compile::Result<ir::Node> {
        let InfixNode { left, operator, right } = node;

        if let Node::Itself(_) = left.deref() {
            if let Node::Identifier(property) = right.deref() {
                return Ok(ir::Node::LoadValueFromSelf(LoadValueFromSelfNode {
                    property: ir::Identifier::from(property),
                }));
            }
        }

        let Node::Identifier(object_identifier) = left.deref() else { todo!() };

        let Node::Identifier(property) = right.deref() else { todo!() };

        return Ok(ir::Node::LoadValueFromObject(LoadValueFromObjectNode {
            object: ir::Identifier::from(object_identifier),
            property: ir::Identifier::from(property),
        }));
    }

    fn compile_type_instantiation(&mut self, node: &parse::InfixNode) -> crate::compile::Result<ir::Node> {
        let InfixNode { left, operator, right } = node;

        let Type(TypeNode::Custom(type_node)) = left.deref() else { panic!() };
        let Node::Tuple(arguments_node) = right.deref() else { panic!() };

        let mut arguments = self.compile_named_arguments(arguments_node)?;

        return Ok(ir::Node::InstantiateType(InstantiateTypeNode {
            type_id: TypeId(23),
            type_name: Identifier(type_node.token.value()),
            arguments,
        }));
    }

    fn compile_arguments(&mut self, node: &parse::TupleNode) -> crate::compile::Result<Vec<ir::Node>> {
        let mut result = Vec::with_capacity(node.nodes.len());
        for node in &node.nodes {
            result.push(self.compile_node(node)?)
        }
        Ok(result)
    }

    fn compile_named_arguments(&mut self, node: &parse::TupleNode) -> crate::compile::Result<Vec<NamedArgumentNode>> {
        let mut result = Vec::with_capacity(node.nodes.len());

        for node in &node.nodes {
            let Node::Infix(InfixNode { left, operator, right }) = node else { panic!() };
            assert!(matches!(operator, InfixOperator::Assign(_)));
            let Node::Identifier(identifier) = left.deref() else { panic!() };
            let right = self.compile_node(right)?;
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