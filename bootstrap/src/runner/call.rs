use std::collections::HashMap;

use crate::ast::CallFunctionNode;
use crate::runner::Runner;
use crate::runner::value::{FunctionValue, Value};

impl Runner {
    pub(crate) fn run_node_call_function(&mut self, node: &CallFunctionNode) -> crate::runner::Result<Value> {
        self.reset_interrupt();

        let function = if let Some(Value::Function(func)) = self.scope.get_value(node.function.0.as_str()) {
            func.clone()
        } else {
            todo!()
        };

        let mut args: Vec<Value> = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            args.push(self.run_node(arg)?); // Now we can mutably borrow `self` without conflict
        }

        let mut args = HashMap::with_capacity(node.arguments.len());
        let mut counter = 0;

        for arg in &node.arguments {
            let arg_node = function.arguments.get(counter).unwrap();

            let name = arg_node.identifier.0.clone();
            // FIXME resolve  name from definition
            args.insert(name, self.run_node(arg)?);
            counter += 1;
        }

        self.scope.enter();
        for arg in &args {
            self.scope.insert_value(arg.0, arg.1.clone())
        }
        let result = self.run_block(&function.body);
        self.scope.leave();

        self.reset_interrupt();
        result
    }

    pub(crate) fn run_node_call(&mut self, function_value: FunctionValue, arguments: HashMap<String, Value>) -> crate::runner::Result<Value> {
        self.reset_interrupt();

        self.scope.enter();
        for arg in &arguments {
            self.scope.insert_value(arg.0, arg.1.clone())
        }
        let result = self.run_block(&function_value.body);
        self.scope.leave();

        self.reset_interrupt();
        result
    }
}