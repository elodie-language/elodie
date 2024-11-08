use crate::ast::BlockExpression;
use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;

impl Interpreter {
    pub(crate) fn interpret_block_expression(&mut self, expr: &BlockExpression) -> crate::interpreter::Result<Value> {
        let mut value = Value::Unit;

        for expr in &expr.body {
            value = self.interpret_expression(expr)?;
        }

        Ok(value)
    }
}