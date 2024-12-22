use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;

use crate::{frontend, load_library_file};
use crate::backend::run::scope::Scope;
use crate::backend::run::type_definitions::TypeDefinitions;
use crate::backend::run::value::{IntrinsicFunctionValue, ListValue, ObjectValue, Value};
use crate::backend::run::value::Value::{IntrinsicFunction, Unit};
use crate::common::context::Context;
use crate::common::node::{CalculationOperator, CompareOperator, Node};
use crate::frontend::{ast, ast_from_str};
use crate::frontend::ast::AstTreeNode;
use crate::ir::TypeId;

mod block;
mod call;
mod declaration;
mod r#if;
mod r#loop;
pub mod scope;
pub mod type_definitions;
pub mod value;

#[derive(Debug)]
pub enum Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub struct Runner<'a> {
    ctx: &'a mut Context,
    scope: Scope,
    pub interrupt: Option<Interrupt>,
    type_definitions: TypeDefinitions,
    pub print_colors: bool,
}

#[derive(Debug, Clone)]
pub enum Interrupt {
    Break(Value),
    Continue,
    Return(Value),
}

pub fn run_file(file: &String, print_colors: bool) {
    fn load_text_from_file(path: &str) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    let mut ctx = Context::new();
    let mut root_values = HashMap::new();
    let mut root_types = HashMap::new();

    // root_values.insert(ctx.string_table.insert("ec_io_print"), IntrinsicFunction(IntrinsicFunctionValue(Rc::new(|args: &[Value]| {
    //     for arg in args {
    //         if arg.to_string() == "\\n" {
    //             println!();
    //         } else {
    //             print!("{} ", arg.to_string().replace("\\x1b", "\x1b"));
    //         }
    //     }
    //     Ok(Value::Unit)
    // }))));

    let mut intrinsics = ObjectValue::new();
    intrinsics.set_property(
        ctx.string_table.push_str("print"),
        IntrinsicFunction(IntrinsicFunctionValue(Rc::new(|args: &[Value]| {
            for arg in args {
                if arg.to_string() == "\\n" {
                    println!();
                } else {
                    print!("{} ", arg.to_string().replace("\\x1b", "\x1b"));
                }
            }
            Ok(Value::Unit)
        }))),
    );

    intrinsics.set_property(
        ctx.string_table.push_str("list_length"),
        IntrinsicFunction(IntrinsicFunctionValue(Rc::new(|args| {
            let Value::List(list) = args.get(0).unwrap() else {
                panic!("not list")
            };
            let len: u32 = list.0.borrow().len() as u32;
            Ok(Value::Number(len.into()))
        }))),
    );

    intrinsics.set_property(
        ctx.string_table.push_str("list_append"),
        IntrinsicFunction(IntrinsicFunctionValue(Rc::new(|args| {
            let Value::List(list) = args.get(0).unwrap() else {
                panic!("not list")
            };
            let arg = args.get(1).cloned().unwrap();
            list.0.borrow_mut().push(arg);
            Ok(Value::Unit)
        }))),
    );

    intrinsics.set_property(
        ctx.string_table.push_str("list_get"),
        IntrinsicFunction(IntrinsicFunctionValue(Rc::new(|args| {
            let Value::List(list) = args.get(0).unwrap() else {
                panic!("not list")
            };
            let Value::Number(arg) = args.get(1).cloned().unwrap() else {
                panic!("not a number")
            };
            Ok(list.0.borrow().get(arg as usize - 1).cloned().unwrap())
        }))),
    );

    intrinsics.set_property(
        ctx.string_table.push_str("exit"),
        IntrinsicFunction(IntrinsicFunctionValue(Rc::new(|args| {
            let Value::Number(code) = args.get(0).cloned().unwrap() else {
                panic!("not a number")
            };
            exit(code as i32)
        }))),
    );

    root_values.insert(
        ctx.string_table.push_str("intrinsics"),
        Value::Object(intrinsics),
    );
    let scope = Scope::new(root_values, root_types);

    let (scope, definitions) = {
        let std_content = load_library_file("core/index.ec").unwrap();
        let std_file = ast_from_str(&mut ctx, std_content.as_str()).unwrap();
        run(
            &mut ctx,
            scope,
            TypeDefinitions {
                definitions: Default::default(),
            },
            std_file,
            true,
        )
            .unwrap()
    };

    let (scope, definitions) = {
        let std_content = load_library_file("std/index.ec").unwrap();
        let std_file = ast_from_str(&mut ctx, std_content.as_str()).unwrap();
        run(&mut ctx, scope, definitions, std_file, true).unwrap()
    };

    let mut path = PathBuf::from(file);
    let content = load_text_from_file(path.to_str().unwrap()).unwrap();
    let source_file = ast_from_str(&mut ctx, content.as_str()).unwrap();

    run(&mut ctx, scope, definitions, source_file, true).unwrap();
}

