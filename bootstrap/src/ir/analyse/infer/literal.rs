use std::str::FromStr;

use bigdecimal::BigDecimal;

use crate::common::node::Node::{LiteralBoolean, LiteralNumber, LiteralString};
use crate::common::Span;
use crate::frontend::ast::{AstLiteralBooleanNode, AstLiteralNumberNode, AstLiteralStringNode};
use crate::ir::analyse::{InferredType, TypedTreeNode, TypeLiteralBooleanNode, TypeLiteralNumberNode, TypeLiteralStringNode};
use crate::ir::analyse::infer::Inference;

// FIXME no unwrap
impl<'a> Inference<'a> {
    pub(crate) fn infer_literal_boolean(
        &mut self,
        span: Span,
        node: &AstLiteralBooleanNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());

        Ok(TypedTreeNode::new(
            LiteralBoolean(TypeLiteralBooleanNode {
                value: bool::from_str(str).unwrap(),
            }),
            span,
            InferredType::Boolean,
        ))
    }

    pub(crate) fn infer_literal_number(
        &mut self,
        span: Span,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());

        Ok(TypedTreeNode::new(
            LiteralNumber(TypeLiteralNumberNode {
                value: BigDecimal::from_str(str).unwrap(),
            }),
            span,
            InferredType::Number,
        ))
    }

    pub(crate) fn infer_literal_string(
        &mut self,
        span: Span,
        node: &AstLiteralStringNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        Ok(TypedTreeNode::new(
            LiteralString(TypeLiteralStringNode {
                value: node.0.value(),
            }),
            span,
            InferredType::String,
        ))
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use crate::common::context::Context;
    use crate::frontend::ast_from_str;
    use crate::ir::analyse;
    use crate::ir::analyse::InferredType;

    #[test]
    fn number_literal() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "9924").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_literal_number();
        assert_eq!(result.inferred_type, InferredType::Number);
        assert_eq!(inner.value, BigDecimal::from(9924));
    }

    #[test]
    fn string_literal() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "'Elodie'").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_literal_string();
        assert_eq!(result.inferred_type, InferredType::String);
        assert_eq!(ctx.get_str(inner.value), "Elodie");
    }

    #[test]
    fn true_literal() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "true").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_literal_boolean();
        assert_eq!(result.inferred_type, InferredType::Boolean);
        assert_eq!(inner.value, true);
    }

    #[test]
    fn false_literal() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "false").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_literal_boolean();
        assert_eq!(result.inferred_type, InferredType::Boolean);
        assert_eq!(inner.value, false);
    }
}
