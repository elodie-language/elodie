use crate::build::c;
use crate::build::c::{LiteralBooleanExpression, LiteralExpression, LiteralFloat4Expression, LiteralFloat8Expression, LiteralInt16Expression, LiteralInt1Expression, LiteralInt2Expression, LiteralInt4Expression, LiteralInt8Expression, LiteralStringExpression, LiteralUint16Expression, LiteralUint1Expression, LiteralUint2Expression, LiteralUint4Expression, LiteralUint8Expression};
use crate::build::c::emitter::Emitter;

impl Emitter {
    pub(crate) fn literal(&mut self, node: &c::LiteralExpression) {
        match node {
            LiteralExpression::Bool(LiteralBooleanExpression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Float4(LiteralFloat4Expression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Float8(LiteralFloat8Expression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Int1(LiteralInt1Expression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Int2(LiteralInt2Expression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Int4(LiteralInt4Expression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Int8(LiteralInt8Expression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Int16(LiteralInt16Expression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Uint1(LiteralUint1Expression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Uint2(LiteralUint2Expression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Uint4(LiteralUint4Expression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Uint8(LiteralUint8Expression { value }) => {
                self.str(value.to_string().as_str())
            }
            LiteralExpression::Uint16(LiteralUint16Expression { value }) => {
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
