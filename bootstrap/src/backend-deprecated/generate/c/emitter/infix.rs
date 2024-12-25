use crate::backend::generate::c::emitter::Emitter;
use crate::backend::generate::c::{InfixExpression, InfixOperator};

impl Emitter {
    pub(crate) fn infix(&mut self, expression: &InfixExpression) {
        self.expression(&expression.left);
        match expression.operator {
            InfixOperator::Add => self.token("+"),
            InfixOperator::Subtract => unimplemented!(),
            InfixOperator::Multiply => unimplemented!(),
            InfixOperator::Divide => unimplemented!(),
            InfixOperator::Modulo => unimplemented!(),
            InfixOperator::Equal => self.token("=="),
            InfixOperator::NotEqual => self.token("!="),
            InfixOperator::LessThan => unimplemented!(),
            InfixOperator::GreaterThan => unimplemented!(),
            InfixOperator::Assign => unimplemented!(),
        }

        self.expression(&expression.right);
    }
}
