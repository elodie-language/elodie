use crate::common::{Column, Index, Position, Row, StringTableId, Span};
use crate::frontend::context::Context;
use crate::frontend::lex::token::TokenKind::{EOF, Identifier};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub value: StringTableId,

}

impl Token {
    pub fn is_eof(&self) -> bool {
        self.kind == EOF
    }
    pub fn is_identifier(&self) -> bool {
        self.kind == Identifier
    }
    pub fn is_literal(&self, literal: LiteralToken) -> bool {
        self.kind == TokenKind::Literal(literal)
    }
    pub fn is_separator(&self, separator: SeparatorToken) -> bool {
        self.kind == TokenKind::Separator(separator)
    }
    pub fn is_keyword(&self, keyword: KeywordToken) -> bool {
        self.kind == TokenKind::Keyword(keyword)
    }
    pub fn is_operator(&self, operator: OperatorToken) -> bool {
        self.kind == TokenKind::Operator(operator)
    }
    pub fn value(&self) -> StringTableId {
        return self.value;
    }
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

pub fn eof() -> TokenKind {
    EOF
}

pub fn identifier() -> TokenKind {
    Identifier
}

pub fn keyword(keyword: KeywordToken) -> TokenKind {
    TokenKind::Keyword(keyword)
}

pub fn literal(literal: LiteralToken) -> TokenKind {
    TokenKind::Literal(literal)
}

pub fn operator(operator: OperatorToken) -> TokenKind {
    TokenKind::Operator(operator)
}

pub fn separator(separator: SeparatorToken) -> TokenKind {
    TokenKind::Separator(separator)
}

pub fn test_token(ctx: &mut Context, kind: TokenKind, value: &str) -> Token {
    Token {
        kind,
        span: Span {
            start: Position::new(Row(1), Column(1), Index(0)),
            end: Position::new(Row(1), Column(1 + value.len()), Index(value.len())),
        },
        value: ctx.string_table.push_str(value),
    }
}

pub fn test_token_with_offset(
    ctx: &mut Context,
    kind: TokenKind,
    value: &str,
    offset: usize,
) -> Token {
    Token {
        kind,
        span: Span {
            start: Position::new(Row(1), Column(offset + 1), Index(offset)),
            end: Position::new(
                Row(1),
                Column(offset + 1 + value.len()),
                Index(offset + value.len()),
            ),
        },
        value: ctx.string_table.push_str(value),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordToken {
    Break,
    Const,
    Continue,
    Else,
    Export,
    External,
    From,
    For,
    Function,
    If,
    Define,
    Import,
    In,
    Let,
    Loop,
    Package,
    Readonly,
    Return,
    Itself,
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
    OpenParen,        // (
    CloseParen,       // )
    OpenCurly,        // {
    CloseCurly,       // }
    OpenBracket,      // [
    CloseBracket,     // ]
    LeftAngle,        // <
    DoubleLeftAngle,  // <<
    LeftAngleEqual,   // <=
    RightAngle,       // >
    DoubleRightAngle, // >>
    RightAngleEqual,  // >=
    Dot,              // .
    Colon,            // :
    DoubleColon,      // ::
    Arrow,            // ->
    DoubleDot,        // ..
    Plus,             // +
    Minus,            // -
    Asterisk,         // *
    Slash,            // /
    Ampersand,        // &
    DoubleAmpersand,  // &&
    Pipe,             // |
    DoublePipe,       // ||
    Caret,            // ^
    Percent,          // %
    Equal,            // =
    DoubleEqual,      // ==
    Bang,             // !
    BangEqual,        // !=
    QuestionMark,     // ?
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SeparatorToken {
    Semicolon, // ;
    Comma,     // ,
    NewLine,
}

