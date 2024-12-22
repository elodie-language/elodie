use std::rc::Rc;

use crate::common::node::Node::DeclareVariable;
use crate::common::Span;
use crate::frontend::ast::AstDeclareVariableNode;
use crate::ir::analyse::{TypeDeclareVariableNode, TypedTreeNode};
use crate::ir::analyse::infer::Inference;
use crate::ir::symbol::SymbolName;

impl<'a> Inference<'a> {
    pub(crate) fn declare_variable(
        &mut self,
        span: Span,
        node: &AstDeclareVariableNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let symbol = self.register_variable(SymbolName::from(&node.variable));

        let mut value = Rc::new(self.node(&node.value)?);

        let inferred_type = if let Some(type_node) = &node.value_type {
            self.type_from_type_node(type_node)?
        } else {
            value.inferred_type.clone()
        };

        Ok(TypedTreeNode::new(
            DeclareVariable(TypeDeclareVariableNode { symbol, value }),
            span,
            inferred_type,
        ))
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use crate::common::context::Context;
    use crate::frontend::ast_from_str;
    use crate::ir::analyse::{analyse, InferredType};
    use crate::ir::symbol::SymbolId;

    #[test]
    fn declare_number_variable() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value = 23").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_declared_variable();
        assert_eq!(result.inferred_type, InferredType::Number);
        assert_eq!(inner.symbol, SymbolId(1));
        assert_eq!(inner.value.as_literal_number().value, BigDecimal::from(23))
    }

    #[test]
    fn declare_number_variable_with_explicit_type() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Number = 23").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_declared_variable();
        assert_eq!(result.inferred_type, InferredType::Number);
        assert_eq!(inner.symbol, SymbolId(1));
        assert_eq!(inner.value.as_literal_number().value, BigDecimal::from(23))
    }

    #[test]
    fn declare_string_variable_with_explicit_type() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: String = 'Elo'").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_declared_variable();
        assert_eq!(result.inferred_type, InferredType::String);
        assert_eq!(inner.symbol, SymbolId(1));
        assert_eq!(ctx.str_get(inner.value.as_literal_string().value), "Elo")
    }

    #[test]
    fn declare_boolean_variable() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value = true").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_declared_variable();
        assert_eq!(result.inferred_type, InferredType::Boolean);
        assert_eq!(inner.symbol, SymbolId(1));
        assert_eq!(inner.value.as_literal_boolean().value, true)
    }
}
