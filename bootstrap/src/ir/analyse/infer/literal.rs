use std::str::FromStr;

use bigdecimal::BigDecimal;
use crate::common::node::Node::{LiteralBoolean, LiteralNumber, LiteralString};

use crate::common::Span;
use crate::frontend::ast::{AstLiteralBooleanNode, AstLiteralNumberNode, AstLiteralStringNode};
use crate::ir::analyse::{AnalyseLiteralBooleanNode, AnalyseLiteralNumberNode, AnalyseLiteralStringNode, AnalyseTreeNode, InferredType};
use crate::ir::analyse::infer::Inference;

// FIXME no unwrap
impl<'a> Inference<'a> {
    pub(crate) fn infer_literal_boolean(&mut self, span: Span, node: &AstLiteralBooleanNode) -> crate::ir::analyse::Result<AnalyseTreeNode> {
        let str = self.string_table.get(node.0.value());

        Ok(AnalyseTreeNode::new(LiteralBoolean(AnalyseLiteralBooleanNode { value: bool::from_str(str).unwrap() }), span, InferredType::Boolean))
    }

    pub(crate) fn infer_literal_number(&mut self, span: Span, node: &AstLiteralNumberNode) -> crate::ir::analyse::Result<AnalyseTreeNode> {
        let str = self.string_table.get(node.0.value());

        Ok(AnalyseTreeNode::new(LiteralNumber(AnalyseLiteralNumberNode {
            value: BigDecimal::from_str(str).unwrap()
        }), span, InferredType::Number))
    }

    pub(crate) fn infer_literal_string(&mut self, span: Span, node: &AstLiteralStringNode) -> crate::ir::analyse::Result<AnalyseTreeNode> {
        Ok(AnalyseTreeNode::new(LiteralString(AnalyseLiteralStringNode { value: node.0.value() }), span, InferredType::String))
    }
}


#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use crate::frontend;
    use crate::frontend::ast_from_str;
    use crate::ir::analyse;
    use crate::ir::analyse::InferredType;

    #[test]
    fn number_literal() {
        let mut ctx = frontend::Context::new();
        let ast = ast_from_str(&mut ctx, "9924").unwrap();

        let mut ctx = analyse::Context::new(ctx);
        let analysed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(analysed.nodes.len(), 1);

        let result = &analysed[0];
        let inner = result.as_literal_number();
        assert_eq!(result.inferred_type, InferredType::Number);
        assert_eq!(inner.value, BigDecimal::from(9924));
    }

    #[test]
    fn string_literal() {
        let mut ctx = frontend::Context::new();
        let ast = ast_from_str(&mut ctx, "'Elodie'").unwrap();

        let mut ctx = analyse::Context::new(ctx);
        let analysed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(analysed.nodes.len(), 1);

        let result = &analysed[0];
        let inner = result.as_literal_string();
        assert_eq!(result.inferred_type, InferredType::String);
        assert_eq!(ctx.get_str(inner.value), "Elodie");
    }

    #[test]
    fn true_literal() {
        let mut ctx = frontend::Context::new();
        let ast = ast_from_str(&mut ctx, "true").unwrap();

        let mut ctx = analyse::Context::new(ctx);
        let analysed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(analysed.nodes.len(), 1);

        let result = &analysed[0];
        let inner = result.as_literal_boolean();
        assert_eq!(result.inferred_type, InferredType::Boolean);
        assert_eq!(inner.value, true);
    }

    #[test]
    fn false_literal() {
        let mut ctx = frontend::Context::new();
        let ast = ast_from_str(&mut ctx, "false").unwrap();

        let mut ctx = analyse::Context::new(ctx);
        let analysed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(analysed.nodes.len(), 1);

        let result = &analysed[0];
        let inner = result.as_literal_boolean();
        assert_eq!(result.inferred_type, InferredType::Boolean);
        assert_eq!(inner.value, false);
    }
}