use std::ops::Deref;
use std::rc::Rc;

use crate::ast::r#type::TypeId;

#[derive(Debug)]
pub struct SourceFile {
    // imports
    // exports

    pub body: Vec<Node>,
}

#[derive(Debug)]
pub struct Package {
    pub exports: Vec<Exports>,
    pub body: Vec<Node>,
}

#[derive(Debug)]
pub enum Exports {
    Function(ExportedFunctionSignature)
}

#[derive(Debug)]
pub struct ExportedFunctionSignature {
    pub identifier: Identifier,
    pub arguments: Vec<Rc<FunctionArgumentNode>>,
    pub return_type: TypeId,
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
pub struct CallFunctionNode {
    pub function: Identifier,
    pub arguments: Vec<Node>,
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
    CallFunction(CallFunctionNode),

    ReturnFromFunction(ReturnFromFunctionNode),

    ContinueLoop(ContinueLoopNode),
    Compare(CompareNode),

    If(IfNode),

    UseIdentifier(UseIdentifierNode),
    Loop(LoopNode),

    ValueNumber(f64),
    ValueString(String),
    ValueBoolean(bool),
    ValueUnit,

    DeclareVariable(DeclareVariableNode),
    DeclareFunction(DeclareFunctionNode),
}

#[derive(Debug)]
pub struct ReturnFromFunctionNode {
    pub node: Box<Node>,
    pub return_type_id: TypeId,
}


#[derive(Debug)]
pub struct UseIdentifierNode {
    pub identifier: Identifier,
    pub type_id: TypeId,
}


#[derive(Debug)]
pub struct Identifier(pub String);

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
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