use std::collections::HashMap;
use std::ops::Deref;

use crate::ir::{DeclareFunctionNode, DeclarePackageNode, DeclareVariableNode};
use crate::run::Runner;
use crate::run::value::{FunctionValue, PackageValue, Value};

impl<'a> Runner<'a> {
    pub(crate) fn run_variable_declaration(&mut self, node: &DeclareVariableNode) -> crate::run::Result<Value> {
        let name = node.identifier.0.clone();
        let value = self.run_node(node.value.deref())?;
        self.scope.insert_value(name, value);
        Ok(Value::Unit)
    }

    pub(crate) fn run_function_declaration(&mut self, node: &DeclareFunctionNode) -> crate::run::Result<Value> {
        let name = node.identifier.0.clone();

        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(arg.clone())
        }

        let f = Value::Function(FunctionValue {
            body: node.body.clone(),
            arguments,
        });

        self.scope.insert_value(name, f.clone());
        Ok(f)
    }

    pub(crate) fn run_package_declaration(&mut self, node: &DeclarePackageNode) -> crate::run::Result<Value> {

        let mut functions = HashMap::new();
        for node in &node.functions {
            let name = node.identifier.0.clone();
            let mut arguments = Vec::with_capacity(node.arguments.len());
            for arg in &node.arguments {
                arguments.push(arg.clone())
            }
            let f = FunctionValue {
                body: node.body.clone(),
                arguments,
            };
            functions.insert(name, f);
        }

        let mut packages = HashMap::new();
        for node in &node.packages {
            let identifier = node.identifier.0.clone();
            let value = self.run_package_declaration(node)?;
            let Value::Package(package) = value else { panic!() };
            packages.insert(identifier, package);
        }

        Ok(
            Value::Package(PackageValue {
                identifier: node.identifier.0,
                functions,
                packages,
            }),
        )
    }
}