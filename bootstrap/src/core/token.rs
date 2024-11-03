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
    Comment,
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Keyword {
    Break,
    ConsoleLog, // Temporary hack to facilitate easy tests
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
    Identifier,
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
    LeftAngleEquals,   // <=
    RightAngle,        // >
    DoubleRightAngle,  // >>
    RightAngleEquals,  // >=
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
    Equals,            // =
    DoubleEquals,      // ==
    Bang,              // !
    BangEquals,        // !=
    QuestionMark,      // ?
}

#[derive(Debug,Clone, PartialEq, Eq, Hash)]
pub enum Separator {
    Semicolon,         // ;
    Comma,             // ,
    NewLine,
    Whitespace,
}