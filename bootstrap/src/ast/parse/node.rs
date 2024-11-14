use std::ops::Index;
use std::str::FromStr;

use crate::ast::parse::Error;
use crate::ast::token::{LiteralToken, Token, TokenKind};

pub struct RootNode {
    nodes: Vec<Node>,
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
    Continue(ContinueNode),
    Identifier(IdentifierNode),
    If(IfNode),
    Infix(InfixNode),
    Let(LetNode),
    Literal(LiteralNode),
    Loop(LoopNode),
    Prefix(PrefixNode),
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
pub struct ContinueNode {
    pub token: Token,
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
}

#[derive(Debug, PartialEq)]
pub struct LetNode {
    pub token: Token,
    pub identifier: IdentifierNode,
    pub value: Box<Node>,
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
pub enum PrefixOperator {
    Plus(Token),
    Negate(Token),
    Not(Token),
}

#[derive(Debug, PartialEq)]
pub enum TypeNode {}