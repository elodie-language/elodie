use std::ops::Deref;

use crate::ast::{BinaryExpression, BinaryOperator};
use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::interpreter::value::Value::Bool;

impl Interpreter {
    pub(crate) fn interpret_binary_expression(&mut self, expr: &BinaryExpression) -> crate::interpreter::Result<Value> {
        let left = self.interpret_expression(expr.left.deref())?;
        let right = self.interpret_expression(expr.right.deref())?;

        match expr.operator {
            BinaryOperator::Equal => { Ok(Bool(left == right)) },
            BinaryOperator::NotEqual => { Ok(Bool( left != right))}
            _ => todo!()
        }
    }
}