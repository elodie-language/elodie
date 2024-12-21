use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::backend::run::Runner;
use crate::backend::run::value::{FunctionValue, IntrinsicFunctionValue, PackageValue, Value};
use crate::common::node::Node;
use crate::frontend::ast::{AstDeclareFunctionNode, AstDeclarePackageNode, AstDeclareVariableNode, AstTreeNode, SPAN_NOT_IMPLEMENTED};
use crate::ir::TypeId;

impl<'a> Runner<'a> {
    pub(crate) fn run_external_function_declaration(
        &mut self,
        node: &AstDeclareFunctionNode,
    ) -> crate::backend::run::Result<Value> {
        unimplemented!()
    }

    pub(crate) fn run_variable_declaration(
        &mut self,
        node: &AstDeclareVariableNode,
    ) -> crate::backend::run::Result<Value> {
        let name = node.variable.0;
        let value = self.run_node(node.value.deref())?;
        self.scope.insert_value(name, value);
        Ok(Value::Unit)
    }

    pub(crate) fn run_function_declaration(
        &mut self,
        node: &AstDeclareFunctionNode,
    ) -> crate::backend::run::Result<Value> {
        let name = node.function.0.clone();

        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(Rc::new(arg.clone()))
        }

        let f = Value::Function(FunctionValue {
            body: node.nodes.clone(),
            arguments,
        });

        self.scope.insert_value(name, f.clone());
        Ok(f)
    }

    pub(crate) fn run_package_declaration(
        &mut self,
        node: &AstDeclarePackageNode,
    ) -> crate::backend::run::Result<Value> {
        let mut functions = HashMap::new();
        for node in &node.functions {
            let name = node.function.0;
            let mut arguments = Vec::with_capacity(node.arguments.len());
            for arg in &node.arguments {
                arguments.push(Rc::new(arg.clone()))
            }
            let f = FunctionValue {
                body: node.nodes.clone(),
                arguments,
            };
            functions.insert(name, f);
        }

        let mut packages = HashMap::new();
        for node in &node.packages {
            let identifier = node.package.0;
            let value = self.run_package_declaration(node)?;
            let Value::Package(package) = value else {
                panic!()
            };
            packages.insert(identifier, package);
        }

        for node in &node.definitions {
            self.run_node(&AstTreeNode::new(Node::DefineType(node.clone()), SPAN_NOT_IMPLEMENTED.clone()))?;

            for func in &node.functions {
                let func_ident = func.function.0;
                let func = func;
                let value = self.run_function_declaration(func)?;

                let Value::Function(func) = value else {
                    panic!()
                };
                self.type_definitions
                    .add_function(TypeId(99), func_ident, func);
            }
        }

        let mut external_functions = HashMap::new();

        for node in &node.external_functions {
            // dbg!(node);
            // println!("{}", self.ctx.get_str(node.identifier.0));

            let function = self.ctx.get_str(node.function.0);
            // FIXME load

            let print_colors = self.print_colors.clone();
            match function {
                "cos_f64" => {
                    external_functions.insert(
                        node.function.0,
                        IntrinsicFunctionValue(Rc::new(move |args: &[Value]| {
                            let Value::Number(arg) = args.get(0).cloned().unwrap() else {
                                panic!()
                            };

                            Ok(Value::Number(arg.cos()))
                        })),
                    );
                }
                "print" => {
                    external_functions.insert(
                        node.function.0,
                        IntrinsicFunctionValue(Rc::new(move |args: &[Value]| {
                            for arg in args {
                                if arg.to_string() == "\\n" {
                                    println!();
                                } else {
                                    if print_colors {
                                        print!("{} ", arg.to_string().replace("\\x1b", "\x1b"));
                                    } else {
                                        print!("{} ", arg.to_string())
                                    }
                                }
                            }
                            Ok(Value::Unit)
                        })),
                    );
                }
                _ => unimplemented!("{function}"),
            }
        }

        Ok(Value::Package(PackageValue {
            identifier: node.package.0.clone(),
            functions,
            packages,
            external_functions,
        }))
    }
}
