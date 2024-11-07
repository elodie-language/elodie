use crate::core::span::TextSpan;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Keyword(Keyword),
    Literal(Literal),
    Operator(Operator),
    Separator(Separator),
    Identifier,
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Keyword {
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
pub enum Literal {
    Number,
    String,
    True,
    False,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
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

#[derive(Debug,Clone, PartialEq, Eq, Hash)]
pub enum Separator {
    Semicolon,         // ;
    Comma,             // ,
    NewLine,
}