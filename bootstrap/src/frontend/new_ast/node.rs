use std::hash::Hash;
use std::rc::Rc;

use crate::common::{Column, Index, PackagePath, Position, Row, Span, WithSpan};
use crate::frontend::lex::token::Token;

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
    AccessVariable { variable: Identifier },
    AccessVariableOfObject { object: Identifier, variable: Identifier },
    AccessVariableOfSelf { variable: Identifier },

    Block { nodes: Vec<T> },
    BreakLoop { node: Option<Rc<T>> },

    Calculate { left: Rc<T>, operator: CalculationOperator, right: Rc<T> },

    CallFunction { function: Identifier, arguments: Vec<T> },
    CallFunctionWithLambda { function: Identifier, arguments: Vec<T>, lambda: Vec<T> },
    CallFunctionOfObject { object: Identifier, function: Identifier, arguments: Vec<T> },
    CallFunctionOfPackage { package: PackagePath, function: Identifier, arguments: Vec<T> },

    Compare { left: Rc<T>, operator: CompareOperator, right: Rc<T> },
    ContinueLoop {},

    DeclareVariable(DeclareVariableNode<T>),

    ExportPackage { package: PackagePath, source: Source },

    If { condition: Rc<T>, then: Rc<T>, otherwise: Option<Rc<T>> },

    LiteralBoolean(LiteralBooleanNode),
    LiteralNumber(LiteralNumberNode),
    LiteralString(LiteralStringNode),

    Loop { nodes: Vec<T> },

    ReturnFromFunction { node: Rc<T> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccessVariableNode {
    pub variable: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclareVariableNode<T: Clone + Ast<T>> {
    pub variable: Identifier,
    pub value: Rc<T>,
    pub value_type: Option<AstType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralBooleanNode(pub Token);

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralNumberNode(pub Token);

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralStringNode(pub Token);

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
pub enum Source {
    LocalFile { path: String },
}


#[derive(Clone, Debug, PartialEq)]
pub struct Identifier(pub Token);


#[derive(Debug, Clone, PartialEq)]
pub enum AstType {
    Boolean,
    Object,
    Number,
    String,
    Function { arguments: Vec<Box<AstType>>, return_type: Option<Box<AstType>> },
}
