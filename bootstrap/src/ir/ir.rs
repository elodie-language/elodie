use std::rc::Rc;

use crate::common::StringCacheIdx;
use crate::ir::modifier::Modifiers;
use crate::parse;
use crate::parse::IdentifierNode;
use crate::r#type::TypeId;

#[derive(Debug)]
pub struct SourceFile {
    // imports
    // exports

    pub body: Vec<Node>,
}

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
    pub function: Identifier,
    pub arguments: Vec<Node>,
}

#[derive(Debug)]
pub struct CallFunctionOfPackageNode {
    pub package: Vec<Identifier>, // [std, io]
    pub function: Identifier,
    pub arguments: Vec<Node>,
}

#[derive(Debug)]
pub struct CallFunctionNode {
    pub function: Identifier,
    pub arguments: Vec<Node>,
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
    pub otherwise: BlockNode,
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

    ValueNumber(f64),
    ValueString(String),
    ValueBoolean(bool),
    ValueUnit,

    DeclareVariable(DeclareVariableNode),
    DeclareFunction(DeclareFunctionNode),
    DeclarePackage(DeclarePackageNode),
    DeclareType(DeclareTypeNode),

    InstantiateType(InstantiateTypeNode),
    DefineType(DefineTypeNode),
}

#[derive(Debug)]
pub struct ReturnFromFunctionNode {
    pub node: Box<Node>,
    pub return_type_id: TypeId,
}


#[derive(Debug)]
pub struct LoadValueNode {
    pub identifier: Identifier,
    pub type_id: TypeId,
}

#[derive(Clone, Debug)]
pub struct ItselfNode();

#[derive(Clone, Debug)]
pub struct Identifier(pub StringCacheIdx);

impl From<parse::IdentifierNode> for Identifier {
    fn from(value: IdentifierNode) -> Self {
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
pub struct FunctionArgumentNode {
    pub identifier: Identifier,
    pub type_id: TypeId,
}

pub struct TypedNode {
    pub type_id: TypeId,
    pub node: Node,
}

#[derive(Debug)]
pub struct DeclarePackageNode {
    pub identifier: Identifier,
    pub modifiers: Modifiers,
    pub functions: Vec<DeclareFunctionNode>,
    pub packages: Vec<DeclarePackageNode>,
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
    LocalFile(SourceLocalFileNode)
}

#[derive(Debug)]
pub struct SourceLocalFileNode {
    pub path: String,
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
}

#[derive(Debug)]
pub struct LoadValueFromSelfNode {
    pub property: Identifier,
}