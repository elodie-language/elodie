use crate::generate::c::generator::Generator;
use crate::generate::c::{
    Indent, LiteralDoubleExpression, LiteralExpression, LiteralStringExpression,
};
use crate::ir;
use crate::ir::LiteralNode;

impl Generator {
    pub(crate) fn generate_literal(
        &mut self,
        node: &ir::LiteralNode,
    ) -> crate::generate::c::generator::Result<LiteralExpression> {
        Ok(match node {
            LiteralNode::Bool(_) => unimplemented!(),
            LiteralNode::Number(n) => LiteralExpression::Double(LiteralDoubleExpression {
                indent: Indent::none(),
                value: self.string_table.get(n.value).parse().unwrap(),
            }),
            LiteralNode::String(s) => LiteralExpression::String(LiteralStringExpression {
                indent: Indent::none(),
                value: self.string_table.get(s.value).to_string(),
            }),
        })
    }
}
