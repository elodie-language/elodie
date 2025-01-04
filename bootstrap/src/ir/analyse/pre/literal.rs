use std::i8;
use std::str::FromStr;

use bigdecimal::BigDecimal;

use crate::common::Inferred::{Boolean, Float4, Float8, Int1, Int16, Int2, Int4, Int8, Number, String, Uint1, Uint16, Uint2, Uint4, Uint8};
use crate::common::node::Node::{LiteralBoolean, LiteralFloat4, LiteralFloat8, LiteralInt1, LiteralInt16, LiteralInt2, LiteralInt4, LiteralInt8, LiteralNumber, LiteralString, LiteralUint1, LiteralUint16, LiteralUint2, LiteralUint4, LiteralUint8};
use crate::frontend::ast::{AstLiteralBooleanNode, AstLiteralFloat4Node, AstLiteralNumberNode, AstLiteralStringNode, AstType};
use crate::ir::analyse::{Error, InvalidLiteralError, TypedTreeNode, TypeLiteralBooleanNode, TypeLiteralFloat4Node, TypeLiteralFloat8Node, TypeLiteralInt16Node, TypeLiteralInt1Node, TypeLiteralInt2Node, TypeLiteralInt4Node, TypeLiteralInt8Node, TypeLiteralNumberNode, TypeLiteralStringNode, TypeLiteralUint16Node, TypeLiteralUint1Node, TypeLiteralUint2Node, TypeLiteralUint4Node, TypeLiteralUint8Node};
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

    pub(crate) fn literal_float4(
        &mut self,
        node: &AstLiteralFloat4Node,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralFloat4(TypeLiteralFloat4Node {
                value: f32::from_str(str).unwrap(),
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Float4,
        ))
    }

    pub(crate) fn literal_number_float4(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralFloat4(TypeLiteralFloat4Node {
                value: f32::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Float4 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Float4,
        ))
    }

    pub(crate) fn literal_number_float8(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralFloat8(TypeLiteralFloat8Node {
                value: f64::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Float8 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Float8,
        ))
    }

    pub(crate) fn literal_number_int1(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralInt1(TypeLiteralInt1Node {
                value: i8::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Int1 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Int1,
        ))
    }

    pub(crate) fn literal_number_int2(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralInt2(TypeLiteralInt2Node {
                value: i16::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Int2 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Int2,
        ))
    }

    pub(crate) fn literal_number_int4(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralInt4(TypeLiteralInt4Node {
                value: i32::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Int4 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Int4,
        ))
    }

    pub(crate) fn literal_number_int8(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralInt8(TypeLiteralInt8Node {
                value: i64::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Int8 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Int8,
        ))
    }

    pub(crate) fn literal_number_int16(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralInt16(TypeLiteralInt16Node {
                value: i128::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Int16 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Int16,
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

    pub(crate) fn literal_number_uint1(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralUint1(TypeLiteralUint1Node {
                value: u8::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Uint1 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Uint1,
        ))
    }

    pub(crate) fn literal_number_uint2(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralUint2(TypeLiteralUint2Node {
                value: u16::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Uint2 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Uint2,
        ))
    }

    pub(crate) fn literal_number_uint4(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralUint4(TypeLiteralUint4Node {
                value: u32::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Uint4 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Uint4,
        ))
    }

    pub(crate) fn literal_number_uint8(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralUint8(TypeLiteralUint8Node {
                value: u64::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Uint8 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Uint8,
        ))
    }

    pub(crate) fn literal_number_uint16(
        &mut self,
        node: &AstLiteralNumberNode,
    ) -> crate::ir::analyse::Result<TypedTreeNode> {
        let str = self.string_table.get(node.0.value());
        Ok(TypedTreeNode::new(
            LiteralUint16(TypeLiteralUint16Node {
                value: u128::from_str(str).map_err(|_| {
                    Error::InvalidLiteral(InvalidLiteralError::Uint16 {
                        got: str.to_string(),
                        span: node.0.span.clone(),
                    })
                })?,
                value_ast_type: AstType::Number,
            }),
            self.span(),
            Uint16,
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
    fn number() {
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
    fn string() {
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
    fn r#true() {
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
    fn r#false() {
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
