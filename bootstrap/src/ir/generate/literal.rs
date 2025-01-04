use crate::common::{Span, TypeId};
use crate::common::node::Node::{LiteralBoolean, LiteralFloat4, LiteralFloat8, LiteralInt1, LiteralInt16, LiteralInt2, LiteralInt4, LiteralInt8, LiteralNumber, LiteralString, LiteralUint1, LiteralUint16, LiteralUint2, LiteralUint4, LiteralUint8};
use crate::ir::{IrLiteralFloat4Node, IrLiteralFloat8Node, IrLiteralInt16Node, IrLiteralInt1Node, IrLiteralInt2Node, IrLiteralInt4Node, IrLiteralInt8Node, IrLiteralUint16Node, IrLiteralUint1Node, IrLiteralUint2Node, IrLiteralUint4Node, IrLiteralUint8Node};
use crate::ir::analyse::{TypeLiteralBooleanNode, TypeLiteralFloat4Node, TypeLiteralFloat8Node, TypeLiteralInt16Node, TypeLiteralInt1Node, TypeLiteralInt2Node, TypeLiteralInt4Node, TypeLiteralInt8Node, TypeLiteralNumberNode, TypeLiteralStringNode, TypeLiteralUint16Node, TypeLiteralUint1Node, TypeLiteralUint2Node, TypeLiteralUint4Node, TypeLiteralUint8Node};
use crate::ir::generate::Generator;
use crate::ir::node::{IrLiteralBooleanNode, IrLiteralNumberNode, IrLiteralStringNode, IrTreeNode};

impl<'a> Generator<'a> {
    pub(crate) fn literal_boolean(&mut self, node: &TypeLiteralBooleanNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralBoolean(IrLiteralBooleanNode { value: node.value.clone() }),
            span,
            TypeId::BOOLEAN,
        ))
    }

    pub(crate) fn literal_float4(&mut self, node: &TypeLiteralFloat4Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralFloat4(IrLiteralFloat4Node { value: node.value.clone() }),
            span,
            TypeId::FLOAT4,
        ))
    }

    pub(crate) fn literal_float8(&mut self, node: &TypeLiteralFloat8Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralFloat8(IrLiteralFloat8Node { value: node.value.clone() }),
            span,
            TypeId::FLOAT8,
        ))
    }

    pub(crate) fn literal_int1(&mut self, node: &TypeLiteralInt1Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralInt1(IrLiteralInt1Node { value: node.value.clone() }),
            span,
            TypeId::INT1,
        ))
    }

    pub(crate) fn literal_int2(&mut self, node: &TypeLiteralInt2Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralInt2(IrLiteralInt2Node { value: node.value.clone() }),
            span,
            TypeId::INT2,
        ))
    }

    pub(crate) fn literal_int4(&mut self, node: &TypeLiteralInt4Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralInt4(IrLiteralInt4Node { value: node.value.clone() }),
            span,
            TypeId::INT4,
        ))
    }

    pub(crate) fn literal_int8(&mut self, node: &TypeLiteralInt8Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralInt8(IrLiteralInt8Node { value: node.value.clone() }),
            span,
            TypeId::INT8,
        ))
    }

    pub(crate) fn literal_int16(&mut self, node: &TypeLiteralInt16Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralInt16(IrLiteralInt16Node { value: node.value.clone() }),
            span,
            TypeId::INT16,
        ))
    }

    pub(crate) fn literal_number(&mut self, node: &TypeLiteralNumberNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralNumber(IrLiteralNumberNode { value: node.value.clone() }),
            span,
            TypeId::NUMBER,
        ))
    }

    pub(crate) fn literal_string(&mut self, node: &TypeLiteralStringNode, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralString(IrLiteralStringNode { value: node.value.clone() }),
            span,
            TypeId::STRING,
        ))
    }

    pub(crate) fn literal_uint1(&mut self, node: &TypeLiteralUint1Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralUint1(IrLiteralUint1Node { value: node.value.clone() }),
            span,
            TypeId::UINT1,
        ))
    }

    pub(crate) fn literal_uint2(&mut self, node: &TypeLiteralUint2Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralUint2(IrLiteralUint2Node { value: node.value.clone() }),
            span,
            TypeId::UINT2,
        ))
    }

    pub(crate) fn literal_uint4(&mut self, node: &TypeLiteralUint4Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralUint4(IrLiteralUint4Node { value: node.value.clone() }),
            span,
            TypeId::UINT4,
        ))
    }

    pub(crate) fn literal_uint8(&mut self, node: &TypeLiteralUint8Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralUint8(IrLiteralUint8Node { value: node.value.clone() }),
            span,
            TypeId::UINT8,
        ))
    }

    pub(crate) fn literal_uint16(&mut self, node: &TypeLiteralUint16Node, span: Span) -> crate::ir::generate::Result<IrTreeNode> {
        Ok(IrTreeNode::new(
            LiteralUint16(IrLiteralUint16Node { value: node.value.clone() }),
            span,
            TypeId::UINT16,
        ))
    }
}

mod tests {
    use bigdecimal::BigDecimal;

    use crate::common::{Context, TypeId};
    use crate::ir::ir_from_str;

    #[test]
    fn boolean_literal() {
        let mut ctx = Context::testing();
        let ir = ir_from_str(&mut ctx, "false").unwrap();
        assert_eq!(ir.len(), 1);

        let result = &ir[0];
        assert_eq!(result.as_literal_boolean().value, false);
        assert_eq!(result.type_id, TypeId::BOOLEAN)
    }

    #[test]
    fn number_literal() {
        let mut ctx = Context::testing();
        let ir = ir_from_str(&mut ctx, "9924").unwrap();
        assert_eq!(ir.len(), 1);

        let result = &ir[0];
        assert_eq!(result.as_literal_number().value, BigDecimal::from(9924));
        assert_eq!(result.type_id, TypeId::NUMBER)
    }

    #[test]
    fn string_literal() {
        let mut ctx = Context::testing();
        let ir = ir_from_str(&mut ctx, "'Elodie'").unwrap();
        assert_eq!(ir.len(), 1);

        let result = &ir[0];
        assert_eq!(ctx.str_get(result.as_literal_string().value), "Elodie");
        assert_eq!(result.type_id, TypeId::STRING)
    }
}
