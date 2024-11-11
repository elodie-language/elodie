use std::ops::Deref;

use crate::ast::{BinaryExpression, BinaryOperator};
use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::value::Value::{Bool, String};

impl Interpreter {
    pub(crate) fn interpret_binary_expression(&mut self, expr: &BinaryExpression) -> crate::interpreter::Result<Value> {
        let left = self.interpret_expression(expr.left.deref())?;
        let right = self.interpret_expression(expr.right.deref())?;

        match expr.operator {
            BinaryOperator::Add => {
                if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                    return Ok(Value::Number(l + r));
                } else if let (Value::String(l), Value::String(r)) = (&left,&right){
                    return Ok(String(l.clone() + r))
                } else {
                    todo!()
                }
            }
            BinaryOperator::Subtract => {
                if let (Value::Number(l), Value::Number(r)) = (&left, &right) {
                    return Ok(Value::Number(l - r));
                } else {
                    todo!()
                }
            }
            BinaryOperator::Equal => { Ok(Bool(left == right)) }
            BinaryOperator::NotEqual => { Ok(Bool(left != right)) }
            BinaryOperator::GreaterThan => { Ok(Bool(left > right)) }
            BinaryOperator::Multiply => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    return Ok(Value::Number(l * r));
                } else {
                    todo!()
                }
            }
            _ => todo!()
        }
    }
}