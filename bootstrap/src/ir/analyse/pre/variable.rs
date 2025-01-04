use crate::common::{Inferred, SymbolName};
use crate::common::node::Node::{DeclareVariable, LiteralNumber};
use crate::frontend::ast::{AstDeclareVariableNode, AstType};
use crate::ir::analyse::{TypeDeclareVariableNode, TypedTreeNode};
use crate::ir::analyse::Error::TypeMissMatch;
use crate::ir::analyse::pre::Pre;
use crate::ir::analyse::TypeMissMatchError::DeclaredTypeMissMatch;

impl<'a> Pre<'a> {
    pub(crate) fn declare_variable(
        &mut self,
        node: &AstDeclareVariableNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let variable = &node.variable;

        if let Some(expected) = &node.value_type {
            if let LiteralNumber(node) = &node.value.node() {
                let value = match expected {
                    AstType::Float4 => self.literal_number_float4(node)?,
                    AstType::Float8 => self.literal_number_float8(node)?,
                    AstType::Int1 => self.literal_number_int1(node)?,
                    AstType::Int2 => self.literal_number_int2(node)?,
                    AstType::Int4 => self.literal_number_int4(node)?,
                    AstType::Int8 => self.literal_number_int8(node)?,
                    AstType::Int16 => self.literal_number_int16(node)?,
                    AstType::Number => self.literal_number(node)?,
                    AstType::Uint1 => self.literal_number_uint1(node)?,
                    AstType::Uint2 => self.literal_number_uint2(node)?,
                    AstType::Uint4 => self.literal_number_uint4(node)?,
                    AstType::Uint8 => self.literal_number_uint8(node)?,
                    AstType::Uint16 => self.literal_number_uint16(node)?,
                    _ => {
                        return Err(TypeMissMatch(DeclaredTypeMissMatch {
                            expected: expected.to_string(&self.string_table),
                            got: "Number".to_string(),
                            span: self.span(),
                        }));
                    }
                };

                let value_inferred = value.inferred.clone();
                let variable = self.variable_register(SymbolName::from(&variable.clone()), value_inferred.clone());

                return Ok(TypedTreeNode::new(
                    DeclareVariable(TypeDeclareVariableNode { variable, value: Box::new(value) }),
                    self.span(),
                    value_inferred,
                ));
            }
        }


        let value = Box::new(self.node(&node.value)?);
        let value_inferred = value.inferred.clone();

        let variable = self.variable_register(SymbolName::from(&variable.clone()), value_inferred.clone());

        if let Some(expected) = &node.value_type {
            let matches = match (expected, &value_inferred) {
                (&AstType::Boolean, &Inferred::Boolean) => true,
                (&AstType::Boolean, _) => false,
                (&AstType::Float4, &Inferred::Float4) => true,
                (&AstType::Float4, _) => false,
                (&AstType::Float8, &Inferred::Float8) => true,
                (&AstType::Float8, _) => false,

                (&AstType::Int1, &Inferred::Int1) => true,
                (&AstType::Int1, _) => false,

                (&AstType::Int2, &Inferred::Int2) => true,
                (&AstType::Int2, _) => false,

                (&AstType::Int4, &Inferred::Int4) => true,
                (&AstType::Int4, _) => false,

                (&AstType::Int8, &Inferred::Int8) => true,
                (&AstType::Int8, _) => false,

                (&AstType::Int16, &Inferred::Int16) => true,
                (&AstType::Int16, _) => false,

                (&AstType::Number, &Inferred::Number) => true,
                (&AstType::Number, _) => false,

                (&AstType::String, &Inferred::String) => true,
                (&AstType::String, _) => false,

                (&AstType::Uint1, &Inferred::Uint1) => true,
                (&AstType::Uint1, _) => false,

                (&AstType::Uint2, &Inferred::Uint2) => true,
                (&AstType::Uint2, _) => false,

                (&AstType::Uint4, &Inferred::Uint4) => true,
                (&AstType::Uint4, _) => false,

                (&AstType::Uint8, &Inferred::Uint8) => true,
                (&AstType::Uint8, _) => false,

                (&AstType::Uint16, &Inferred::Uint16) => true,
                (&AstType::Uint16, _) => false,

                (_, _) => unimplemented!()
            };
            if !matches {
                return Err(TypeMissMatch(DeclaredTypeMissMatch {
                    expected: expected.to_string(&self.string_table),
                    got: value_inferred.to_string(&self.string_table),
                    span: self.span(),
                }));
            }
        }

        Ok(TypedTreeNode::new(
            DeclareVariable(TypeDeclareVariableNode { variable, value }),
            self.span(),
            value_inferred,
        ))
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use TypeMissMatchError::DeclaredTypeMissMatch;

    use crate::common::{Inferred, SymbolId};
    use crate::common::Context;
    use crate::frontend::ast_from_str;
    use crate::ir::analyse::{analyse, prepare, TypeMissMatchError};
    use crate::ir::analyse::Error::TypeMissMatch;

    #[test]
    fn declare_number() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value = 23").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Number);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_number().value, BigDecimal::from(23));
    }

    #[test]
    fn declare_number_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Number = 23").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Number);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_number().value, BigDecimal::from(23));
    }

    #[test]
    fn declare_float4_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Float4 = 4").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Float4);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_float4().value, 4f32);
    }

    #[test]
    fn declare_float8_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Float8 = 8").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Float8);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_float8().value, 8f64);
    }

    #[test]
    fn declare_int1_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Int1 = 1").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Int1);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_int1().value, 1i8);
    }

    #[test]
    fn declare_int2_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Int2 = 2").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Int2);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_int2().value, 2i16);
    }

    #[test]
    fn declare_int4_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Int4 = 4").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Int4);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_int4().value, 4i32);
    }

    #[test]
    fn declare_int8_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Int8 = 8").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Int8);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_int8().value, 8i64);
    }

    #[test]
    fn declare_int16_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Int16 = 16").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Int16);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_int16().value, 16i128);
    }

    #[test]
    fn declare_string_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: String = 'Elo'").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::String);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(ctx.str_get(inner.value.as_literal_string().value), "Elo")
    }

    #[test]
    fn declare_uint1_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Uint1 = 1").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Uint1);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_uint1().value, 1u8);
    }

    #[test]
    fn declare_uint2_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Uint2 = 2").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Uint2);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_uint2().value, 2u16);
    }

    #[test]
    fn declare_uint4_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Uint4 = 4").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Uint4);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_uint4().value, 4u32);
    }

    #[test]
    fn declare_uint8_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Uint8 = 8").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Uint8);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_uint8().value, 8u64);
    }

    #[test]
    fn declare_uint16_explicit() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value: Uint16 = 16").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Uint16);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(1));
        assert_eq!(inner.value.as_literal_uint16().value, 16u128);
    }

    #[test]
    fn declare_boolean() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, "let value = true").unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 1);

        let result = &typed[0];
        assert_eq!(result.inferred, Inferred::Boolean);

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

    #[test]
    fn shadow_variable() {
        let mut ctx = Context::testing();
        let ast = ast_from_str(&mut ctx, r#"
            let value = 23
            let value = true
            let value = 'Elodie'
        "#).unwrap();
        let typed = prepare(&mut ctx, ast).unwrap();
        assert_eq!(typed.nodes.len(), 3);

        let result = &typed[2];
        assert_eq!(result.inferred, Inferred::String);

        let inner = result.as_declared_variable();
        assert_eq!(inner.variable, SymbolId(3));
        assert_eq!(ctx.str_get(inner.value.as_literal_string().value), "Elodie");
    }
}
