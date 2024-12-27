use std::str::FromStr;

use bigdecimal::BigDecimal;

use crate::common::Inferred::{Boolean, Number, String};
use crate::common::node::Node::{LiteralBoolean, LiteralNumber, LiteralString};
use crate::frontend::ast::{AstLiteralBooleanNode, AstLiteralNumberNode, AstLiteralStringNode, AstType};
use crate::ir::analyse::{TypedTreeNode, TypeLiteralBooleanNode, TypeLiteralNumberNode, TypeLiteralStringNode};
use crate::ir::analyse::pre::Pre;

// FIXME no unwrap
impl<'a> Pre<'a> {
    pub(crate) fn literal_boolean(
        &mut self,
        node: &AstLiteralBooleanNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralBoolean(TypeLiteralBooleanNode {
                value: bool::from_str(str).unwrap(),
                value_ast_type: AstType::Boolean,
            }),
            self.span(),
            Boolean,
        ))
    }

    pub(crate) fn literal_number(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralNumber(TypeLiteralNumberNode {
                value: BigDecimal::from_str(str).unwrap(),
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Number,
        ))
    }

    pub(crate) fn literal_string(
        &mut self,
        node: &AstLiteralStringNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        Ok(TypedTreeNode::new(
            LiteralString(TypeLiteralStringNode {
                value: node.0.value(),
                value_ast_type: AstType::String,
            }),
            self.span(),
            String,
        ))
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use crate::common::Context;
    use crate::common::Inferred;
    use crate::frontend::ast::AstType;
    use crate::frontend::ast_from_str;
    use crate::ir::analyse::prepare;

    #[test]
    fn number_literal() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "9924").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_literal_number();
        assert_eq!(inner.value, BigDecimal::from(9924));
        assert_eq!(inner.value_ast_type, AstType::Number);

        assert_eq!(result.inferred, Inferred::Number);
    }

    #[test]
    fn string_literal() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "'Elodie'").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_literal_string();
        assert_eq!(ctx.str_get(inner.value), "Elodie");
        assert_eq!(inner.value_ast_type, AstType::String);
        assert_eq!(result.inferred, Inferred::String);
    }

    #[test]
    fn true_literal() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "true").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_literal_boolean();
        assert_eq!(inner.value, true);
        assert_eq!(inner.value_ast_type, AstType::Boolean);
        assert_eq!(result.inferred, Inferred::Boolean);
    }

    #[test]
    fn false_literal() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "false").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_literal_boolean();
        assert_eq!(inner.value, false);
        assert_eq!(inner.value_ast_type, AstType::Boolean);
        assert_eq!(result.inferred, Inferred::Boolean);
    }
}
