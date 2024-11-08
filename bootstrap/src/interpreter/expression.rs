mod call;
mod r#let;
mod r#if;
mod binary;
mod block;

use std::fmt::format;
use crate::ast::{CallArg, CallExpression, Expression, LetExpression, Literal};
use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;

impl Interpreter {

    pub(crate) fn interpret_expression(&mut self, expr: &Expression) -> crate::interpreter::Result<Value> {
        let value = match expr {
            Expression::Binary(bin_expr) => self.interpret_binary_expression(bin_expr)?,
            Expression::Block(block_expr) => self.interpret_block_expression(block_expr)?,
            Expression::Literal(lit) => self.interpret_literal(lit)?,
            Expression::Identifier(name) => {
                self.scope.get(name.0.as_str()).unwrap().clone()
            }
            Expression::Call(call_expr) => self.interpret_call(call_expr)?,
            Expression::Let(let_expr) => self.interpret_let_expression(let_expr)?,
            Expression::If(if_expr) => self.interpret_if(if_expr)?,
            _ => unimplemented!("{expr:?}")
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



}