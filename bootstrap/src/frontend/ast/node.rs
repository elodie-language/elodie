use std::rc::Rc;

use crate::common::{PackagePath, StringTableId};
use crate::common::TypeId;
use crate::frontend::parse;
use crate::frontend::parse::IdentifierNode;
use crate::ir::Modifiers;

#[derive(Debug)]
pub struct BlockNode {
    pub body: Vec<Node>,
    pub return_type: TypeId,
}

#[derive(Debug)]
pub struct BreakLoopNode {
    pub body: Option<Box<Node>>,
    pub return_type: TypeId,
}

#[derive(Debug)]
pub struct CompareNode {
    pub left: Box<Node>,
    pub operator: CompareOperator,
    pub right: Box<Node>,
}

#[derive(Debug)]
pub enum CompareOperator {
    Equal,
    NotEqual,
    GreaterThan,
}

#[derive(Debug)]
pub struct CalculateNode {
    pub left: Box<Node>,
    pub operator: CalculationOperator,
    pub right: Box<Node>,
}

#[derive(Debug)]
pub enum CalculationOperator {
    Add,
    Multiply,
}

#[derive(Debug)]
pub struct ContinueLoopNode {}

#[derive(Debug)]
pub struct CallFunctionOfObjectNode {
    pub object: Identifier,
    // FIXME object_tid : TypeId
    pub function: Identifier,
    // FIXME function_tid: TypeId
    pub arguments: Vec<Node>,
}

#[derive(Debug)]
pub struct CallFunctionOfPackageNode {
    pub package: PackagePath,
    pub function: Identifier,
    pub arguments: Vec<Node>,
}

#[derive(Debug)]
pub struct CallFunctionNode {
    pub function: Identifier,
    pub arguments: Vec<Node>,
}

#[derive(Debug)]
pub struct CallFunctionWithLambdaNode {
    pub call_function: CallFunctionNode,
    pub lambda: Rc<BlockNode>,
}

#[derive(Debug)]
pub struct ExportPackageNode {
    pub identifier: Identifier,
    pub source: Source,
}

#[derive(Debug)]
pub struct IfNode {
    pub condition: Box<Node>,
    pub then: BlockNode,
    pub otherwise: Option<BlockNode>,
    pub return_type: TypeId,
}

#[derive(Debug)]
pub struct LoopNode {
    pub body: Vec<Node>,
    pub return_type: TypeId,
}

#[derive(Debug)]
pub enum Node {
    Block(BlockNode),
    BreakLoop(BreakLoopNode),

    Calculate(CalculateNode),
    CallFunctionOfObject(CallFunctionOfObjectNode),
    CallFunctionOfPackage(CallFunctionOfPackageNode),
    CallFunction(CallFunctionNode),
    CallFunctionWithLambda(CallFunctionWithLambdaNode),

    ExportPackage(ExportPackageNode),

    ReturnFromFunction(ReturnFromFunctionNode),

    ContinueLoop(ContinueLoopNode),
    Compare(CompareNode),

    If(IfNode),
    // Itself(ItselfNode),
    LoadValue(LoadValueNode),
    LoadValueFromObject(LoadValueFromObjectNode),
    LoadValueFromSelf(LoadValueFromSelfNode),
    Loop(LoopNode),

    Literal(LiteralNode),
    Unit,

    DeclareVariable(DeclareVariableNode),
    DeclareFunction(DeclareFunctionNode),
    DeclareExternalFunction(DeclareExternalFunctionNode),
    DeclarePackage(DeclarePackageNode),
    DeclareType(DeclareTypeNode),

    InstantiateType(InstantiateTypeNode),
    DefineType(DefineTypeNode),

    InterpolateString(InterpolateStringNode),
}

#[derive(Debug)]
pub enum LiteralNode {
    Bool(LiteralBoolNode),
    Number(LiteralNumberNode),
    String(LiteralStringNode),
}

impl LiteralNode {
    pub fn ty(&self) -> TypeId {
        match self {
            LiteralNode::Bool(n) => n.ty,
            LiteralNode::Number(n) => n.ty,
            LiteralNode::String(n) => n.ty
        }
    }
}

