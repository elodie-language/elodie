use std::rc::Rc;

use bigdecimal::BigDecimal;

use crate::common::{Span, StringTableId, SymbolId, TypeId, WithSpan};
use crate::common::node::{AccessVariableNode, AccessVariableOfObjectNode, AccessVariableOfSelfNode, BlockNode, BreakLoopNode, CalculateNode, CallFunctionNode, CallFunctionOfObjectNode, CallFunctionOfPackageNode, CallFunctionWithLambdaNode, CompareNode, CompareOperator, ContinueLoopNode, DeclareExternalFunctionNode, DeclareFunctionNode, DeclarePackageNode, DeclareTypeNode, DeclareVariableNode, DefineTypeNode, ExportPackageNode, IfNode, InstantiateTypeNode, InterpolateStringNode, LiteralBooleanNode, LiteralFloat4Node, LiteralFloat8Node, LiteralInt16Node, LiteralInt1Node, LiteralInt2Node, LiteralInt4Node, LiteralInt8Node, LiteralNumberNode, LiteralStringNode, LiteralUint16Node, LiteralUint1Node, LiteralUint2Node, LiteralUint4Node, LiteralUint8Node, LoopNode, Node, ReturnFromFunctionNode, Variant};

#[derive(Clone, Debug, PartialEq)]
pub struct IrVariant {}

impl Variant for IrVariant {}

pub type IrNode = Node<
    IrVariant,
    IrAccessVariableNode,
    IrAccessVariableOfObjectNode,
    IrAccessVariableOfSelfNode,
    IrBlockNode,
    IrBreakLoopNode,
    IrCalculateNode,
    IrCallFunctionNode,
    IrCallFunctionWithLambdaNode,
    IrCallFunctionOfObjectNode,
    IrCallFunctionOfPackageNode,
    IrCompareNode,
    IrContinueLoopNode,
    IrDeclareExternalFunctionNode,
    IrDeclareFunctionNode,
    IrDeclarePackageNode,
    IrDeclareTypeNode,
    IrDeclareVariableNode,
    IrDefineTypeNode,
    IrExportPackageNode,
    IrIfNode,
    IrInterpolateStringNode,
    IrInstantiateTypeNode,
    IrLiteralBooleanNode,
    IrLiteralFloat4Node,
    IrLiteralFloat8Node,
    IrLiteralInt1Node,
    IrLiteralInt2Node,
    IrLiteralInt4Node,
    IrLiteralInt8Node,
    IrLiteralInt16Node,
    IrLiteralNumberNode,
    IrLiteralStringNode,
    IrLiteralUint1Node,
    IrLiteralUint2Node,
    IrLiteralUint4Node,
    IrLiteralUint8Node,
    IrLiteralUint16Node,
    IrLoopNode,
    IrReturnFromFunctionNode,
>;

#[derive(Clone, Debug, PartialEq)]
pub struct IrTreeNode {
    pub node: IrNode,
    pub span: Span,
    pub type_id: TypeId,
}

impl IrTreeNode {
    pub fn as_access_variable(&self) -> &IrAccessVariableNode {
        if let Node::AccessVariable(result) = &self.node {
            result
        } else {
            panic!("not access variable")
        }
    }

    pub fn as_declare_variable(&self) -> &IrDeclareVariableNode {
        if let Node::DeclareVariable(result) = &self.node {
            result
        } else {
            panic!("not declare variable")
        }
    }

    pub fn as_literal_boolean(&self) -> &IrLiteralBooleanNode {
        if let Node::LiteralBoolean(result) = &self.node {
            result
        } else {
            panic!("not literal boolean")
        }
    }

    pub fn as_literal_number(&self) -> &IrLiteralNumberNode {
        if let Node::LiteralNumber(result) = &self.node {
            result
        } else {
            panic!("not literal number")
        }
    }

    pub fn as_literal_string(&self) -> &IrLiteralStringNode {
        if let Node::LiteralString(result) = &self.node {
            result
        } else {
            panic!("not literal string")
        }
    }
}

impl IrTreeNode {
    pub fn node(&self) -> &IrNode {
        &self.node
    }
    pub fn node_to_owned(self) -> IrNode {
        self.node
    }
}

impl IrTreeNode {
    pub fn new(node: IrNode, span: Span, type_id: TypeId) -> IrTreeNode {
        IrTreeNode {
            node,
            span,
            type_id,
        }
    }
}

