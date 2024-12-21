use std::rc::Rc;

use bigdecimal::BigDecimal;

use crate::common::{Span, StringTableId, WithSpan};
use crate::common::node::{AccessVariableNode, AccessVariableOfObjectNode, AccessVariableOfSelfNode, BlockNode, BreakLoopNode, CalculateNode, CallFunctionNode, CallFunctionOfObjectNode, CallFunctionOfPackageNode, CallFunctionWithLambdaNode, CompareNode, ContinueLoopNode, DeclareExternalFunctionNode, DeclareFunctionNode, DeclarePackageNode, DeclareTypeNode, DeclareVariableNode, DefineTypeNode, ExportPackageNode, IfNode, InstantiateTypeNode, InterpolateStringNode, LiteralBooleanNode, LiteralNumberNode, LiteralStringNode, LoopNode, Node, ReturnFromFunctionNode, Variant};
use crate::ir::analyse::InferredType;
use crate::ir::symbol::SymbolId;

#[derive(Clone, Debug, PartialEq)]
pub struct AnalyseVariant {}

impl Variant for AnalyseVariant {}

pub type AnalyseNode = crate::common::node::Node<
    AnalyseVariant,
    AnalyseAccessVariableNode,
    AnalyseAccessVariableOfObjectNode,
    AnalyseAccessVariableOfSelfNode,
    AnalyseBlockNode,
    AnalyseBreakLoopNode,
    AnalyseCalculateNode,
    AnalyseCallFunctionNode,
    AnalyseCallFunctionWithLambdaNode,
    AnalyseCallFunctionOfObjectNode,
    AnalyseCallFunctionOfPackageNode,
    AnalyseCompareNode,
    AnalyseContinueLoopNode,
    AnalyseDeclareExternalFunctionNode,
    AnalyseDeclareFunctionNode,
    AnalyseDeclarePackageNode,
    AnalyseDeclareTypeNode,
    AnalyseDeclareVariableNode,
    AnalyseDefineTypeNode,
    AnalyseExportPackageNode,
    AnalyseIfNode,
    AnalyseInterpolateStringNode,
    AnalyseInstantiateTypeNode,
    AnalyseLiteralBooleanNode,
    AnalyseLiteralNumberNode,
    AnalyseLiteralStringNode,
    AnalyseLoopNode,
    AnalyseReturnFromFunctionNode
>;

#[derive(Clone, Debug, PartialEq)]
pub struct AnalyseTreeNode {
    pub node: AnalyseNode,
    pub span: Span,
    pub inferred_type: InferredType,
}


impl AnalyseTreeNode {
    pub fn as_declared_variable(&self) -> &AnalyseDeclareVariableNode {
        if let Node::DeclareVariable(result) = &self.node {
            result
        } else {
            panic!("not declare variable")
        }
    }

    pub fn as_literal_boolean(&self) -> &AnalyseLiteralBooleanNode {
        if let Node::LiteralBoolean(result) = &self.node {
            result
        } else {
            panic!("not literal boolean")
        }
    }

    pub fn as_literal_number(&self) -> &AnalyseLiteralNumberNode {
        if let Node::LiteralNumber(result) = &self.node {
            result
        } else {
            panic!("not literal number")
        }
    }

    pub fn as_literal_string(&self) -> &AnalyseLiteralStringNode {
        if let Node::LiteralString(result) = &self.node {
            result
        } else {
            panic!("not literal string")
        }
    }
}

impl AnalyseTreeNode {
    pub fn node(&self) -> &AnalyseNode { &self.node }
    pub fn node_to_owned(self) -> AnalyseNode { self.node }
}

impl AnalyseTreeNode {
    pub fn new(node: AnalyseNode, span: Span, inferred_type: InferredType) -> AnalyseTreeNode {
        AnalyseTreeNode { node, span, inferred_type }
    }
}

impl WithSpan for AnalyseTreeNode {
    fn span(&self) -> Span {
        self.span.clone()
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseAccessVariableNode {}

impl AccessVariableNode<AnalyseVariant> for AnalyseAccessVariableNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseAccessVariableOfObjectNode {}

impl AccessVariableOfObjectNode<AnalyseVariant> for AnalyseAccessVariableOfObjectNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseAccessVariableOfSelfNode {}

impl AccessVariableOfSelfNode<AnalyseVariant> for AnalyseAccessVariableOfSelfNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseBlockNode {}

impl BlockNode<AnalyseVariant> for AnalyseBlockNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseBreakLoopNode {}

impl BreakLoopNode<AnalyseVariant> for AnalyseBreakLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseCalculateNode {}

impl CalculateNode<AnalyseVariant> for AnalyseCalculateNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseCallFunctionNode {}

impl CallFunctionNode<AnalyseVariant> for AnalyseCallFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseCallFunctionWithLambdaNode {}

impl CallFunctionWithLambdaNode<AnalyseVariant> for AnalyseCallFunctionWithLambdaNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseCallFunctionOfObjectNode {}

impl CallFunctionOfObjectNode<AnalyseVariant> for AnalyseCallFunctionOfObjectNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseCallFunctionOfPackageNode {}

impl CallFunctionOfPackageNode<AnalyseVariant> for AnalyseCallFunctionOfPackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseCompareNode {}

impl CompareNode<AnalyseVariant> for AnalyseCompareNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseContinueLoopNode {}

impl ContinueLoopNode<AnalyseVariant> for AnalyseContinueLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseDeclareExternalFunctionNode {}

impl DeclareExternalFunctionNode<AnalyseVariant> for AnalyseDeclareExternalFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseDeclareFunctionNode {}

impl DeclareFunctionNode<AnalyseVariant> for AnalyseDeclareFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseDeclarePackageNode {}

impl DeclarePackageNode<AnalyseVariant> for AnalyseDeclarePackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseDeclareTypeNode {}

impl DeclareTypeNode<AnalyseVariant> for AnalyseDeclareTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseDefineTypeNode {}

impl DefineTypeNode<AnalyseVariant> for AnalyseDefineTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseDeclareVariableNode {
    pub symbol: SymbolId,
    pub value: Rc<AnalyseTreeNode>,
}

impl DeclareVariableNode<AnalyseVariant> for AnalyseDeclareVariableNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseExportPackageNode {}

impl ExportPackageNode<AnalyseVariant> for AnalyseExportPackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseIfNode {}

impl IfNode<AnalyseVariant> for AnalyseIfNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseInterpolateStringNode {}

impl InterpolateStringNode<AnalyseVariant> for AnalyseInterpolateStringNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseInstantiateTypeNode {}

impl InstantiateTypeNode<AnalyseVariant> for AnalyseInstantiateTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseLiteralBooleanNode {
    pub value: bool,
}

impl LiteralBooleanNode<AnalyseVariant> for AnalyseLiteralBooleanNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseLiteralNumberNode {
    pub value: BigDecimal,
}

impl LiteralNumberNode<AnalyseVariant> for AnalyseLiteralNumberNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseLiteralStringNode {
    pub value: StringTableId,
}

impl LiteralStringNode<AnalyseVariant> for AnalyseLiteralStringNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseLoopNode {}

impl LoopNode<AnalyseVariant> for AnalyseLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyseReturnFromFunctionNode {}

impl ReturnFromFunctionNode<AnalyseVariant> for AnalyseReturnFromFunctionNode {}

