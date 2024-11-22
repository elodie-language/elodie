use std::ops::Index;
use std::str::FromStr;

use crate::ast::modifier::Modifiers;
use crate::ast::parse::Error;
use crate::ast::lex::token::{LiteralToken, Token, TokenKind};

#[derive(Debug)]
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
    From(FromNode),
    FunctionDeclaration(FunctionDeclarationNode),
    Identifier(IdentifierNode),
    If(IfNode),
    Infix(InfixNode),
    Let(LetNode),
    Literal(LiteralNode),
    Loop(LoopNode),
    Nop,
    PackageDeclaration(PackageDeclarationNode),
    Prefix(PrefixNode),
    Return(ReturnNode),
    Tuple(TupleNode),
    Type(TypeNode),
    TypeDeclaration(TypeDeclarationNode),
}

impl Node {
    pub fn as_block(&self) -> &BlockNode { if let Node::Block(result) = self { result } else { panic!("not block") } }
    pub fn as_break(&self) -> &BreakNode { if let Node::Break(result) = self { result } else { panic!("not break") } }
    pub fn as_call(&self) -> &CallNode { if let Node::Call(result) = self { result } else { panic!("not call") } }
    pub fn as_continue(&self) -> &ContinueNode { if let Node::Continue(result) = self { result } else { panic!("not continue") } }
    pub fn as_from(&self) -> &FromNode { if let Node::From(result) = self { result } else { panic!("not from") } }
    pub fn as_function_declaration(&self) -> &FunctionDeclarationNode {
        if let Node::FunctionDeclaration(result) = self { result } else { panic!("not function declaration") }
    }
    pub fn as_identifier(&self) -> &IdentifierNode {
        if let Node::Identifier(result) = self { result } else { panic!("not identifier") }
    }
    pub fn as_if(&self) -> &IfNode {
        if let Node::If(result) = self { result } else { panic!("not if") }
    }
    pub fn as_infix(&self) -> &InfixNode {
        if let Node::Infix(result) = self { result } else { panic!("not infix") }
    }
    pub fn as_let(&self) -> &LetNode {
        if let Node::Let(result) = self { result } else { panic!("not let") }
    }
    pub fn as_literal(&self) -> &LiteralNode {
        if let Node::Literal(result) = self { result } else { panic!("not literal") }
    }
    pub fn as_loop(&self) -> &LoopNode {
        if let Node::Loop(result) = self { result } else { panic!("not loop") }
    }

    pub fn as_package_declaration(&self) -> &PackageDeclarationNode {
        if let Node::PackageDeclaration(result) = self { result } else { panic!("not package declaration") }
    }
    pub fn as_prefix(&self) -> &PrefixNode {
        if let Node::Prefix(result) = self { result } else { panic!("not prefix") }
    }
    pub fn as_return(&self) -> &ReturnNode {
        if let Node::Return(result) = self { result } else { panic!("not return") }
    }
    pub fn as_tuple(&self) -> &TupleNode {
        if let Node::Tuple(result) = self { result } else { panic!("not tuple") }
    }
    pub fn as_type(&self) -> &TypeNode {
        if let Node::Type(result) = self { result } else { panic!("not type") }
    }
    pub fn as_type_declaration(&self) -> &TypeDeclarationNode {
        if let Node::TypeDeclaration(result) = self { result } else { panic!("not type declaration") }
    }
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

impl BreakNode {
    pub fn as_result(&self) -> &Node { if let Some(ref node) = self.result { node } else { panic!() } }
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
pub struct FromExportNode {
    pub token: Token,
    pub from_node: Box<Node>,
    pub what_node: Box<Node>,
}


#[derive(Debug, PartialEq)]
pub enum FromNode {
    Export(FromExportNode)
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclarationNode {
    pub token: Token,
    pub identifier: IdentifierNode,
    pub arguments: Vec<FunctionDeclarationArgumentNode>,
    pub return_type: Option<Box<TypeNode>>,
    pub block: BlockNode,
    pub modifiers: Modifiers,
}

impl FunctionDeclarationNode {
    pub fn as_return_type(&self) -> &TypeNode { if let Some(ref node) = self.return_type { node } else { panic!() } }
}


#[derive(Debug, PartialEq)]
pub struct FunctionDeclarationArgumentNode {
    pub identifier: IdentifierNode,
    pub r#type: Option<Box<TypeNode>>,
}

impl FunctionDeclarationArgumentNode {
    pub fn as_type(&self) -> &TypeNode { if let Some(ref node) = self.r#type { node } else { panic!() } }
}

#[derive(Debug, PartialEq)]
pub struct IdentifierNode(pub Token);

impl IdentifierNode {
    pub fn value(&self) -> &str {
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
    AccessPackage(Token),
    AccessProperty(Token),
    Assign(Token),
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
            InfixOperator::AccessPackage(t) => t.clone(),
            InfixOperator::AccessProperty(t) => t.clone(),
            InfixOperator::Assign(t) => t.clone(),
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
pub struct PackageDeclarationNode {
    pub token: Token,
    pub identifier: IdentifierNode,
    pub block: BlockNode,
    pub modifiers: Modifiers,
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

impl ReturnNode {
    pub fn as_result(&self) -> &Node { if let Some(ref node) = self.result { node } else { panic!() } }
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
    Custom(TypeCustomNode),
}

#[derive(Debug, PartialEq)]
pub enum TypeFundamentalNode {
    Boolean(Token),
    Number(Token),
    String(Token),
}

#[derive(Debug, PartialEq)]
pub struct TypeCustomNode {
    pub token: Token,
}

#[derive(Debug, PartialEq)]
pub struct TypeFunctionNode {
    pub arguments: Vec<TypeFunctionArgumentNode>,
    pub return_type: Option<Box<TypeNode>>,
}

impl TypeFunctionNode {
    pub fn as_return_type(&self) -> &TypeNode { if let Some(ref node) = self.return_type { node } else { panic!() } }
}

#[derive(Debug, PartialEq)]
pub struct TypeFunctionArgumentNode {
    pub identifier: Option<IdentifierNode>,
    pub r#type: Box<TypeNode>,
}

#[derive(Debug, PartialEq)]
pub struct TypeDeclarationNode {
    pub token: Token,
    pub identifier: IdentifierNode,
    pub properties: TupleNode,
    pub modifiers: Modifiers,
}