impl WithSpan for IrTreeNode {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrAccessVariableNode {
    pub variable: SymbolId,
}

impl AccessVariableNode<IrVariant> for IrAccessVariableNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrAccessVariableOfObjectNode {}

impl AccessVariableOfObjectNode<IrVariant> for IrAccessVariableOfObjectNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrAccessVariableOfSelfNode {}

impl AccessVariableOfSelfNode<IrVariant> for IrAccessVariableOfSelfNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrBlockNode {
    pub nodes: Box<[Rc<IrTreeNode>]>,
}

impl BlockNode<IrVariant> for IrBlockNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrBreakLoopNode {}

impl BreakLoopNode<IrVariant> for IrBreakLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrCalculateNode {}

impl CalculateNode<IrVariant> for IrCalculateNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrCallFunctionNode {}

impl CallFunctionNode<IrVariant> for IrCallFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrCallFunctionWithLambdaNode {}

impl CallFunctionWithLambdaNode<IrVariant> for IrCallFunctionWithLambdaNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrCallFunctionOfObjectNode {}

impl CallFunctionOfObjectNode<IrVariant> for IrCallFunctionOfObjectNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrCallFunctionOfPackageNode {
    pub package: SymbolId,
    pub function: SymbolId,
    pub arguments: Box<[Rc<IrTreeNode>]>,
}

impl CallFunctionOfPackageNode<IrVariant> for IrCallFunctionOfPackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrCompareNode {
    pub left: Rc<IrTreeNode>,
    pub operator: CompareOperator,
    pub right: Rc<IrTreeNode>,
}

impl CompareNode<IrVariant> for IrCompareNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrContinueLoopNode {}

impl ContinueLoopNode<IrVariant> for IrContinueLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrDeclareExternalFunctionNode {}

impl DeclareExternalFunctionNode<IrVariant> for IrDeclareExternalFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrDeclareFunctionNode {}

impl DeclareFunctionNode<IrVariant> for IrDeclareFunctionNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrDeclarePackageNode {}

impl DeclarePackageNode<IrVariant> for IrDeclarePackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrDeclareTypeNode {}

impl DeclareTypeNode<IrVariant> for IrDeclareTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrDefineTypeNode {}

impl DefineTypeNode<IrVariant> for IrDefineTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrDeclareVariableNode {
    pub variable: SymbolId,
    pub value: Rc<IrTreeNode>,
}

impl DeclareVariableNode<IrVariant> for IrDeclareVariableNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrExportPackageNode {}

impl ExportPackageNode<IrVariant> for IrExportPackageNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrIfNode {
    pub condition: Rc<IrTreeNode>,
    pub then: Rc<IrBlockNode>,
    pub otherwise: Option<Rc<IrBlockNode>>,
}

impl IfNode<IrVariant> for IrIfNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrInterpolateStringNode {
    pub nodes: Box<[Rc<IrTreeNode>]>,
}

impl InterpolateStringNode<IrVariant> for IrInterpolateStringNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrInstantiateTypeNode {}

impl InstantiateTypeNode<IrVariant> for IrInstantiateTypeNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralBooleanNode {
    pub value: bool,
}

impl LiteralBooleanNode<IrVariant> for IrLiteralBooleanNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralFloat4Node {
    pub value: f32,
}

impl LiteralFloat4Node<IrVariant> for IrLiteralFloat4Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralFloat8Node {
    pub value: f64,
}

impl LiteralFloat8Node<IrVariant> for IrLiteralFloat8Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralInt1Node {
    pub value: i8,
}

impl LiteralInt1Node<IrVariant> for IrLiteralInt1Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralInt2Node {
    pub value: i16,
}

impl LiteralInt2Node<IrVariant> for IrLiteralInt2Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralInt4Node {
    pub value: i32,
}

impl LiteralInt4Node<IrVariant> for IrLiteralInt4Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralInt8Node {
    pub value: i64,
}

impl LiteralInt8Node<IrVariant> for IrLiteralInt8Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralInt16Node {
    pub value: i128,
}

impl LiteralInt16Node<IrVariant> for IrLiteralInt16Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralNumberNode {
    pub value: BigDecimal,
}

impl LiteralNumberNode<IrVariant> for IrLiteralNumberNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralStringNode {
    pub value: StringTableId,
}

impl LiteralStringNode<IrVariant> for IrLiteralStringNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralUint1Node {
    pub value: u8,
}

impl LiteralUint1Node<IrVariant> for IrLiteralUint1Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralUint2Node {
    pub value: u16,
}

impl LiteralUint2Node<IrVariant> for IrLiteralUint2Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralUint4Node {
    pub value: u32,
}

impl LiteralUint4Node<IrVariant> for IrLiteralUint4Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralUint8Node {
    pub value: u64,
}

impl LiteralUint8Node<IrVariant> for IrLiteralUint8Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLiteralUint16Node {
    pub value: u128,
}

impl LiteralUint16Node<IrVariant> for IrLiteralUint16Node {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrLoopNode {}

impl LoopNode<IrVariant> for IrLoopNode {}

#[derive(Debug, Clone, PartialEq)]
pub struct IrReturnFromFunctionNode {}

impl ReturnFromFunctionNode<IrVariant> for IrReturnFromFunctionNode {}
