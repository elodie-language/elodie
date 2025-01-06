use std::cell::RefCell;

use bigdecimal::BigDecimal;

use crate::common::{Inferred, Span, StringTableId, SymbolId, WithSpan};
use crate::common::node::{AccessVariableNode, AccessVariableOfObjectNode, AccessVariableOfSelfNode, BlockNode, BreakLoopNode, CalculateNode, CalculateOperator, CallFunctionNode, CallFunctionOfObjectNode, CallFunctionOfPackageNode, CallFunctionWithLambdaNode, CompareNode, CompareOperator, ContinueLoopNode, DeclareExternalFunctionNode, DeclareFunctionNode, DeclarePackageNode, DeclareTypeNode, DeclareVariableNode, DefineTypeNode, ExportPackageNode, IfNode, InstantiateTypeNode, InterpolateStringNode, LiteralBooleanNode, LiteralFloat4Node, LiteralFloat8Node, LiteralInt16Node, LiteralInt1Node, LiteralInt2Node, LiteralInt4Node, LiteralInt8Node, LiteralNumberNode, LiteralStringNode, LiteralUint16Node, LiteralUint1Node, LiteralUint2Node, LiteralUint4Node, LiteralUint8Node, LoopNode, Node, ReturnFromFunctionNode, Variant};
use crate::frontend::ast::AstType;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeVariant {}

impl Variant for TypeVariant {}

pub type TypeNode = crate::common::node::Node<
    TypeVariant,
    TypeAccessVariableNode,
    TypeAccessVariableOfObjectNode,
    TypeAccessVariableOfSelfNode,
    TypeBlockNode,
    TypeBreakLoopNode,
    TypeCalculateNode,
    TypeCallFunctionNode,
    TypeCallFunctionWithLambdaNode,
    TypeCallFunctionOfObjectNode,
    TypeCallFunctionOfPackageNode,
    TypeCompareNode,
    TypeContinueLoopNode,
    TypeDeclareExternalFunctionNode,
    TypeDeclareFunctionNode,
    TypeDeclarePackageNode,
    TypeDeclareTypeNode,
    TypeDeclareVariableNode,
    TypeDefineTypeNode,
    TypeExportPackageNode,
    TypeIfNode,
    TypeInterpolateStringNode,
    TypeInstantiateTypeNode,
    TypeLiteralBooleanNode,
    TypeLiteralFloat4Node,
    TypeLiteralFloat8Node,
    TypeLiteralInt1Node,
    TypeLiteralInt2Node,
    TypeLiteralInt4Node,
    TypeLiteralInt8Node,
    TypeLiteralInt16Node,
    TypeLiteralNumberNode,
    TypeLiteralStringNode,
    TypeLiteralUint1Node,
    TypeLiteralUint2Node,
    TypeLiteralUint4Node,
    TypeLiteralUint8Node,
    TypeLiteralUint16Node,
    TypeLoopNode,
    TypeReturnFromFunctionNode,
>;

pub(crate) enum Direction {
    Forward,
    Backward,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypedTreeNode {
    pub node: TypeNode,
    pub span: Span,
    pub inferred: Inferred,
}

impl TypedTreeNode {
    pub fn as_access_variable(&self) -> &TypeAccessVariableNode {
        if let Node::AccessVariable(result) = &self.node {
            result
        } else {
            panic!("not access variable")
        }
    }

    pub fn as_block(&self) -> &TypeBlockNode {
        if let Node::Block(result) = &self.node {
            result
        } else {
            panic!("not block")
        }
    }

    pub fn as_compare(&self) -> &TypeCompareNode {
        if let Node::Compare(result) = &self.node {
            result
        } else {
            panic!("not compare")
        }
    }

    pub fn as_declared_variable(&self) -> &TypeDeclareVariableNode {
        if let Node::DeclareVariable(result) = &self.node {
            result
        } else {
            panic!("not declare variable")
        }
    }

    pub fn as_if(&self) -> &TypeIfNode {
        if let Node::If(result) = &self.node {
            result
        } else {
            panic!("not if")
        }
    }

    pub fn as_interpolate_string(&self) -> &TypeInterpolateStringNode {
        if let Node::InterpolateString(result) = &self.node {
            result
        } else {
            panic!("not interpolate string")
        }
    }

    pub fn as_literal_boolean(&self) -> &TypeLiteralBooleanNode {
        if let Node::LiteralBoolean(result) = &self.node {
            result
        } else {
            panic!("not literal boolean")
        }
    }

    pub fn as_literal_float4(&self) -> &TypeLiteralFloat4Node {
        if let Node::LiteralFloat4(result) = &self.node {
            result
        } else {
            panic!("not literal float4")
        }
    }

