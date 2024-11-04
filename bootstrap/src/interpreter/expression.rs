use crate::core::ast::{CallArg, CallExpression, Expression, Literal};
use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;

impl Interpreter {
    pub(crate) fn interpret_expression(&self, expr: &Expression) -> crate::interpreter::Result<Value> {
        let value = match expr {
            Expression::Literal(lit) => self.interpret_literal(lit)?,
            Expression::Identifier(name) => {
                panic!("Undefined identifier: {}", name);
            }
            Expression::Call(call_expr) => self.interpret_call(call_expr)?,
            _ => unimplemented!()
        };
        Ok(value)
    }

    pub(crate) fn interpret_literal(&self, literal: &Literal) -> crate::interpreter::Result<Value> {
        match literal {
            Literal::Number(f) => Ok(Value::Number(f.clone())),
            Literal::String(s) => Ok(Value::String(s.clone())),
            _ => unimplemented!()
        }
    }

    pub(crate) fn interpret_call(&self, call: &CallExpression) -> crate::interpreter::Result<Value> {
        if let Expression::PropertyAccess(ref access) = *call.expression{
            if let Some(boxed_expression) = &access.lhs {
                if let Expression::Identifier(object) = boxed_expression.as_ref() {
                    if let Some(Value::Object(object)) = self.env.get(object){

                        if let boxed_expression = &access.rhs{
                            if let Expression::Identifier(function) = boxed_expression.as_ref(){

                                if let Some(Value::Function(function)) = object.get_property(function){
                                    let mut args: Vec<Value> = Vec::with_capacity(call.arguments.len());
                                    for arg in &call.arguments {
                                        args.push(self.interpret_call_arg(arg)?);
                                    }

                                    // Call the function
                                    return Ok(function.0(&args));
                                }
                            }
                        }
                    }
                }
            }
        }
        todo!()

    }

    fn interpret_call_arg(&self, arg: &CallArg) -> crate::interpreter::Result<Value> {
        self.interpret_expression(arg.value.as_ref())
    }
}