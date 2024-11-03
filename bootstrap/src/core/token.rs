use crate::core::span::TextSpan;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Literal(Literal),
    Operator(Operator),
    Separator(Separator),
    Comment,
    EOF,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Literal {
    Identifier,
    Number,
    String,
    True,
    False,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Separator {
    Semicolon,         // ;
    Comma,             // ,
    NewLine,
    Whitespace,
}