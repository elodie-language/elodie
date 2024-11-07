use crate::ast::{CallArg, CallExpression, Expression, LetExpression, Literal};
use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;

impl Interpreter {

    pub(crate) fn interpret_expression(&mut self, expr: &Expression) -> crate::interpreter::Result<Value> {
        let value = match expr {
            Expression::Literal(lit) => self.interpret_literal(lit)?,
            Expression::Identifier(name) => {
                self.scope.get(name.0.as_str()).unwrap().clone()
            }
            Expression::Call(call_expr) => self.interpret_call(call_expr)?,
            Expression::Let(let_expr) => self.interpret_let_expression(let_expr)?,
            _ => unimplemented!()
        };
        Ok(value)
    }

    pub(crate) fn interpret_literal(&self, literal: &Literal) -> crate::interpreter::Result<Value> {
        match literal {
            Literal::Number(f) => Ok(Value::Number(f.clone())),
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Boolean(v) => Ok(Value::Bool(v.clone()))
        }
    }

    pub(crate) fn interpret_call(&mut self, call: &CallExpression) -> crate::interpreter::Result<Value> {
        let mut args: Vec<Value> = Vec::with_capacity(call.arguments.len());
        for arg in &call.arguments {
            args.push(self.interpret_call_arg(arg)?); // Now we can mutably borrow `self` without conflict
        }

        // Step 1: Retrieve the function reference early to avoid overlapping borrows
        let function = if let Expression::PropertyAccess(ref access) = *call.expression {
            if let Some(boxed_expression) = &access.lhs {
                if let Expression::Identifier(object) = boxed_expression.as_ref() {
                    if let Some(Value::Object(object)) = self.scope.get(object.0.as_str()).as_ref() {
                        if let Expression::Identifier(function) = access.rhs.as_ref() {
                            if let Some(Value::Function(func)) = object.get_property(function.0.as_str()) {
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

        // Step 2: If we found the function, interpret arguments and invoke it
        if let Some(function) = function {

            // Call the function
            return Ok(function.0(&args));
        }

        todo!()
    }

    fn interpret_call_arg(&mut self, arg: &CallArg) -> crate::interpreter::Result<Value> {
        self.interpret_expression(arg.value.as_ref())
    }

    pub(crate) fn interpret_let_expression(&mut self, expr: &LetExpression) -> crate::interpreter::Result<Value> {
        let name = expr.name.0.as_str();

        let value = self.interpret_expression(&expr.value)?;

        self.scope.insert(name, value);

        Ok(Value::Unit)
    }
}