use std::rc::Rc;

use bigdecimal::BigDecimal;

use crate::common::node::{
    AccessVariableNode, AccessVariableOfObjectNode, AccessVariableOfSelfNode, BlockNode,
    BreakLoopNode, CalculateNode, CallFunctionNode, CallFunctionOfObjectNode,
    CallFunctionOfPackageNode, CallFunctionWithLambdaNode, CompareNode, ContinueLoopNode,
    DeclareExternalFunctionNode, DeclareFunctionNode, DeclarePackageNode, DeclareTypeNode,
    DeclareVariableNode, DefineTypeNode, ExportPackageNode, IfNode, InstantiateTypeNode,
    InterpolateStringNode, LiteralBooleanNode, LiteralNumberNode, LiteralStringNode, LoopNode,
    Node, ReturnFromFunctionNode, Variant,
};
use crate::common::{Span, StringTableId, WithSpan};
use crate::ir::analyse::InferredType;
use crate::ir::symbol::SymbolId;

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
    TypeLiteralNumberNode,
    TypeLiteralStringNode,
    TypeLoopNode,
    TypeReturnFromFunctionNode,
>;

#[derive(Clone, Debug, PartialEq)]
pub struct TypedTreeNode {
    pub node: TypeNode,
    pub span: Span,
    pub inferred_type: InferredType,
}

impl TypedTreeNode {
    pub fn as_declared_variable(&self) -> &TypeDeclareVariableNode {
        if let Node::DeclareVariable(result) = &self.node {
            result
        } else {
            panic!("not declare variable")
        }
    }

    pub fn as_literal_boolean(&self) -> &TypeLiteralBooleanNode {
        if let Node::LiteralBoolean(result) = &self.node {
            result
        } else {
            panic!("not literal boolean")
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
}

impl TypedTreeNode {
    pub fn node(&self) -> &TypeNode {
        &self.node
    }
    pub fn node_to_owned(self) -> TypeNode {
        self.node
    }
}

impl TypedTreeNode {
    pub fn new(node: TypeNode, span: Span, inferred_type: InferredType) -> TypedTreeNode {
        TypedTreeNode {
            node,
            span,
            inferred_type,
        }
    }
}

impl WithSpan for TypedTreeNode {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAccessVariableNode {}

impl AccessVariableNode<TypeVariant> for TypeAccessVariableNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAccessVariableOfObjectNode {}

impl AccessVariableOfObjectNode<TypeVariant> for TypeAccessVariableOfObjectNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAccessVariableOfSelfNode {}

impl AccessVariableOfSelfNode<TypeVariant> for TypeAccessVariableOfSelfNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeBlockNode {}

impl BlockNode<TypeVariant> for TypeBlockNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeBreakLoopNode {}

impl BreakLoopNode<TypeVariant> for TypeBreakLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCalculateNode {}

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
pub struct TypeCallFunctionOfPackageNode {}

impl CallFunctionOfPackageNode<TypeVariant> for TypeCallFunctionOfPackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCompareNode {}

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
    pub symbol: SymbolId,
    pub value: Rc<TypedTreeNode>,
}

impl DeclareVariableNode<TypeVariant> for TypeDeclareVariableNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeExportPackageNode {}

impl ExportPackageNode<TypeVariant> for TypeExportPackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeIfNode {}

impl IfNode<TypeVariant> for TypeIfNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeInterpolateStringNode {}

impl InterpolateStringNode<TypeVariant> for TypeInterpolateStringNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeInstantiateTypeNode {}

impl InstantiateTypeNode<TypeVariant> for TypeInstantiateTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralBooleanNode {
    pub value: bool,
}

impl LiteralBooleanNode<TypeVariant> for TypeLiteralBooleanNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralNumberNode {
    pub value: BigDecimal,
}

impl LiteralNumberNode<TypeVariant> for TypeLiteralNumberNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLiteralStringNode {
    pub value: StringTableId,
}

impl LiteralStringNode<TypeVariant> for TypeLiteralStringNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLoopNode {}

impl LoopNode<TypeVariant> for TypeLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeReturnFromFunctionNode {}

impl ReturnFromFunctionNode<TypeVariant> for TypeReturnFromFunctionNode {}
