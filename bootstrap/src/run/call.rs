use std::collections::HashMap;

use crate::common::StringCacheIdx;
use crate::ir::{CallFunctionNode, CallFunctionWithLambdaNode};
use crate::run::Runner;
use crate::run::value::{FunctionValue, Value};

impl<'a> Runner<'a> {
    pub(crate) fn run_node_call_function(&mut self, node: &CallFunctionNode) -> crate::run::Result<Value> {
        self.reset_interrupt();

        let function = if let Some(Value::Function(func)) = self.scope.get_value(&node.function.0) {
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
            self.scope.insert_value(arg.0.clone(), arg.1.clone())
        }
        let result = self.run_block(&function.body);
        self.scope.leave();

        self.reset_interrupt();
        result
    }

    pub(crate) fn run_node_call_function_with_lambdad(&mut self, node: &CallFunctionWithLambdaNode) -> crate::run::Result<Value> {
        self.reset_interrupt();

        let function = if let Some(Value::Function(func)) = self.scope.get_value(&node.call_function.function.0) {
            func.clone()
        } else {
            todo!()
        };

        let node_arguments = &node.call_function.arguments;

        let mut args: Vec<Value> = Vec::with_capacity(node_arguments.len());
        for arg in node_arguments {
            args.push(self.run_node(arg)?); // Now we can mutably borrow `self` without conflict
        }

        let mut args = HashMap::with_capacity(node_arguments.len());
        let mut counter = 0;

        for arg in node_arguments {
            let arg_node = function.arguments.get(counter).unwrap();

            let name = arg_node.identifier.0.clone();
            // FIXME resolve  name from definition
            args.insert(name, self.run_node(arg)?);
            counter += 1;
        }

        // last parameter is lambda function
        let lambda_function = Value::Function(FunctionValue {
            arguments: vec![],
            body: node.lambda.clone(),
        });

        args.insert(function.arguments.last().unwrap().identifier.0, lambda_function);


        self.scope.enter();
        for arg in &args {
            self.scope.insert_value(arg.0.clone(), arg.1.clone())
        }
        let result = self.run_block(&function.body);
        self.scope.leave();

        self.reset_interrupt();
        result
    }

    pub(crate) fn run_node_call(&mut self, function_value: FunctionValue, arguments: HashMap<StringCacheIdx, Value>) -> crate::run::Result<Value> {
        self.reset_interrupt();

        self.scope.enter();
        for arg in &arguments {
            self.scope.insert_value(arg.0.clone(), arg.1.clone())
        }
        let result = self.run_block(&function_value.body);
        self.scope.leave();

        self.reset_interrupt();
        result
    }
}