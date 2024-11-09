use crate::ast::BlockExpression;
use crate::interpreter::{Interpreter, Interrupt};
use crate::interpreter::value::Value;

impl Interpreter {
    pub(crate) fn interpret_block_expression(&mut self, expr: &BlockExpression) -> crate::interpreter::Result<Value> {
        let mut value = Value::Unit;
        self.scope.enter();

        for expr in &expr.body {
            if let Some(Interrupt::Return(return_value)) = &self.interrupt {
                return Ok(return_value.clone());
            }

            value = self.interpret_expression(expr)?;
        }

        self.scope.leave();
        Ok(value)
    }
}