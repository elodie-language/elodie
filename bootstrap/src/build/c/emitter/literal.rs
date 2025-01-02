use crate::build::c;
use crate::build::c::{
    LiteralBooleanExpression, LiteralDoubleExpression, LiteralExpression, LiteralIntExpression,
    LiteralStringExpression,
};
use crate::build::c::emitter::Emitter;

impl Emitter {
    pub(crate) fn literal(&mut self, node: &c::LiteralExpression) {
        match node {
            LiteralExpression::Bool(LiteralBooleanExpression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Double(LiteralDoubleExpression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Int(LiteralIntExpression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::String(LiteralStringExpression { value }) => {
                self.str("\"");
                self.str(value);
                self.str("\"");
            }
        }
    }
}
