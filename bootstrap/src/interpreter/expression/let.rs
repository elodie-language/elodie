use crate::ast::LetExpression;
use crate::interpreter::Interpreter;
use crate::interpreter::value::Value;

impl Interpreter {

    pub(crate) fn interpret_let_expression(&mut self, expr: &LetExpression) -> crate::interpreter::Result<Value> {
        let name = expr.name.0.as_str();
        let value = self.interpret_expression(&expr.value)?;
        self.scope.insert(name, value);
        Ok(Value::Unit)
    }

}