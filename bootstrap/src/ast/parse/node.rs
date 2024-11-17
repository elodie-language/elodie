use std::ops::Index;
use std::str::FromStr;

use crate::ast::parse::Error;
use crate::ast::token::{LiteralToken, Token, TokenKind};

pub struct RootNode {
    pub nodes: Vec<Node>,
}

impl Index<usize> for RootNode {
    type Output = Node;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

impl From<Vec<Node>> for RootNode {
    fn from(value: Vec<Node>) -> Self {
        Self { nodes: value }
    }
}

impl RootNode {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Block(BlockNode),
    Break(BreakNode),
    Call(CallNode),
    Continue(ContinueNode),
    FunctionDeclaration(FunctionDeclarationNode),
    Identifier(IdentifierNode),
    If(IfNode),
    Infix(InfixNode),
    Let(LetNode),
    Literal(LiteralNode),
    Loop(LoopNode),
    Prefix(PrefixNode),
    Return(ReturnNode),
    Tuple(TupleNode),
    Type(TypeNode),
}

#[derive(Debug, PartialEq)]
pub struct BlockNode {
    pub nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct BreakNode {
    pub token: Token,
    pub result: Option<Box<Node>>,
}

#[derive(Debug, PartialEq)]
pub struct CallNode {
    pub callee: Box<Node>,
    pub arguments: Vec<CallArgument>,
}

#[derive(Debug, PartialEq)]
pub struct CallArgument {
    pub identifier: Option<IdentifierNode>,
    pub node: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct ContinueNode {
    pub token: Token,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclarationNode {
    pub token: Token,
    pub identifier: IdentifierNode,
    pub arguments: Vec<FunctionDeclarationArgumentNode>,
    pub return_type: Option<Box<TypeNode>>,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclarationArgumentNode {
    pub identifier: IdentifierNode,
    pub r#type: Option<Box<TypeNode>>,
}

#[derive(Debug, PartialEq)]
pub struct IdentifierNode(pub Token);

impl IdentifierNode {
    pub fn identifier(&self) -> &str {
        self.0.span.value.as_str()
    }
}

#[derive(Debug, PartialEq)]
pub struct IfNode {
    pub token: Token,
    pub condition: Box<Node>,
    pub then: BlockNode,
    pub otherwise: Option<ElseNode>,
}

#[derive(Debug, PartialEq)]
pub struct ElseNode {
    pub token: Token,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq)]
pub struct InfixNode {
    pub left: Box<Node>,
    pub operator: InfixOperator,
    pub right: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub enum InfixOperator {
    Add(Token),
    Arrow(Token),
    AccessProperty(Token),
    Call(Token),
    Subtract(Token),
    Multiply(Token),
    Divide(Token),
    Modulo(Token),
    Equal(Token),
    NotEqual(Token),
    LessThan(Token),
    LessThanOrEqual(Token),
    GreaterThan(Token),
    GreaterThanOrEqual(Token),
    TypeAscription(Token),
}

impl InfixOperator {
    pub fn token(&self) -> Token {
        match self {
            InfixOperator::Add(t) => t.clone(),
            InfixOperator::Arrow(t) => t.clone(),
            InfixOperator::AccessProperty(t) => t.clone(),
            InfixOperator::Call(t) => t.clone(),
            InfixOperator::Subtract(t) => t.clone(),
            InfixOperator::Multiply(t) => t.clone(),
            InfixOperator::Divide(t) => t.clone(),
            InfixOperator::Modulo(t) => t.clone(),
            InfixOperator::Equal(t) => t.clone(),
            InfixOperator::NotEqual(t) => t.clone(),
            InfixOperator::LessThan(t) => t.clone(),
            InfixOperator::LessThanOrEqual(t) => t.clone(),
            InfixOperator::GreaterThan(t) => t.clone(),
            InfixOperator::GreaterThanOrEqual(t) => t.clone(),
            InfixOperator::TypeAscription(t) => t.clone()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LetNode {
    pub token: Token,
    pub identifier: IdentifierNode,
    pub node: Box<Node>,
    pub r#type: Option<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub enum LiteralNode {
    Number(LiteralNumberNode),
    String(LiteralStringNode),
    Boolean(LiteralBooleanNode),
}

#[derive(Debug, PartialEq)]
pub struct LiteralNumberNode(pub Token);

impl LiteralNumberNode {
    pub fn value(&self) -> crate::ast::parse::Result<f64> {
        f64::from_str(self.0.value())
            .map_err(|_| Error::UnsupportedNumber(self.0.value().to_string()))
    }
}

#[derive(Debug, PartialEq)]
pub struct LiteralStringNode(pub Token);

impl LiteralStringNode {
    pub fn value(&self) -> &str {
        self.0.value()
    }
}

#[derive(Debug, PartialEq)]
pub struct LiteralBooleanNode(pub Token);

impl LiteralBooleanNode {
    pub fn value(&self) -> bool {
        self.0.kind == TokenKind::Literal(LiteralToken::True)
    }
}

#[derive(Debug, PartialEq)]
pub struct LoopNode {
    pub token: Token,
    pub block: BlockNode,
}

#[derive(Debug, PartialEq)]
pub struct PrefixNode {
    pub operator: PrefixOperator,
    pub node: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnNode {
    pub token: Token,
    pub result: Option<Box<Node>>,
}


#[derive(Debug, PartialEq)]
pub enum PrefixOperator {
    Plus(Token),
    Negate(Token),
    Not(Token),
}

#[derive(Debug, PartialEq)]
pub struct TupleNode {
    pub token: Token,
    pub nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum TypeNode {
    Fundamental(TypeFundamentalNode),
    Function(TypeFunctionNode),
}

#[derive(Debug, PartialEq)]
pub enum TypeFundamentalNode {
    Boolean(Token),
    Number(Token),
    String(Token),
}

#[derive(Debug, PartialEq)]
pub struct TypeFunctionNode {
    pub arguments: Vec<TypeFunctionArgumentNode>,
    pub return_type: Option<Box<TypeNode>>,
}

#[derive(Debug, PartialEq)]
pub struct TypeFunctionArgumentNode {
    pub identifier: Option<IdentifierNode>,
    pub r#type: Box<TypeNode>,
}
