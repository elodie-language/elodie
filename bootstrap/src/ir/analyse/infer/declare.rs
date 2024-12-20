use std::rc::Rc;

use crate::common::Span;
use crate::frontend::ast;
use crate::frontend::ast::node::AstNode;
use crate::ir::analyse::{AnalysedNode, DeclareVariableNode};
use crate::ir::analyse::infer::Inference;
use crate::ir::analyse::Node::DeclareVariable;
use crate::ir::symbol::SymbolName;

impl<'a> Inference<'a> {
    pub(crate) fn infer_declare_variable(&mut self, span: Span, node: &ast::DeclareVariableNode<AstNode>) -> crate::ir::analyse::Result<AnalysedNode> {
        let symbol = self.register_variable(SymbolName::from(&node.variable));

        let mut value = Rc::new(self.infer_node(&node.value)?);

        let inferred_type = if let Some(type_node) = &node.value_type {
            self.type_from_type_node(type_node)?
        } else {
            value.inferred_type.clone()
        };

        Ok(AnalysedNode::new(DeclareVariable(DeclareVariableNode { symbol, value }), span, inferred_type))
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use crate::frontend;
    use crate::frontend::new_ast_from_str;
    use crate::ir::analyse::{analyse, InferredType};
    use crate::ir::context::Context;
    use crate::ir::symbol::SymbolId;

    #[test]
    fn declare_number_variable() {
        let mut ctx = frontend::Context::new();
        let ast = new_ast_from_str(&mut ctx, "let value = 23").unwrap();

        let mut ctx = Context::new(ctx);
        let analysed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(analysed.nodes.len(), 1);

        let result = &analysed[0];
        let inner = result.as_declared_variable();
        assert_eq!(result.inferred_type, InferredType::Number);
        assert_eq!(inner.symbol, SymbolId(1));
        assert_eq!(inner.value.as_literal_number().value, BigDecimal::from(23))
    }

    #[test]
    fn declare_number_variable_with_explicit_type() {
        let mut ctx = frontend::Context::new();
        let ast = new_ast_from_str(&mut ctx, "let value: Number = 23").unwrap();

        let mut ctx = Context::new(ctx);
        let analysed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(analysed.nodes.len(), 1);

        let result = &analysed[0];
        let inner = result.as_declared_variable();
        assert_eq!(result.inferred_type, InferredType::Number);
        assert_eq!(inner.symbol, SymbolId(1));
        assert_eq!(inner.value.as_literal_number().value, BigDecimal::from(23))
    }

    #[test]
    fn declare_string_variable_with_explicit_type() {
        let mut ctx = frontend::Context::new();
        let ast = new_ast_from_str(&mut ctx, "let value: String = 'Elo'").unwrap();

        let mut ctx = Context::new(ctx);
        let analysed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(analysed.nodes.len(), 1);

        let result = &analysed[0];
        let inner = result.as_declared_variable();
        assert_eq!(result.inferred_type, InferredType::String);
        assert_eq!(inner.symbol, SymbolId(1));
        assert_eq!(ctx.get_str(inner.value.as_literal_string().value), "Elo")
    }

    #[test]
    fn declare_boolean_variable() {
        let mut ctx = frontend::Context::new();
        let ast = new_ast_from_str(&mut ctx, "let value = true").unwrap();

        let mut ctx = Context::new(ctx);
        let analysed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(analysed.nodes.len(), 1);

        let result = &analysed[0];
        let inner = result.as_declared_variable();
        assert_eq!(result.inferred_type, InferredType::Boolean);
        assert_eq!(inner.symbol, SymbolId(1));
        assert_eq!(inner.value.as_literal_boolean().value, true)
    }
}