use std::cmp::PartialOrd;
use std::collections::HashMap;

use crate::common::context::Context;
use crate::frontend::lex::token::SeparatorToken::NewLine;
use crate::frontend::lex::token::TokenKind::{Keyword, Literal, Operator, Separator};
use crate::frontend::lex::token::{
    KeywordToken, LiteralToken, OperatorToken, SeparatorToken, Token, TokenKind,
};
pub use crate::frontend::parse::node::*;
use crate::frontend::parse::precedence::Precedence;
use crate::frontend::parse::Error::UnexpectedEndOfFile;

mod block;
mod define;
mod external;
mod from;
mod function;
mod identifier;
mod r#if;
mod infix;
mod literal;
mod r#loop;
mod modifier;
mod node;
mod package;
pub(crate) mod precedence;
mod primary;
mod string;
mod tuple;
mod r#type;
mod type_declaration;
mod variable;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidIdentifier(Token),
    InvalidType(Token),
    UnexpectedEndOfFile,
    UnexpectedToken { expected: TokenKind, got: Token },
    UnsupportedNumber(String),
    UnsupportedToken(Token),
    UnknownType(Token),
}

impl Error {
    pub(crate) fn eof() -> Self {
        Self::UnexpectedEndOfFile
    }
    pub(crate) fn unexpected(expected: TokenKind, got: Token) -> Self {
        Self::UnexpectedToken { expected, got }
    }
    pub(crate) fn unsupported(token: Token) -> Self {
        Self::UnsupportedToken(token)
    }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub(crate) fn parse(ctx: &mut Context, tokens: Vec<Token>) -> Result<Vec<Node>> {
    Parser::new(ctx, tokens).parse()
}

struct Parser<'a> {
    ctx: &'a mut Context,
    tokens: Vec<Token>,
    precedence_map: HashMap<TokenKind, Precedence>,
}

impl<'a> Parser<'a> {
    fn new(ctx: &'a mut Context, tokens: Vec<Token>) -> Self {
        let mut precedence_map = HashMap::new();
        precedence_map.insert(Operator(OperatorToken::Equal), Precedence::Assignment);

        precedence_map.insert(Operator(OperatorToken::DoubleEqual), Precedence::Comparison);
        precedence_map.insert(Operator(OperatorToken::BangEqual), Precedence::Comparison);

        precedence_map.insert(Operator(OperatorToken::LeftAngle), Precedence::Comparison);
        precedence_map.insert(
            Operator(OperatorToken::LeftAngleEqual),
            Precedence::Comparison,
        );
        precedence_map.insert(Operator(OperatorToken::RightAngle), Precedence::Comparison);
        precedence_map.insert(
            Operator(OperatorToken::RightAngleEqual),
            Precedence::Comparison,
        );

        precedence_map.insert(Operator(OperatorToken::Plus), Precedence::Term);
        precedence_map.insert(Operator(OperatorToken::Minus), Precedence::Term);

        precedence_map.insert(Operator(OperatorToken::Asterisk), Precedence::Factor);
        precedence_map.insert(Operator(OperatorToken::Slash), Precedence::Factor);
        precedence_map.insert(Operator(OperatorToken::Percent), Precedence::Factor);

        precedence_map.insert(Operator(OperatorToken::OpenParen), Precedence::Call);
        precedence_map.insert(Operator(OperatorToken::OpenCurly), Precedence::LambdaCall);

        precedence_map.insert(Operator(OperatorToken::Dot), Precedence::Primary);
        precedence_map.insert(Operator(OperatorToken::DoubleColon), Precedence::Primary);

        precedence_map.insert(Operator(OperatorToken::Arrow), Precedence::Primary);
        precedence_map.insert(Operator(OperatorToken::Colon), Precedence::Primary);

        let mut tokens = tokens;
        tokens.pop();
        tokens.reverse();

        Self {
            ctx,
            tokens,
            precedence_map,
        }
    }

    fn parse(&mut self) -> Result<Vec<Node>> {
        let mut nodes = vec![];
        loop {
            if self.is_eof() {
                break;
            }
            nodes.push(self.parse_node(Precedence::None)?);
            if !self.is_eof() {
                self.consume_if(TokenKind::Separator(NewLine))?;
            }
        }
        Ok(nodes)
    }

