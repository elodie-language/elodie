use Error::TypeMissMatch;
use TypeMissMatchError::DeclaredTypeMissMatch;

use crate::common::node::Node::DeclareVariable;
use crate::common::{Span, SymbolName};
use crate::frontend::ast::{AstDeclareVariableNode, AstType};
use crate::ir::analyse::{Error, InferredType, TypeDeclareVariableNode, TypedTreeNode, TypeMissMatchError};
use crate::ir::analyse::pre::Pre;

impl<'a> Pre<'a> {
    pub(crate) fn declare_variable(
        &mut self,
        span: Span,
        node: &AstDeclareVariableNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let symbol = self.register_variable(SymbolName::from(&node.variable));

        let value = Box::new(self.node(&node.value)?);
        let value_inferred = value.inferred.clone();

        if let Some(expected) = &node.value_type {
            if value_inferred != InferredType::Unknown {
                let matches = match (expected, &value_inferred) {
                    (&AstType::Boolean, &InferredType::Boolean) => true,
                    (&AstType::Boolean, _) => false,
                    (&AstType::Number, &InferredType::Number) => true,
                    (&AstType::Number, _) => false,
                    (&AstType::String, &InferredType::String) => true,
                    (&AstType::String, _) => false,
                    (_, _) => unimplemented!()
                };

                if !matches {
                    return Err(TypeMissMatch(DeclaredTypeMissMatch {
                        expected: expected.to_string(&self.string_table),
                        got: value_inferred.to_string(&self.string_table),
                        span,
                    }));
                }
            }
        }

        Ok(TypedTreeNode::new(
            DeclareVariable(TypeDeclareVariableNode { variable: symbol, value }),
            span,
            value_inferred,
        ))
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use TypeMissMatchError::DeclaredTypeMissMatch;

    use crate::common::context::Context;
    use crate::common::SymbolId;
    use crate::frontend::ast_from_str;
    use crate::ir::analyse::{analyse, InferredType, prepare, TypeMissMatchError};
    use crate::ir::analyse::Error::TypeMissMatch;

    #[test]
    fn declare_number_variable() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value = 23").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, InferredType::Number);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_number().value, BigDecimal::from(23));
    }

    #[test]
    fn declare_number_variable_with_explicit_type() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Number = 23").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, InferredType::Number);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_number().value, BigDecimal::from(23));
    }

    #[test]
    fn declare_string_variable_with_explicit_type() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: String = 'Elo'").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, InferredType::String);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(ctx.str_get(inner.value.as_literal_string().value), "Elo")
    }

    #[test]
    fn declare_boolean_variable() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value = true").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, InferredType::Boolean);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_boolean().value, true)
    }

    #[test]
    fn declared_type_different_then_value() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value : String = 23").unwrap();
        let result = analyse(&mut ctx, ast);
        assert!(result.is_err());

        let error = result.err().unwrap();

        let TypeMissMatch(DeclaredTypeMissMatch { expected, got, .. }) = error else { panic!() };
        assert_eq!(expected, "String");
        assert_eq!(got, "Number");
    }
}
