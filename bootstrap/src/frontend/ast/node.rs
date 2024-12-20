use std::hash::Hash;
use std::rc::Rc;

use crate::common::{Column, Index, PackagePath, Position, Row, Span, WithSpan};
use crate::frontend::lex::token::Token;
use crate::frontend::modifier::Modifiers;

pub trait Ast<T: Ast<T>>: Clone {
    fn node(&self) -> &Node<T>;
    fn node_mut(&mut self) -> &mut Node<T>;
    fn node_to_owned(self) -> Node<T>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstNode {
    node: Node<AstNode>,
    span: Span,
}

impl AstNode {
    pub fn new(node: Node<AstNode>, span: Span) -> AstNode {
        AstNode { node, span }
    }
}

pub static SPAN_NOT_IMPLEMENTED: Span = Span {
    start: Position {
        row: Row(0),
        column: Column(0),
        index: Index(0),
    },
    end: Position {
        row: Row(0),
        column: Column(0),
        index: Index(0),
    },
};

impl Ast<AstNode> for AstNode {
    fn node(&self) -> &Node<AstNode> { &self.node }
    fn node_mut(&mut self) -> &mut Node<AstNode> { &mut self.node }
    fn node_to_owned(self) -> Node<AstNode> { self.node }
}

impl WithSpan for AstNode {
    fn span(&self) -> Span { self.span.clone() }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node<T: Ast<T>> {
    AccessVariable(AccessVariableNode),
    AccessVariableOfObject(AccessVariableOfObject),
    AccessVariableOfSelf(AccessVariableOfSelf),

    Block(BlockNode<T>),
    BreakLoop(BreakLoopNode<T>),

    Calculate(CalculateNode<T>),

    CallFunction(CallFunctionNode<T>),
    CallFunctionWithLambda(CallFunctionWithLambdaNode<T>),
    CallFunctionOfObject(CallFunctionOfObjectNode<T>),
    CallFunctionOfPackage(CallFunctionOfPackageNode<T>),

    Compare(CompareNode<T>),
    ContinueLoop,

    DeclareExternalFunction(DeclareExternalFunctionNode),
    DeclareFunction(DeclareFunctionNode<T>),
    DeclarePackage(DeclarePackageNode<T>),
    DeclareType(DeclareTypeNode),
    DeclareVariable(DeclareVariableNode<T>),

    DefineType(DefineTypeNode<T>),

    ExportPackage(ExportPackageNode),

    If(IfNode<T>),
    InterpolateString(InterpolateStringNode<T>),
    InstantiateType(InstantiateTypeNode<T>),

    LiteralBoolean(LiteralBooleanNode),
    LiteralNumber(LiteralNumberNode),
    LiteralString(LiteralStringNode),

    Loop(LoopNode<T>),

    ReturnFromFunction(ReturnFromFunctionNode<T>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccessVariableNode {
    pub variable: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccessVariableOfObject {
    pub object: Identifier,
    pub variable: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccessVariableOfSelf {
    pub variable: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockNode<T: Ast<T>> {
    pub nodes: Vec<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakLoopNode<T: Ast<T>> {
    pub node: Option<Rc<T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalculateNode<T: Ast<T>> {
    pub left: Rc<T>,
    pub operator: CalculationOperator,
    pub right: Rc<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallFunctionNode<T: Ast<T>> {
    pub function: Identifier,
    pub arguments: Vec<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallFunctionWithLambdaNode<T: Ast<T>> {
    pub function: Identifier,
    pub arguments: Vec<T>,
    pub lambda: Vec<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallFunctionOfObjectNode<T: Ast<T>> {
    pub object: Identifier,
    pub function: Identifier,
    pub arguments: Vec<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallFunctionOfPackageNode<T: Ast<T>> {
    pub package: PackagePath,
    pub function: Identifier,
    pub arguments: Vec<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompareNode<T: Ast<T>> {
    pub left: Rc<T>,
    pub operator: CompareOperator,
    pub right: Rc<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclareExternalFunctionNode {
    pub function: Identifier,
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Option<AstType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclareFunctionNode<T: Ast<T>> {
    pub function: Identifier,
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Option<AstType>,
    pub body: BlockNode<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclarePackageNode<T: Ast<T>> {
    pub package: Identifier,
    pub modifiers: Modifiers,
    pub external_functions: Vec<DeclareExternalFunctionNode>,
    pub functions: Vec<DeclareFunctionNode<T>>,
    pub packages: Vec<DeclarePackageNode<T>>,
    pub types: Vec<DeclareTypeNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclareTypeNode {
    pub r#type: Identifier,
    pub modifiers: Modifiers,
    pub variables: Vec<TypeVariable>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefineTypeNode<T: Ast<T>> {
    pub r#type: Identifier,
    pub modifiers: Modifiers,
    pub functions: Vec<DeclareFunctionNode<T>>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct DeclareVariableNode<T: Ast<T>> {
    pub variable: Identifier,
    pub value: Rc<T>,
    pub value_type: Option<AstType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportPackageNode {
    pub package: PackagePath,
    pub source: Source,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfNode<T: Ast<T>> {
    pub condition: Rc<T>,
    pub then: Rc<T>,
    pub otherwise: Option<Rc<T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterpolateStringNode<T: Ast<T>> {
    pub values: Vec<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstantiateTypeNode<T: Ast<T>> {
    pub r#type: Identifier,
    pub arguments: Vec<NamedArgument<T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralBooleanNode(pub Token);

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralNumberNode(pub Token);

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralStringNode(pub Token);

#[derive(Debug, Clone, PartialEq)]
pub struct LoopNode<T: Ast<T>> {
    pub nodes: Vec<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnFromFunctionNode<T: Ast<T>> {
    pub node: Rc<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalculationOperator {
    Add,
    Multiply,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompareOperator {
    Equal,
    NotEqual,
    GreaterThan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionArgument {
    pub argument: Identifier,
    pub argument_type: Option<AstType>,
}


#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    LocalFile { path: String },
}


#[derive(Clone, Debug, PartialEq)]
pub struct Identifier(pub Token);

#[derive(Clone, Debug, PartialEq)]
pub struct NamedArgument<T: Ast<T>> {
    pub identifier: Identifier,
    pub value: T,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstType {
    Boolean,
    Object,
    Number,
    String,
    Function { arguments: Vec<Box<AstType>>, return_type: Option<Box<AstType>> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeVariable {
    pub variable: Identifier,
    pub r#type: AstType,
}