    pub(crate) fn parse_node(&mut self, precedence: Precedence) -> Result<Node> {
        let mut left = self.parse_primary()?;

        while !self.is_eof() && precedence < self.current_precedence()? {
            left = Node::Infix(self.parse_infix(left)?);
        }
        Ok(left)
    }

    pub(crate) fn advance(&mut self) -> Result<Token> {
        self.tokens.pop().ok_or(Error::eof())
    }

    pub(crate) fn consume(&mut self, expected: TokenKind) -> Result<Token> {
        self.current_expect(expected)?;
        self.advance()
    }

    pub(crate) fn consume_if(&mut self, expected: TokenKind) -> Result<Option<Token>> {
        if self.is_eof() || self.current()?.kind != expected {
            return Ok(None);
        }

        Ok(Some(self.consume(expected)?))
    }

    pub(crate) fn consume_while(&mut self, expected: TokenKind) -> Result<()> {
        loop {
            if self.is_eof() || self.current()?.kind != expected {
                return Ok(());
            }
            self.advance()?;
        }
    }

    pub(crate) fn consume_literal(&mut self, expected: LiteralToken) -> Result<Token> {
        self.current_expect_literal(expected)?;
        self.advance()
    }

    pub(crate) fn consume_operator(&mut self, expected: OperatorToken) -> Result<Token> {
        self.current_expect_operator(expected)?;
        self.advance()
    }

    pub(crate) fn consume_keyword(&mut self, expected: KeywordToken) -> Result<Token> {
        self.current_expect_keyword(expected)?;
        self.advance()
    }

    pub(crate) fn consume_separator(&mut self, expected: SeparatorToken) -> Result<Token> {
        self.current_expect_separator(expected)?;
        self.advance()
    }

    pub(crate) fn current(&self) -> Result<&Token> {
        self.tokens.last().ok_or(UnexpectedEndOfFile)
    }

    pub(crate) fn current_expect(&self, expected: TokenKind) -> Result<()> {
        let got = self.current()?;
        if got.kind == expected {
            Ok(())
        } else {
            return Err(Error::unexpected(expected, got.clone()));
        }
    }

    pub(crate) fn current_expect_literal(&self, literal: LiteralToken) -> Result<()> {
        self.current_expect(Literal(literal))
    }

    pub(crate) fn current_expect_operator(&self, operator: OperatorToken) -> Result<()> {
        self.current_expect(Operator(operator))
    }

    pub(crate) fn current_expect_keyword(&self, keyword: KeywordToken) -> Result<()> {
        self.current_expect(Keyword(keyword))
    }

    pub(crate) fn current_expect_separator(&self, separator: SeparatorToken) -> Result<()> {
        self.current_expect(Separator(separator))
    }

    pub(crate) fn current_precedence(&self) -> Result<Precedence> {
        if self.is_eof() {
            return Ok(Precedence::None);
        };

        let current = self.current()?;
        let precedence = self.precedence_map.get(&current.kind).cloned();
        Ok(precedence.unwrap_or(Precedence::None))
    }

    pub(crate) fn peek(&self) -> Result<&Token> {
        if self.tokens.len() < 2 {
            return Err(Error::eof());
        }
        self.tokens.get(self.tokens.len() - 2).ok_or(Error::eof())
    }

    pub(crate) fn peek_expect(&self, expected: TokenKind) -> Result<()> {
        let got = self.peek()?;
        if got.kind == expected {
            Ok(())
        } else {
            return Err(Error::unexpected(expected, got.clone()));
        }
    }

    fn is_eof(&self) -> bool {
        self.tokens.is_empty()
    }

