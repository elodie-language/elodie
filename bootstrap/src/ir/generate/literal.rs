use crate::common::node::Node::{LiteralBoolean, LiteralNumber, LiteralString};
use crate::common::Span;
use crate::ir::analyse::{TypeLiteralBooleanNode, TypeLiteralNumberNode, TypeLiteralStringNode};
use crate::ir::generate::Generator;
use crate::ir::node::{IrLiteralBooleanNode, IrLiteralNumberNode, IrLiteralStringNode, IrTreeNode};

impl<'a> Generator<'a> {
    pub(crate) fn literal_boolean(&mut self, node: &TypeLiteralBooleanNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralBoolean(IrLiteralBooleanNode { value: node.value.clone() }),
            span,
            self.type_table.type_id_boolean(),
        ))
    }

    pub(crate) fn literal_number(&mut self, node: &TypeLiteralNumberNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralNumber(IrLiteralNumberNode { value: node.value.clone() }),
            span,
            self.type_table.type_id_number(),
        ))
    }

    pub(crate) fn literal_string(&mut self, node: &TypeLiteralStringNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralString(IrLiteralStringNode { value: node.value.clone() }),
            span,
            self.type_table.type_id_string(),
        ))
    }
}

mod tests {
    use bigdecimal::BigDecimal;

    use crate::common::context::Context;
    use crate::ir::ir_from_str;

    #[test]
    fn boolean_literal() {
        let mut ctx = Context::testing();
        let ir = ir_from_str(&mut ctx, "false").unwrap();
        assert_eq!(ir.len(), 1);

        let result = &ir[0];
        assert_eq!(result.as_literal_boolean().value, false);
        assert_eq!(result.type_id, ctx.type_id_boolean())
    }

    #[test]
    fn number_literal() {
        let mut ctx = Context::testing();
        let ir = ir_from_str(&mut ctx, "9924").unwrap();
        assert_eq!(ir.len(), 1);

        let result = &ir[0];
        assert_eq!(result.as_literal_number().value, BigDecimal::from(9924));
        assert_eq!(result.type_id, ctx.type_id_number())
    }

    #[test]
    fn string_literal() {
        let mut ctx = Context::testing();
        let ir = ir_from_str(&mut ctx, "'Elodie'").unwrap();
        assert_eq!(ir.len(), 1);

        let result = &ir[0];
        assert_eq!(ctx.str_get(result.as_literal_string().value), "Elodie");
        assert_eq!(result.type_id, ctx.type_id_string())
    }
}
