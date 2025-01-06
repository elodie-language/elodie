use std::ops::Deref;

use crate::common::{GetString, Inferred, TypeId};
use crate::common::node::Node;
use crate::ir::analyse::infer::Inferrer;
use crate::ir::analyse::TypedTreeNode;

impl<'a> Inferrer<'a> {
    pub(crate) fn declare_variable(&mut self, node: &mut TypedTreeNode) -> crate::ir::analyse::Result<()> {
        // let mut inner = node.as_declared_variable();

        if let Node::DeclareVariable(node) = &mut node.node {
            // result
            self.node(&mut node.value)?;

            let symbol = &mut self.symbol_table[node.variable];

            self.scope.register_symbol(symbol);

            match &node.value.inferred {
                Inferred::Boolean => { symbol.set_type_id(TypeId::BOOLEAN) }
                Inferred::Float4 => { symbol.set_type_id(TypeId::FLOAT4) }
                Inferred::Float8 => { symbol.set_type_id(TypeId::FLOAT8) }
                Inferred::Int1 => { symbol.set_type_id(TypeId::INT1) }
                Inferred::Int2 => { symbol.set_type_id(TypeId::INT2) }
                Inferred::Int4 => { symbol.set_type_id(TypeId::INT4) }
                Inferred::Int8 => { symbol.set_type_id(TypeId::INT8) }
                Inferred::Int16 => { symbol.set_type_id(TypeId::INT16) }
                Inferred::Number => { symbol.set_type_id(TypeId::NUMBER) }
                Inferred::String => { symbol.set_type_id(TypeId::STRING) }
                Inferred::Uint1 => { symbol.set_type_id(TypeId::UINT1) }
                Inferred::Uint2 => { symbol.set_type_id(TypeId::UINT2) }
                Inferred::Uint4 => { symbol.set_type_id(TypeId::UINT4) }
                Inferred::Uint8 => { symbol.set_type_id(TypeId::UINT8) }
                Inferred::Uint16 => { symbol.set_type_id(TypeId::UINT16) }
                inferred => unimplemented!("{inferred:#?}")
            }

            Ok(())
        } else {
            panic!("not declare variable")
        }

        // self.node(inner.value.as_mut())?;
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
    fn shadow_variable() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, r#"

        let value = 23
        {
            let value = 9924
        }

        "#).unwrap();
        let typed = analyse(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 2);

        let outer = &typed[0];
        assert_eq!(outer.inferred, Inferred::Number);

        let outer = outer.as_declared_variable();
        assert_eq!(outer.variable, SymbolId(1));
        assert_eq!(outer.value.as_literal_number().value, BigDecimal::from(23));

        let inner = &typed[1];
        assert_eq!(inner.inferred, Inferred::Number);

        let inner = inner.as_block();
        assert_eq!(inner.nodes.len(), 1);
        let inner = inner.nodes[0].as_declared_variable();
        assert_eq!(inner.variable, SymbolId(2));
        assert_eq!(inner.value.as_literal_number().value, BigDecimal::from(9924));

        assert!(ctx.symbol_is_number(SymbolId(1)));
        assert!(ctx.symbol_is_number(SymbolId(2)));
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