    pub fn as_literal_float8(&self) -> &TypeLiteralFloat8Node {
        if let Node::LiteralFloat8(result) = &self.node {
            result
        } else {
            panic!("not literal float8")
        }
    }

    pub fn as_literal_int1(&self) -> &TypeLiteralInt1Node {
        if let Node::LiteralInt1(result) = &self.node {
            result
        } else {
            panic!("not literal int1")
        }
    }

    pub fn as_literal_int2(&self) -> &TypeLiteralInt2Node {
        if let Node::LiteralInt2(result) = &self.node {
            result
        } else {
            panic!("not literal int2")
        }
    }

    pub fn as_literal_int4(&self) -> &TypeLiteralInt4Node {
        if let Node::LiteralInt4(result) = &self.node {
            result
        } else {
            panic!("not literal int4")
        }
    }

    pub fn as_literal_int8(&self) -> &TypeLiteralInt8Node {
        if let Node::LiteralInt8(result) = &self.node {
            result
        } else {
            panic!("not literal int8")
        }
    }

    pub fn as_literal_int16(&self) -> &TypeLiteralInt16Node {
        if let Node::LiteralInt16(result) = &self.node {
            result
        } else {
            panic!("not literal int16")
        }
    }

    pub fn as_literal_number(&self) -> &TypeLiteralNumberNode {
        if let Node::LiteralNumber(result) = &self.node {
            result
        } else {
            panic!("not literal number")
        }
    }

    pub fn as_literal_string(&self) -> &TypeLiteralStringNode {
        if let Node::LiteralString(result) = &self.node {
            result
        } else {
            panic!("not literal string")
        }
    }

    pub fn as_literal_uint1(&self) -> &TypeLiteralUint1Node {
        if let Node::LiteralUint1(result) = &self.node {
            result
        } else {
            panic!("not literal uint1")
        }
    }

    pub fn as_literal_uint2(&self) -> &TypeLiteralUint2Node {
        if let Node::LiteralUint2(result) = &self.node {
            result
        } else {
            panic!("not literal uint2")
        }
    }

    pub fn as_literal_uint4(&self) -> &TypeLiteralUint4Node {
        if let Node::LiteralUint4(result) = &self.node {
            result
        } else {
            panic!("not literal uint4")
        }
    }

    pub fn as_literal_uint8(&self) -> &TypeLiteralUint8Node {
        if let Node::LiteralUint8(result) = &self.node {
            result
        } else {
            panic!("not literal uint8")
        }
    }

