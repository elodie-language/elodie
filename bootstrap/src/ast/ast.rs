use std::ops::Deref;

use crate::ast::r#type::TypeId;

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
pub struct BreakNode {
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
    GreaterThan
}

#[derive(Debug)]
pub struct CalculateNode {
    pub left: Box<Node>,
    pub operator: CalculationOperator,
    pub right: Box<Node>,
}

#[derive(Debug)]
pub enum CalculationOperator {
    Multiply
}

#[derive(Debug)]
pub struct ContinueNode {}

#[derive(Debug)]
pub struct CallFunctionOfObjectNode {
    pub object: Identifier,
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
    Break(BreakNode),
    Calculate(CalculateNode),
    CallFunctionOfObject(CallFunctionOfObjectNode),
    Continue(ContinueNode),
    Compare(CompareNode),

    If(IfNode),

    LoadVariable(LoadVariableNode),
    Loop(LoopNode),

    ValueNumber(f64),
    ValueString(String),
    ValueBoolean(bool),

    DeclareVariable(DeclareVariableNode),
}


#[derive(Debug)]
pub struct LoadVariableNode {
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