use bigdecimal::ToPrimitive;

use crate::build::c::{Indent, LiteralBooleanExpression, LiteralDoubleExpression, LiteralExpression, LiteralStringExpression};
use crate::build::c::generator::Generator;
use crate::ir::{IrLiteralBooleanNode, IrLiteralNumberNode, IrLiteralStringNode};

impl Generator {

    pub(crate) fn literal_bool(
        &mut self,
        node: &IrLiteralBooleanNode,
    ) -> crate::build::c::generator::Result<LiteralExpression> {
        Ok(LiteralExpression::Bool(LiteralBooleanExpression {
            indent: Indent::none(),
            value: node.value,
        }))
    }

    pub(crate) fn literal_number(
        &mut self,
        node: &IrLiteralNumberNode,
    ) -> crate::build::c::generator::Result<LiteralExpression> {
        // FIXME becomes big decimal
        Ok(LiteralExpression::Double(LiteralDoubleExpression {
            indent: Indent::none(),
            value: node.value.to_f64().unwrap(),
        }))
    }

    pub(crate) fn literal_string(
        &mut self,
        node: &IrLiteralStringNode,
    ) -> crate::build::c::generator::Result<LiteralExpression> {
        Ok(LiteralExpression::String(LiteralStringExpression {
            indent: Indent::none(),
            value: self.string_table.get(node.value).to_string(),
        }))
    }

}
