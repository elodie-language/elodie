use std::ops::Deref;

use crate::common::Inferred;
use crate::ir::analyse::infer::Inferrer;
use crate::ir::analyse::TypedTreeNode;

impl<'a> Inferrer<'a> {
    pub(crate) fn declare_variable(&mut self, node: &TypedTreeNode) -> crate::ir::analyse::Result<()> {
        let inner = node.as_declared_variable();

        let symbol = &mut self.symbol_table[inner.variable];

        self.scope.register_symbol(symbol);

        match node.inferred {
            Inferred::Boolean => {
                symbol.set_type_id(self.type_table.type_id_boolean())
            }
            Inferred::Number => {
                symbol.set_type_id(self.type_table.type_id_number())
            }
            Inferred::String => {
                symbol.set_type_id(self.type_table.type_id_string())
            }
            _ => unimplemented!()
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use crate::common::{Inferred, SymbolId};
    use crate::common::Context;
    use crate::frontend::ast_from_str;
    use crate::ir::analyse::analyse;

    #[test]
    fn declare_number_variable() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value = 23").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);
        assert!(ctx.symbol_is_number(SymbolId(1)));
    }

    #[test]
    fn declare_number_variable_with_explicit_type() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Number = 23").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_declared_variable();
        assert_eq!(result.inferred, Inferred::Number);
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_number().value, BigDecimal::from(23));

        assert!(ctx.symbol_is_number(SymbolId(1)));
    }

    #[test]
    fn declare_string_variable_with_explicit_type() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: String = 'Elo'").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_declared_variable();
        assert_eq!(result.inferred, Inferred::String);
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(ctx.str_get(inner.value.as_literal_string().value), "Elo");

        assert!(ctx.symbol_is_string(SymbolId(1)));
    }

    #[test]
    fn declare_boolean_variable() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value = true").unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        let inner = result.as_declared_variable();
        assert_eq!(result.inferred, Inferred::Boolean);
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_boolean().value, true);

        assert!(ctx.symbol_is_boolean(SymbolId(1)));
    }
}