pub fn run(
    ctx: &mut Context,
    scope: Scope,
    definitions: TypeDefinitions,
    file: frontend::Ast,
    print_colors: bool,
) -> Result<(Scope, TypeDefinitions)> {
    let mut runner = Runner::new(ctx, scope, definitions, print_colors);
    runner.run(file)?;
    Ok((runner.scope, runner.type_definitions))
}

impl<'a> Runner<'a> {
    pub(crate) fn new(
        ctx: &'a mut Context,
        scope: Scope,
        definitions: TypeDefinitions,
        print_colors: bool,
    ) -> Self {
        Self {
            ctx,
            scope,
            interrupt: None,
            type_definitions: definitions,
            print_colors,
        }
    }

    pub fn run(&mut self, source_file: frontend::Ast) -> Result<Value> {
        for node in &source_file.nodes {
            self.run_node(node)?;
        }
        Ok(Value::Unit)
    }

    pub(crate) fn run_node(&mut self, node: &AstTreeNode) -> Result<Value> {
        match node.node() {
            Node::BreakLoop(break_node) => self.run_break(break_node),

            Node::DeclareVariable(declaration) => self.run_variable_declaration(declaration),
            Node::DeclareFunction(declaration) => self.run_function_declaration(declaration),
            Node::DeclarePackage(declaration) => {
                let value = self.run_package_declaration(declaration)?;
                let Value::Package(package) = value else {
                    panic!()
                };
                self.scope
                    .insert_value(package.identifier.clone(), Value::Package(package));
                Ok(Value::Unit)
            }

            Node::CallFunctionOfObject(ast::AstCallFunctionOfObjectNode {
                                           object,
                                           function,
                                           arguments,
                                           ..
                                       }) => {
                // let some_arg_value = if let Node::CallFunction(arg_1) = &arguments[0] {
                //     let value = self.run_call_function(arg_1)?.clone();
                //     Some(value)
                // } else {
                //     None
                // };
                //
                // let mut direct_args = Vec::with_capacity(arguments.len());
                // for arg in arguments {
                //     direct_args.push(self.run_node(arg)?);/
                // }

                let obj_name = self.ctx.str_get(object.0).to_string();

                let mut args: Vec<Value> = Vec::with_capacity(arguments.len());
                for arg in arguments {
                    args.push(self.run_node(arg)?); // Now we can mutably borrow `self` without conflict
                }

                if let Value::List(object) = self.scope.get_value(&object.0).unwrap() {
                    let mut args = HashMap::with_capacity(arguments.len());
                    args.insert(
                        self.ctx.string_table.push_str("self"),
                        Value::List(object.clone()),
                    );

                    let mut counter = 0;

                    let func =
                        if let Some(Value::Function(func)) = self.scope.get_value(&function.0) {
                            func.clone()
                        } else {
                            todo!()
                        };

                    for arg in arguments {
                        let arg_node = func.arguments.get(counter).unwrap();

                        let name = arg_node.argument.0;
                        // FIXME resolve  name from definition
                        args.insert(name, self.run_node(arg)?);
                        counter += 1;
                    }

                    // let mut args = HashMap::with_capacity(arguments.len());

                    // args.extend(&direct_args);

                    let func = self.type_definitions.get_function(&TypeId(99), &function.0);
                    self.scope.enter();

                    let result = self.run_node_call(func.clone(), args);

                    self.scope.leave();

                    return result;
                }

                let Value::Object(object) = self.scope.get_value(&object.0).unwrap() else {
                    panic!()
                };

                // FIXME
                if obj_name == "intrinsics" {
                    // println!("{}", self.ctx.get_str(function.0));
                    let func = object
                        .get_property_host_function(&function.clone())
                        .unwrap();

                    let mut args = Vec::with_capacity(arguments.len());
                    for arg in arguments {
                        if let Node::AccessVariable(load_varialbe_node) = arg.node() {
                            let value = self
                                .scope
                                .get_value(&load_varialbe_node.variable.0)
                                .unwrap()
                                .clone();
                            args.push(value);
                        } else if let Node::LiteralString(node) = arg.node() {
                            args.push(Value::String(self.ctx.str_get(node.0.value).to_string()));
                        } else if let Node::LiteralNumber(node) = arg.node() {
                            args.push(Value::Number(
                                self.ctx.str_get(node.0.value()).parse().unwrap(),
                            ));
                        } else {
                            unimplemented!("{:#?}", arg);
                        }
                    }

                    return func.0(args.as_slice());
                } else {
                    let mut args = HashMap::with_capacity(arguments.len());
                    args.insert(
                        self.ctx.string_table.push_str("self"),
                        Value::Object(object.clone()),
                    );

                    let func = self.type_definitions.get_function(&TypeId(99), &function.0);
                    self.scope.enter();

                    let result = self.run_node_call(func.clone(), args);

                    self.scope.leave();

                    return result;
                };
            }

            Node::CallFunctionOfPackage(ast::AstCallFunctionOfPackageNode {
                                            package: packages,
                                            function,
                                            arguments,
                                            ..
                                        }) => {
                let mut args = HashMap::with_capacity(arguments.len());

                let mut packages = packages.clone();
                let mut root = packages.first().unwrap();
                let Value::Package(root_package) = self.scope.get_value(&root).unwrap().clone()
                    else {
                        panic!()
                    };

                let mut target_package = root_package;
                loop {
                    packages = packages.pop();
                    if let Some(p) = packages.first() {
                        target_package = match target_package.packages.get(&p) {
                            None => {
                                panic!(
                                    "package {} not found in {}",
                                    self.ctx.str_get(p),
                                    self.ctx.str_get(root)
                                )
                            }
                            Some(p) => p.clone(),
                        };
                        root = p;
                    } else {
                        break;
                    }
                }

                if let Some(IntrinsicFunctionValue(func)) =
                    target_package.get_intrinsic_function(function.0)
                {
                    let mut args = Vec::with_capacity(arguments.len());
                    for arg in arguments {
                        if let Node::AccessVariable(load_varialbe_node) = arg.node() {
                            let value = self
                                .scope
                                .get_value(&load_varialbe_node.variable.0)
                                .unwrap()
                                .clone();
                            args.push(value);
                        } else if let Node::LiteralString(node) = arg.node() {
                            args.push(Value::String(self.ctx.str_get(node.0.value).to_string()));
                        } else if let Node::LiteralNumber(node) = arg.node() {
                            args.push(Value::Number(
                                self.ctx.str_get(node.0.value()).parse().unwrap(),
                            ))
                        } else {
                            unimplemented!("{:#?}", arg);
                        }
                    }

                    return func(args.as_slice());
                }

                //FIXME recursively get package
                // let target_package = if package.len() == 1 {
                //     &root_package
                // } else {
                //     root_package.packages.get(&package.last().unwrap().0).unwrap()
                // };

                let func = target_package.get_function(function.0).unwrap();

                // makes sure that a package can access its internal functions
                self.scope.enter();
                for (key, value) in &target_package.functions {
                    self.scope
                        .insert_value(key.clone(), Value::Function(value.clone()))
                }

                let mut counter = 0;
                for arg in arguments {
                    let arg_node = func.arguments.get(counter).unwrap();
                    let name = arg_node.argument.0.clone();
                    args.insert(name, self.run_node(arg)?);
                    counter += 1;
                }

                for (key, value) in &target_package.functions {
                    self.scope
                        .insert_value(key.clone(), Value::Function(value.clone()))
                }

                let result = self.run_node_call(func.clone(), args);

                self.scope.leave();

                result
            }

            Node::CallFunction(function_node) => self.run_node_call_function(function_node),
            Node::CallFunctionWithLambda(lambda) => self.run_node_call_function_with_lambda(lambda),
            Node::ReturnFromFunction(node) => {
                if let Some(node) = &node.node {
                    let value = self.run_node(node)?;
                    self.interrupt(Interrupt::Return(value.clone()));
                    Ok(value)
                } else {
                    self.interrupt(Interrupt::Return(Value::Unit));
                    Ok(Unit)
                }
            }
            Node::LiteralString(node) => {
                Ok(Value::String(self.ctx.str_get(node.0.value).to_string()))
            }
            Node::LiteralNumber(node) => Ok(Value::Number(
                self.ctx.str_get(node.0.value).parse().unwrap(),
            )),
            Node::LiteralBoolean(node) => Ok(Value::Bool(self.ctx.str_get(node.0.value) == "true")),

            Node::Loop(loop_node) => self.run_loop(loop_node),
            Node::If(if_node) => self.run_if(if_node),

            Node::Block(block_node) => self.run_block(block_node),

            Node::Compare(compare_node) => {
                let left = self.run_node(compare_node.left.deref())?;
                let right = self.run_node(compare_node.right.deref())?;

                if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                    return match compare_node.operator {
                        CompareOperator::GreaterThan => Ok(Value::Bool(l > r)),
                        CompareOperator::Equal => Ok(Value::Bool(l == r)),
                        CompareOperator::NotEqual => Ok(Value::Bool(l != r)),
                    };
                }

                if let (Value::Bool(l), Value::Bool(r)) = (&left, &right) {
                    return match compare_node.operator {
                        CompareOperator::GreaterThan => Ok(Value::Bool(l > r)),
                        CompareOperator::Equal => Ok(Value::Bool(l == r)),
                        CompareOperator::NotEqual => Ok(Value::Bool(l != r)),
                    };
                }

                unimplemented!()
            }

            Node::Calculate(calculation_node) => {
                let left = self.run_node(calculation_node.left.deref())?;
                let right = self.run_node(calculation_node.right.deref())?;

                if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                    return match calculation_node.operator {
                        CalculationOperator::Multiply => Ok(Value::Number(l * r)),
                        CalculationOperator::Add => Ok(Value::Number(l + r)),
                    };
                }

                if let (Value::String(l), Value::String(r)) = (&left, &right) {
                    return match calculation_node.operator {
                        CalculationOperator::Add => Ok(Value::String(l.clone() + r)),
                        _ => todo!(),
                    };
                }

                unimplemented!()
            }
            Node::AccessVariable(load_variable) => {
                let value = self
                    .scope
                    .get_value(&load_variable.variable.0)
                    .unwrap()
                    .clone();
                Ok(value)
            }
            Node::AccessVariableOfObject(load) => {
                let value = self.scope.get_value(&load.object.0).unwrap().clone();
                let Value::Object(object_value) = value else {
                    panic!("not object")
                };
                Ok(object_value
                    .get_property(&load.variable.0)
                    .cloned()
                    .unwrap())
            }
            Node::DeclareType(decl) => {
                // let mut properties = HashMap::new();
                //
                // for prop in &decl.variables {
                //     properties.insert(prop.variable.0, TypeVariable {});
                // }
                //
                //
                // let name = TypeName(decl.r#type.0);
                //
                //
                // // let r#type = Type {
                // //     id: TypeId(0),
                // //     name,
                // //     variables: properties,
                // // };
                //
                // self.scope.insert_type(decl.r#type.0, r#type);
                Ok(Value::Unit)
            }
            Node::InstantiateType(node) => {
                let mut properties = HashMap::with_capacity(node.arguments.len());

                for arg in &node.arguments {
                    properties.insert(arg.identifier.0, self.run_node(&arg.value)?);
                }

                let type_name = self.ctx.str_get(node.r#type.0);

                // FIXME dirty hack to make lists works as quick as possible
                if type_name == "List" {
                    return Ok(Value::List(ListValue(Rc::new(RefCell::new(vec![])))));
                }

                let obj = Value::Object(ObjectValue { properties });

                // self.scope.insert_value(node.identifier.0.to_string(), obj.clone());

                Ok(obj)
            }
            Node::DefineType(node) => {
                let func_ident = node.functions.get(0).unwrap().function.0;
                let func = node.functions.get(0).unwrap().clone();
                let value = self.run_function_declaration(&func)?;

                let Value::Function(func) = value else {
                    panic!()
                };
                self.type_definitions
                    .add_function(TypeId(99), func_ident, func);

                Ok(Value::Unit)
            }
            Node::AccessVariableOfSelf(load_variable) => {
                let value = self
                    .scope
                    .get_value(&self.ctx.string_table.push_str("self"))
                    .unwrap()
                    .clone();
                let Value::Object(object_value) = value else {
                    panic!("not object")
                };
                Ok(object_value
                    .get_property(&load_variable.variable.0)
                    .cloned()
                    .unwrap())
            }
            Node::InterpolateString(node) => {
                let mut result = String::new();
                for node in &node.nodes {
                    result += self.run_node(node)?.to_string().as_str()
                }
                Ok(Value::String(result))
            }
            _ => unimplemented!("{:?}", node),
        }
    }

    pub fn interrupt(&mut self, loop_interrupt: Interrupt) {
        self.interrupt = Some(loop_interrupt)
    }

    pub fn reset_interrupt(&mut self) {
        self.interrupt = None
    }
}
