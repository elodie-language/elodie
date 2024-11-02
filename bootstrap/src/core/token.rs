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
    Function,
    Let,
    If,
    Else,
    For,
    In,
    Return,
    Break,
    Continue,
    Import,
    Export,
    Readonly,
    From,
    Loop,
    Implement,
    Type,
    Trait,
    Const,
    ConsoleLog, // Temporary hack to facilitate easy tests
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Identifier,

    S8(i8),
    S16(i16),
    S32(i32),
    S64(i64),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    F32(f32),
    F64(f64),

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
pub enum Separator{
    Semicolon,         // ;
    Comma,             // ,
    NewLine,
    Whitespace,
}