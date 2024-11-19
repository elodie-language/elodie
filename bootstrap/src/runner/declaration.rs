use std::ops::Deref;

use crate::ast::{DeclareFunctionNode, DeclareVariableNode};
use crate::runner::Runner;
use crate::runner::value::{Function, Value};

impl Runner {
    pub(crate) fn run_variable_declaration(&mut self, node: &DeclareVariableNode) -> crate::runner::Result<Value> {
        let name = node.identifier.0.clone();
        let value = self.run_node(node.value.deref())?;
        self.scope.insert(name, value);
        Ok(Value::Unit)
    }

    pub(crate) fn run_function_declaration(&mut self, node: &DeclareFunctionNode) -> crate::runner::Result<Value> {
        let name = node.identifier.0.clone();

        let mut arguments = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            arguments.push(arg.clone())
        }

        let f = Value::Function(Function {
            body: node.body.clone(),
            arguments,
        });

        self.scope.insert(name, f);
        Ok(Value::Unit)
    }
}