#[derive(Debug)]
pub struct LiteralBoolNode {
    pub value: bool,
    pub ty: TypeId,
}

#[derive(Debug)]
pub struct LiteralNumberNode {
    pub value: StringTableId,
    pub ty: TypeId,
}

#[derive(Debug)]
pub struct LiteralStringNode {
    pub value: StringTableId,
    pub ty: TypeId,
}

#[derive(Debug)]
pub struct ReturnFromFunctionNode {
    pub node: Box<Node>,
    pub return_type_id: TypeId,
}

#[derive(Debug)]
pub struct LoadValueNode {
    pub identifier: Identifier,
    pub ty: TypeId,
}

#[derive(Clone, Debug)]
pub struct ItselfNode();

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Identifier(pub StringTableId);

impl From<parse::IdentifierNode> for Identifier {
    fn from(value: IdentifierNode) -> Self {
        Identifier(value.0.span.value)
    }
}

impl From<Rc<parse::IdentifierNode>> for Identifier {
    fn from(value: Rc<IdentifierNode>) -> Self {
        Identifier(value.0.span.value)
    }
}


impl From<&parse::IdentifierNode> for Identifier {
    fn from(value: &IdentifierNode) -> Self {
        Identifier(value.0.value())
    }
}

impl AsRef<Identifier> for Identifier {
    fn as_ref(&self) -> &Identifier {
        &self
    }
}

#[derive(Debug)]
pub struct DeclareVariableNode {
    pub identifier: Identifier,
    pub value: Box<Node>,
    pub value_type: TypeId,
}

#[derive(Debug)]
pub struct DeclareFunctionNode {
    pub identifier: Identifier,
    pub arguments: Vec<Rc<FunctionArgumentNode>>,
    pub return_type: TypeId,
    pub body: Rc<BlockNode>,
}

#[derive(Debug)]
pub struct DeclareExternalFunctionNode {
    pub identifier: Identifier,
    pub arguments: Vec<Rc<FunctionArgumentNode>>,
    pub return_type: TypeId,
}

#[derive(Debug)]
pub struct FunctionArgumentNode {
    pub identifier: Identifier,
    pub ty: TypeId,
}

pub struct TypedNode {
    pub ty: TypeId,
    pub node: Node,
}

#[derive(Debug)]
pub struct DeclarePackageNode {
    pub identifier: Identifier,
    pub modifiers: Modifiers,
    pub external_functions: Vec<DeclareExternalFunctionNode>,
    pub functions: Vec<DeclareFunctionNode>,
    pub packages: Vec<DeclarePackageNode>,
    pub definitions: Vec<DefineTypeNode>,
}

#[derive(Debug)]
pub struct DeclareTypeNode {
    pub identifier: Identifier,
    pub modifiers: Modifiers,
    pub properties: Vec<DeclarePropertyNode>,
}

#[derive(Debug)]
pub struct DeclarePropertyNode {
    pub identifier: Identifier,
    pub r#type: TypeId,
}

#[derive(Debug)]
pub struct DefineTypeNode {
    pub identifier: Identifier,
    pub modifiers: Modifiers,
    pub functions: Vec<DeclareFunctionNode>,
}

#[derive(Debug)]
pub enum Source {
    LocalFile(SourceLocalFileNode),
}

#[derive(Debug)]
pub struct SourceLocalFileNode {
    pub path: String,
}

// FIXME compiler should give a hint whether the interpolated string will be used locally only and whether it is small enough to be allocated on the stack only
#[derive(Debug)]
pub struct InterpolateStringNode {
    pub nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct InstantiateTypeNode {
    pub type_id: TypeId,
    pub type_name: Identifier,
    pub arguments: Vec<NamedArgumentNode>,
}

#[derive(Debug)]
pub struct NamedArgumentNode {
    pub identifier: Identifier,
    pub value: Node,
}

#[derive(Debug)]
pub struct LoadValueFromObjectNode {
    pub object: Identifier,
    pub property: Identifier,
    pub ty: TypeId,
}

#[derive(Debug)]
pub struct LoadValueFromSelfNode {
    pub property: Identifier,
}