    pub fn as_literal_uint16(&self) -> &TypeLiteralUint16Node {
        if let Node::LiteralUint16(result) = &self.node {
            result
        } else {
            panic!("not literal uint16")
        }
    }
}

impl TypedTreeNode {
    pub fn node(&self) -> &TypeNode {
        &self.node
    }
    pub fn node_mut(&mut self) -> &mut TypeNode {
        &mut self.node
    }
    pub fn node_to_owned(self) -> TypeNode {
        self.node
    }
}

impl TypedTreeNode {
    pub fn new(node: TypeNode, span: Span, inferred: Inferred) -> TypedTreeNode {
        TypedTreeNode {
            node,
            span,
            inferred,
        }
    }
}

impl WithSpan for TypedTreeNode {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAccessVariableNode {
    pub variable: SymbolId,
}

impl AccessVariableNode<TypeVariant> for TypeAccessVariableNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAccessVariableOfObjectNode {}

impl AccessVariableOfObjectNode<TypeVariant> for TypeAccessVariableOfObjectNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAccessVariableOfSelfNode {}

impl AccessVariableOfSelfNode<TypeVariant> for TypeAccessVariableOfSelfNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeBlockNode {
    pub nodes: Box<[TypedTreeNode]>,
}

impl BlockNode<TypeVariant> for TypeBlockNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeBreakLoopNode {
    pub node: Option<Box<TypedTreeNode>>,
}

impl BreakLoopNode<TypeVariant> for TypeBreakLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCalculateNode {
    pub left: Box<TypedTreeNode>,
    pub operator: CalculateOperator,
    pub right: Box<TypedTreeNode>,
}

impl CalculateNode<TypeVariant> for TypeCalculateNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCallFunctionNode {}

impl CallFunctionNode<TypeVariant> for TypeCallFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCallFunctionWithLambdaNode {}

impl CallFunctionWithLambdaNode<TypeVariant> for TypeCallFunctionWithLambdaNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCallFunctionOfObjectNode {}

impl CallFunctionOfObjectNode<TypeVariant> for TypeCallFunctionOfObjectNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCallFunctionOfPackageNode {
    pub package: SymbolId,
    pub function: SymbolId,
    pub arguments: Box<[TypedTreeNode]>,
}

impl CallFunctionOfPackageNode<TypeVariant> for TypeCallFunctionOfPackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCompareNode {
    pub left: Box<TypedTreeNode>,
    pub operator: CompareOperator,
    pub right: Box<TypedTreeNode>,
}

impl CompareNode<TypeVariant> for TypeCompareNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeContinueLoopNode {}

impl ContinueLoopNode<TypeVariant> for TypeContinueLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclareExternalFunctionNode {}

impl DeclareExternalFunctionNode<TypeVariant> for TypeDeclareExternalFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclareFunctionNode {}

impl DeclareFunctionNode<TypeVariant> for TypeDeclareFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclarePackageNode {}

impl DeclarePackageNode<TypeVariant> for TypeDeclarePackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclareTypeNode {}

impl DeclareTypeNode<TypeVariant> for TypeDeclareTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDefineTypeNode {}

impl DefineTypeNode<TypeVariant> for TypeDefineTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDeclareVariableNode {
    pub variable: SymbolId,
    pub value: Box<TypedTreeNode>,
}

impl DeclareVariableNode<TypeVariant> for TypeDeclareVariableNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeExportPackageNode {}

impl ExportPackageNode<TypeVariant> for TypeExportPackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeIfNode {
    pub condition: Box<TypedTreeNode>,
    pub then: RefCell<TypeBlockNode>,
    pub otherwise: Option<RefCell<TypeBlockNode>>,
}

impl IfNode<TypeVariant> for TypeIfNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeInterpolateStringNode {
    pub nodes: Box<[TypedTreeNode]>,
}

impl InterpolateStringNode<TypeVariant> for TypeInterpolateStringNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeInstantiateTypeNode {}

impl InstantiateTypeNode<TypeVariant> for TypeInstantiateTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralBooleanNode {
    pub value: bool,
    pub value_ast_type: AstType,
}

impl LiteralBooleanNode<TypeVariant> for TypeLiteralBooleanNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralFloat4Node {
    pub value: f32,
    pub value_ast_type: AstType,
}

impl LiteralFloat4Node<TypeVariant> for TypeLiteralFloat4Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralFloat8Node {
    pub value: f64,
    pub value_ast_type: AstType,
}

impl LiteralFloat8Node<TypeVariant> for TypeLiteralFloat8Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralInt1Node {
    pub value: i8,
    pub value_ast_type: AstType,
}

impl LiteralInt1Node<TypeVariant> for TypeLiteralInt1Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralInt2Node {
    pub value: i16,
    pub value_ast_type: AstType,
}

impl LiteralInt2Node<TypeVariant> for TypeLiteralInt2Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralInt4Node {
    pub value: i32,
    pub value_ast_type: AstType,
}

impl LiteralInt4Node<TypeVariant> for TypeLiteralInt4Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralInt8Node {
    pub value: i64,
    pub value_ast_type: AstType,
}

impl LiteralInt8Node<TypeVariant> for TypeLiteralInt8Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralInt16Node {
    pub value: i128,
    pub value_ast_type: AstType,
}

impl LiteralInt16Node<TypeVariant> for TypeLiteralInt16Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralNumberNode {
    pub value: BigDecimal,
    pub value_ast_type: AstType,
}

impl LiteralNumberNode<TypeVariant> for TypeLiteralNumberNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralStringNode {
    pub value: StringTableId,
    pub value_ast_type: AstType,
}

impl LiteralStringNode<TypeVariant> for TypeLiteralStringNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralUint1Node {
    pub value: u8,
    pub value_ast_type: AstType,
}

impl LiteralUint1Node<TypeVariant> for TypeLiteralUint1Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralUint2Node {
    pub value: u16,
    pub value_ast_type: AstType,
}

impl LiteralUint2Node<TypeVariant> for TypeLiteralUint2Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralUint4Node {
    pub value: u32,
    pub value_ast_type: AstType,
}

impl LiteralUint4Node<TypeVariant> for TypeLiteralUint4Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralUint8Node {
    pub value: u64,
    pub value_ast_type: AstType,
}

impl LiteralUint8Node<TypeVariant> for TypeLiteralUint8Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralUint16Node {
    pub value: u128,
    pub value_ast_type: AstType,
}

impl LiteralUint16Node<TypeVariant> for TypeLiteralUint16Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLoopNode {
    pub nodes: RefCell<TypeBlockNode>,
}

impl LoopNode<TypeVariant> for TypeLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeReturnFromFunctionNode {}

impl ReturnFromFunctionNode<TypeVariant> for TypeReturnFromFunctionNode {}
