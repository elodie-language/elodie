use crate::backend::build::c;
use crate::backend::build::c::emitter::Emitter;
use crate::backend::build::c::{
    LiteralBooleanExpression, LiteralDoubleExpression, LiteralExpression, LiteralIntExpression,
    LiteralStringExpression,
};

impl Emitter {
    pub(crate) fn emit_literal(&mut self, node: &c::LiteralExpression) {
        match node {
            LiteralExpression::Bool(LiteralBooleanExpression { indent, value }) => {
                self.emit_str(value.to_string().as_str())
            }
            LiteralExpression::Double(LiteralDoubleExpression { indent, value }) => {
                self.emit_str(value.to_string().as_str())
            }
            LiteralExpression::Int(LiteralIntExpression { indent, value }) => {
                self.emit_str(value.to_string().as_str())
            }
            LiteralExpression::String(LiteralStringExpression { indent, value }) => {
                self.emit_str("\"");
                self.emit_str(value);
                self.emit_str("\"");
            }
        }
    }
}
