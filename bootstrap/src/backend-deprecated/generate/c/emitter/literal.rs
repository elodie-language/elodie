use crate::backend::generate::c;
use crate::backend::generate::c::emitter::Emitter;
use crate::backend::generate::c::{
    LiteralBooleanExpression, LiteralDoubleExpression, LiteralExpression, LiteralIntExpression,
    LiteralStringExpression,
};

impl Emitter {
    pub(crate) fn literal(&mut self, node: &c::LiteralExpression) {
        match node {
            LiteralExpression::Bool(LiteralBooleanExpression { indent, value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Double(LiteralDoubleExpression { indent, value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Int(LiteralIntExpression { indent, value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::String(LiteralStringExpression { indent, value }) => {
                self.str("\"");
                self.str(value);
                self.str("\"");
            }
        }
    }
}
