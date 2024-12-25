use crate::build::c::emitter::Emitter;
use crate::build::c::{InfixExpression, InfixOperator};

impl Emitter {
    pub(crate) fn emit_infix(&mut self, expression: &InfixExpression) {
        self.emit_expression(&expression.left);
        match expression.operator {
            InfixOperator::Add => self.emit_token("+"),
            InfixOperator::Subtract => unimplemented!(),
            InfixOperator::Multiply => unimplemented!(),
            InfixOperator::Divide => unimplemented!(),
            InfixOperator::Modulo => unimplemented!(),
            InfixOperator::Equal => self.emit_token("=="),
            InfixOperator::NotEqual => self.emit_token("!="),
            InfixOperator::LessThan => unimplemented!(),
            InfixOperator::GreaterThan => unimplemented!(),
            InfixOperator::Assign => unimplemented!(),
        }

        self.emit_expression(&expression.right);
    }
}
