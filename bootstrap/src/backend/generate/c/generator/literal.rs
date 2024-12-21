use crate::backend::generate::c::{Indent, LiteralBooleanExpression, LiteralDoubleExpression, LiteralExpression, LiteralStringExpression};
use crate::backend::generate::c::generator::Generator;
use crate::frontend::ast::{AstLiteralBooleanNode, AstLiteralNumberNode, AstLiteralStringNode};

impl Generator {
    pub(crate) fn generate_literal_bool(
        &mut self,
        node: &AstLiteralBooleanNode,
    ) -> crate::backend::generate::c::generator::Result<LiteralExpression> {
        Ok(LiteralExpression::Bool(LiteralBooleanExpression { indent: Indent::none(), value: self.string_table.get(node.0.value) == "true" })
        )
    }

    pub(crate) fn generate_literal_number(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::backend::generate::c::generator::Result<LiteralExpression> {
        Ok(LiteralExpression::Double(LiteralDoubleExpression { indent: Indent::none(), value: self.string_table.get(node.0.value).parse().unwrap() })
        )
    }

    pub(crate) fn generate_literal_string(
        &mut self,
        node: &AstLiteralStringNode,
    ) -> crate::backend::generate::c::generator::Result<LiteralExpression> {
        Ok(LiteralExpression::String(LiteralStringExpression { indent: Indent::none(), value: self.string_table.get(node.0.value).to_string() })
        )
    }
}

