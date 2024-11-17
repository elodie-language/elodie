use std::collections::HashMap;

use crate::ast::{CallParameter, CallExpression, Expression};
use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;

impl Interpreter {
    pub(crate) fn interpret_call(&mut self, call: &CallExpression) -> crate::interpreter::Result<Value> {
        let mut args: Vec<Value> = Vec::with_capacity(call.arguments.len());
        for arg in &call.arguments {
            args.push(self.interpret_call_arg(arg)?); // Now we can mutably borrow `self` without conflict
        }
// builtin attached to object
        let function = if let Expression::PropertyAccess(ref access) = *call.expression {
            if let Some(boxed_expression) = &access.lhs {
                if let Expression::Identifier(object) = boxed_expression.as_ref() {
                    if let Some(Value::Object(object)) = self.scope.get(object.0.as_str()).as_ref() {
                        if let Expression::Identifier(function) = access.rhs.as_ref() {
                            if let Some(Value::BuiltinFunction(func)) = object.get_property(function.0.as_str()) {
                                Some(func)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(function) = function {
            return function.0(&args);
        }

// function
        self.reset_interrupt();
        let function = if let Expression::Identifier(ref identifier) = *call.expression {
            if let Some(Value::Function(func)) = self.scope.get(identifier.0.as_str()) {
                func.clone()
            } else {
                todo!()
            }
        } else { todo!() };

        let mut args = HashMap::with_capacity(call.arguments.len());
        let mut counter = 0;
        for arg in &call.arguments {
            let parameter = function.parameters.get(counter).unwrap();

            let name = parameter.name.0.clone();
            // FIXME resolve  name from definition
            args.insert(name, self.interpret_call_arg(arg)?);
            counter += 1;
        }

        self.scope.enter();
        for arg in &args {
            self.scope.insert(arg.0, arg.1.clone())
        }
        let result = self.interpret_block_expression(&function.body);
        self.scope.leave();

        self.reset_interrupt();
        result
    }

    fn interpret_call_arg(&mut self, arg: &CallParameter) -> crate::interpreter::Result<Value> {
        self.interpret_expression(arg.value.as_ref())
    }
}