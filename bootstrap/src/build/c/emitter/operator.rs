use crate::build::c;
use crate::build::c::emitter::Emitter;
use crate::common::node::CompareOperator;

impl Emitter {
    pub(crate) fn compare(&mut self, node: &c::CompareExpression) {
        self.expression(node.left.as_ref());
        match node.operator {
            CompareOperator::Equal => self.str("=="),
            CompareOperator::NotEqual => self.str("!="),
            CompareOperator::GreaterThan => self.str(">"),
            CompareOperator::GreaterThanEqual => self.str(">="),
            CompareOperator::LessThan => self.str("<"),
            CompareOperator::LessThanEqual => self.str("<=")
        }
        self.expression(node.right.as_ref());
    }
}