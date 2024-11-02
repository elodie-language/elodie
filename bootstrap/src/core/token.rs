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

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),

    UInt8(u8),
    Uint16(u16),
    UInt32(u32),
    UInt64(u64),

    Float32(f32),
    FloatF64(f64),

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