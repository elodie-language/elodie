use crate::ast::{Expression, IfExpression};
use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;

impl Interpreter {
    pub(crate) fn interpret_if(&mut self, expr: &IfExpression) -> crate::interpreter::Result<Value> {
        let condition = self.interpret_expression(&expr.condition)?;
        match condition {
            Value::Bool(v) => {
                if v {
                    self.interpret_expression(&Expression::Block(expr.then.clone()))
                } else {
                    if let Some(otherwise) = &expr.otherwise {
                        self.interpret_expression(&Expression::Block(otherwise.clone()))
                    } else {
                        Ok(Value::Unit)
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}