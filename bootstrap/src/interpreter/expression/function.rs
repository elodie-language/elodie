use std::ops::Deref;

use crate::ast::{FunctionDeclarationExpression, ReturnExpression};
use crate::interpreter::Interpreter;
use crate::interpreter::value::{Function, Value};

impl Interpreter {
    pub(crate) fn function_declaration(&mut self, expr: &FunctionDeclarationExpression) -> crate::interpreter::Result<Value> {
        let name = expr.name.clone().unwrap().0;

        let body = expr.body.clone();

        let f = Value::Function(Function { body });

        self.scope.insert(name, f.clone());
        // self.scope.insert()
        return Ok(f.clone());
    }

    pub(crate) fn r#return(&mut self, expr: &ReturnExpression) -> crate::interpreter::Result<Value> {
        if let Some(ref expr) = expr.expr {
            return self.interpret_expression(expr.deref());
        } else {
            return Ok(Value::Unit);
        }
    }
}