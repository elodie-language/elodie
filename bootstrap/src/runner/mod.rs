use std::ops::Deref;

use crate::ast::{CalculationOperator, CallFunctionOfObjectNode, CompareOperator, Node, SourceFile};
use crate::runner::scope::Scope;
use crate::runner::value::Value;

mod scope;
mod value;
mod declaration;
mod r#loop;
mod r#if;
mod block;

#[derive(Debug)]
pub enum Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub struct Runner {
    scope: Scope,
    pub interrupt: Option<Interrupt>,
}

#[derive(Debug, Clone)]
pub enum Interrupt {
    Break(Value),
    Continue,
    Return(Value),
}

impl Runner {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
            interrupt: None,
        }
    }

    pub fn run(&mut self, source_file: SourceFile) -> Result<Value> {
        for node in &source_file.body {
            self.run_node(node)?;
        }

        // println!("{:?}", source_file.body);
        Ok(Value::Unit)
    }

    pub(crate) fn run_node(&mut self, node: &Node) -> Result<Value> {
        match node {
            Node::Break(break_node) => self.run_break(break_node),
            Node::DeclareVariable(declaration) => self.run_variable_declaration(declaration),
            Node::CallFunctionOfObject(CallFunctionOfObjectNode { object, function, arguments }) => {
                let Value::Object(object) = self.scope.get(object.deref()).unwrap() else { panic!() };
                let func = object.get_property_host_function(function).unwrap();

                if let Node::LoadVariable(load_varialbe_node) = &arguments[0] {
                    let value = self.scope.get(load_varialbe_node.identifier.0.as_str()).unwrap();
                    return func.0(&[value]);
                }

                if let Node::ValueString(arg_1) = &arguments[0] {
                    return func.0(&[&Value::String(arg_1.to_string())]);
                }
                unimplemented!()
            }
            Node::ValueString(value) => Ok(Value::String(value.to_string())),
            Node::ValueNumber(value) => Ok(Value::Number(value.clone())),
            Node::ValueBoolean(value) => Ok(Value::Bool(value.clone())),
            Node::Loop(loop_node) => self.run_loop(loop_node),
            Node::If(if_node) => self.run_if(if_node),

            Node::Compare(compare_node) => {
                let left = self.run_node(compare_node.left.deref())?;
                let right = self.run_node(compare_node.right.deref())?;

                if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                    return match compare_node.operator {
                        CompareOperator::GreaterThan => Ok(Value::Bool(l > r))
                    };
                }
                unimplemented!()
            }

            Node::Calculate(calculation_node) => {
                let left = self.run_node(calculation_node.left.deref())?;
                let right = self.run_node(calculation_node.right.deref())?;

                if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                    return match calculation_node.operator {
                        CalculationOperator::Multiply => Ok(Value::Number(l * r))
                    };
                }

                unimplemented!()
            }
            Node::LoadVariable(load_variable) => {
                let value = self.scope.get(load_variable.identifier.0.as_str()).unwrap().clone();
                Ok(value)
            },
            _ => unimplemented!("{:?}", node)
        }
    }

    pub fn interrupt(&mut self, loop_interrupt: Interrupt) {
        self.interrupt = Some(loop_interrupt)
    }

    pub fn reset_interrupt(&mut self) {
        self.interrupt = None
    }
}