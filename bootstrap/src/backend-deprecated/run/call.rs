use std::collections::HashMap;

use crate::backend::run::value::{FunctionValue, Value};
use crate::backend::run::Runner;
use crate::common::StringTableId;
use crate::frontend::ast::{AStCallFunctionNode, AstCallFunctionWithLambdaNode};

impl<'a> Runner<'a> {
    pub(crate) fn run_node_call_function(
        &mut self,
        node: &AStCallFunctionNode,
    ) -> crate::backend::run::Result<Value> {
        self.reset_interrupt();

        let mut args: Vec<Value> = Vec::with_capacity(node.arguments.len());
        for arg in &node.arguments {
            args.push(self.run_node(arg)?);
        }

        if let Some(Value::IntrinsicFunction(func)) = self.scope.get_value(&node.function.0) {
            return func.0(&args);
        }

        let function = if let Some(Value::Function(func)) = self.scope.get_value(&node.function.0) {
            func.clone()
        } else {
            todo!()
        };

        let mut args = HashMap::with_capacity(node.arguments.len());
        let mut counter = 0;

        for arg in &node.arguments {
            let arg_node = function.arguments.get(counter).unwrap();

            let name = arg_node.argument.0;
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

    pub(crate) fn run_node_call_function_with_lambda(
        &mut self,
        node: &AstCallFunctionWithLambdaNode,
    ) -> crate::backend::run::Result<Value> {
        self.reset_interrupt();

        let function = if let Some(Value::Function(func)) = self.scope.get_value(&node.function.0) {
            func.clone()
        } else {
            todo!()
        };

        let node_arguments = &node.arguments;

        let mut args: Vec<Value> = Vec::with_capacity(node_arguments.len());
        for arg in node_arguments {
            args.push(self.run_node(arg)?); // Now we can mutably borrow `self` without conflict
        }

        let mut args = HashMap::with_capacity(node_arguments.len());
        let mut counter = 0;

        for arg in node_arguments {
            let arg_node = function.arguments.get(counter).unwrap();

            let name = arg_node.argument.0;
            // FIXME resolve  name from definition
            args.insert(name, self.run_node(arg)?);
            counter += 1;
        }

        // last parameter is lambda function
        let lambda_function = Value::Function(FunctionValue {
            arguments: vec![],
            body: node.lambda.clone(),
        });

        args.insert(
            function.arguments.last().unwrap().argument.0,
            lambda_function,
        );

        self.scope.enter();
        for arg in &args {
            self.scope.insert_value(arg.0.clone(), arg.1.clone())
        }
        let result = self.run_block(&function.body);
        self.scope.leave();

        self.reset_interrupt();
        result
    }

    pub(crate) fn run_node_call(
        &mut self,
        function_value: FunctionValue,
        arguments: HashMap<StringTableId, Value>,
    ) -> crate::backend::run::Result<Value> {
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
