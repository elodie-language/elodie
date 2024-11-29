use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::ir::{DeclareExternalFunctionNode, DeclareFunctionNode, DeclarePackageNode, DeclareVariableNode};
use crate::r#type::TypeId;
use crate::run::Runner;
use crate::run::value::{FunctionValue, IntrinsicFunctionValue, PackageValue, Value};
use crate::run::value::Value::IntrinsicFunction;

impl<'a> Runner<'a> {

    pub(crate) fn run_external_function_declaration(&mut self, node: &DeclareExternalFunctionNode) -> crate::run::Result<Value> {
        unimplemented!()
    }

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

        for node in &node.definitions {
            // self.run_node(&Node::DefineType(node))?;
            for func in &node.functions {
                let func_ident = func.identifier.0;
                let func = func;
                let value = self.run_function_declaration(func)?;

                let Value::Function(func) = value else { panic!() };
                self.type_definitions.add_function(TypeId(99), func_ident, func);
            }
        }

        let mut external_functions = HashMap::new();

        for node in &node.external_functions {
            // dbg!(node);
            // println!("{}", self.ctx.get_str(node.identifier.0));

            let fun = self.ctx.get_str(node.identifier.0);
// FIXME load
            match fun {
             "cos_f64" => {
                 external_functions.insert(node.identifier.0, IntrinsicFunctionValue(Rc::new(move |args: &[Value]| {
                     let Value::Number(arg) = args.get(0).cloned().unwrap() else {panic!()};

                     Ok(Value::Number(arg.cos()))
                 })));
             },
              _ => unimplemented!("{fun}")
             }
        }

        Ok(
            Value::Package(PackageValue {
                identifier: node.identifier.0,
                functions,
                packages,
                external_functions
            }),
        )
    }
}