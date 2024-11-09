use crate::ast::{Expression, Literal};
use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;

mod binary;
mod block;
mod call;
mod function;
mod r#if;
mod r#let;
mod r#loop;

impl Interpreter {
    pub(crate) fn interpret_expression(&mut self, expr: &Expression) -> crate::interpreter::Result<Value> {
        let value = match expr {
            Expression::Continue(continue_expr) => self.interpret_continue_expression(continue_expr)?,
            Expression::Break(break_expr) => self.interpret_break_expression(break_expr)?,
            Expression::Binary(bin_expr) => self.interpret_binary_expression(bin_expr)?,
            Expression::Block(block_expr) => self.interpret_block_expression(block_expr)?,
            Expression::FunctionDeclaration(f_expr) => self.function_declaration(f_expr)?,
            Expression::Literal(lit) => self.interpret_literal(lit)?,
            Expression::Identifier(name) => {
                self.scope.get(name.0.as_str()).unwrap().clone()
            }
            Expression::Call(call_expr) => self.interpret_call(call_expr)?,
            Expression::Let(let_expr) => self.interpret_let_expression(let_expr)?,
            Expression::If(if_expr) => self.interpret_if(if_expr)?,
            Expression::Loop(loop_expr) => self.interpret_loop_expression(loop_expr)?,
            Expression::Return(return_expr) => self.r#return(return_expr)?,
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