    pub(crate) fn skip_new_line(&mut self) -> Result<()> {
        self.consume_while(Separator(NewLine))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use LiteralToken::False;
    use OperatorToken::Plus;

    use crate::common::context::Context;
    use crate::frontend::lex::lex;
    use crate::frontend::lex::token::LiteralToken::{Number, True};
    use crate::frontend::lex::token::SeparatorToken::Semicolon;
    use crate::frontend::lex::token::{literal, separator, LiteralToken, OperatorToken};
    use crate::frontend::parse::precedence::Precedence;
    use crate::frontend::parse::precedence::Precedence::Term;
    use crate::frontend::parse::{Error, Parser};

    #[test]
    fn advance_but_eof() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.advance();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn advance() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "1 + 2").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);

        let token_one = parser.advance().unwrap();
        let token_two = parser.advance().unwrap();
        let token_three = parser.advance().unwrap();

        assert_eq!(ctx.get_str(token_one.value()), "1");
        assert!(token_one.is_literal(Number));

        assert!(token_two.is_operator(Plus));

        assert!(token_three.is_literal(Number));
        assert_eq!(ctx.get_str(token_three.value()), "2");
    }

    #[test]
    fn consume_but_eof() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.consume(literal(True));
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn consume_but_unexpected_token() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "false").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.consume(literal(True));
        assert!(result.is_err());

        if let Error::UnexpectedToken { expected, got } = result.err().unwrap() {
            assert_eq!(expected, literal(True));
            assert!(got.is_literal(False));
        }
    }

    #[test]
    fn consume() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "true 99").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.consume(literal(True)).unwrap();
        assert!(result.is_literal(True));

        let result = parser.consume(literal(Number)).unwrap();
        assert!(result.is_literal(Number));
    }

    #[test]
    fn consume_if_but_eof() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.consume_if(literal(True));
        assert_eq!(result, Ok(None))
    }

    #[test]
    fn consume_if_but_unexpected_token() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "false").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.consume_if(literal(True));
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn consume_if() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "true 99").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.consume_if(literal(True)).unwrap().unwrap();
        assert!(result.is_literal(True));

        let result = parser.consume_if(literal(Number)).unwrap().unwrap();
        assert!(result.is_literal(Number));
    }

    #[test]
    fn current_but_eof() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.current();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn current() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "true false").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);

        let token_one = parser.current().unwrap().clone();
        parser.advance().unwrap();
        let token_two = parser.current().unwrap().clone();

        assert!(token_one.is_literal(True));
        assert_eq!(ctx.get_str(token_one.value()), "true");

        assert!(token_two.is_literal(False));
        assert_eq!(ctx.get_str(token_two.value()), "false");
    }

    #[test]
    fn current_expect_but_eof() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.current_expect(separator(Semicolon));
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn current_expect() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "true false").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);

        let result = parser.current_expect(literal(True));
        assert!(result.is_ok());

        parser.advance().unwrap();

        let result = parser.current_expect(literal(False));
        assert!(result.is_ok());
    }

    #[test]
    fn current_expect_but_different() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "true").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);

        let result = parser.current_expect(literal(False));
        assert!(result.is_err());

        if let Error::UnexpectedToken { expected, got } = result.err().unwrap() {
            assert_eq!(expected, literal(False));
            assert!(got.is_literal(True))
        }
    }

    #[test]
    fn current_precedence_but_eof() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.current_precedence();
        assert_eq!(result, Ok(Precedence::None))
    }

    #[test]
    fn current_precedence() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "+").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.current_precedence();
        assert_eq!(result, Ok(Term))
    }

    #[test]
    fn peek_but_eof() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.peek();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn peek_but_nothing_to_peek() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "true").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.peek();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn peek() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "true false 1").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);

        let token_one = parser.peek().unwrap().clone();
        parser.advance().unwrap();
        let token_two = parser.peek().unwrap().clone();

        assert!(token_one.is_literal(False));
        assert_eq!(ctx.get_str(token_one.value()), "false");

        assert!(token_two.is_literal(Number));
        assert_eq!(ctx.get_str(token_two.value()), "1");
    }

    #[test]
    fn peek_expect_but_eof() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.peek_expect(separator(Semicolon));
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn peek_expect_but_nothing_to_peek() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "true").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);

        let result = parser.peek_expect(separator(Semicolon));
        assert_eq!(result, Err(Error::UnexpectedEndOfFile));
    }

    #[test]
    fn peek_expect() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "true false 99").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);

        let result = parser.peek_expect(literal(False));
        assert!(result.is_ok());

        parser.advance().unwrap();

        let result = parser.peek_expect(literal(Number));
        assert!(result.is_ok());
    }

    #[test]
    fn peek_expect_but_different() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "true 99").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);

        let result = parser.peek_expect(literal(False));
        assert!(result.is_err());

        if let Error::UnexpectedToken { expected, got } = result.err().unwrap() {
            assert_eq!(expected, literal(False));
            assert!(got.is_literal(Number))
        }
    }
}
