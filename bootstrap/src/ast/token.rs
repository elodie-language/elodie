use crate::ast::token::TokenKind::{EOF, Identifier};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn is_eof(&self) -> bool { self.kind == EOF }
    pub fn is_literal(&self, literal: LiteralToken) -> bool { self.kind == TokenKind::Literal(literal) }
    pub fn is_keyword(&self, keyword: KeywordToken) -> bool { self.kind == TokenKind::Keyword(keyword) }
    pub fn is_operator(&self, operator: OperatorToken) -> bool { self.kind == TokenKind::Operator(operator) }
    pub fn value(&self) -> &str { return self.span.value.as_str(); }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Keyword(KeywordToken),
    Literal(LiteralToken),
    Operator(OperatorToken),
    Separator(SeparatorToken),
    Identifier,
    EOF,
}


pub fn eof() -> TokenKind { EOF }

pub fn identifier() -> TokenKind { Identifier }

pub fn keyword(keyword: KeywordToken) -> TokenKind { TokenKind::Keyword(keyword) }

pub fn literal(literal: LiteralToken) -> TokenKind { TokenKind::Literal(literal) }

pub fn operator(operator: OperatorToken) -> TokenKind { TokenKind::Operator(operator) }

pub fn separator(separator: SeparatorToken) -> TokenKind { TokenKind::Separator(separator) }

pub fn test_token(kind: TokenKind, value: &str) -> Token {
    Token {
        kind,
        span: TextSpan {
            start: Position::new(Row(1), Column(1), Index(0)),
            end: Position::new(Row(1), Column(1 + value.len()), Index(value.len())),
            value: value.to_string(),
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordToken {
    Break,
    Const,
    Continue,
    Else,
    Export,
    From,
    For,
    Function,
    If,
    Implement,
    Import,
    In,
    Let,
    Loop,
    Readonly,
    Return,
    Trait,
    Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralToken {
    Number,
    String,
    True,
    False,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperatorToken {
    OpenParen,         // (
    CloseParen,        // )
    OpenCurly,         // {
    CloseCurly,        // }
    OpenBracket,       // [
    CloseBracket,      // ]
    LeftAngle,         // <
    DoubleLeftAngle,   // <<
    LeftAngleEqual,   // <=
    RightAngle,        // >
    DoubleRightAngle,  // >>
    RightAngleEqual,  // >=
    Dot,               // .
    Colon,             // :
    DoubleColon,       // ::
    Arrow,             // ->
    DoubleDot,         // ..
    Plus,              // +
    Minus,             // -
    Asterisk,          // *
    Slash,             // /
    Ampersand,         // &
    DoubleAmpersand,   // &&
    Pipe,              // |
    DoublePipe,        // ||
    Caret,             // ^
    Percent,           // %
    Equal,            // =
    DoubleEqual,      // ==
    Bang,              // !
    BangEqual,        // !=
    QuestionMark,      // ?
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SeparatorToken {
    Semicolon,         // ;
    Comma,             // ,
    NewLine,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextSpan {
    pub start: Position,
    pub end: Position,
    pub value: String,
}

impl TextSpan {
    pub fn new(start: Position, end: Position, value: String) -> Self {
        Self {
            start,
            end,
            value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Row(pub usize);

impl PartialEq<usize> for Row {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Column(pub usize);

impl PartialEq<usize> for Column {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Index(pub usize);

impl PartialEq<usize> for Index {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub row: Row,
    pub column: Column,
    pub index: Index,
}

impl Position {
    pub fn new(row: Row, column: Column, index: Index) -> Self {
        Self {
            row,
            column,
            index,
        }
    }
}

impl PartialEq<(usize, usize, usize)> for Position {
    fn eq(&self, other: &(usize, usize, usize)) -> bool {
        self.row == other.0 &&
            self.column == other.1 &&
            self.index == other.2
    }
}