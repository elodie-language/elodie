use std::collections::HashMap;

use crate::new_ast::parse::Error::{UnexpectedEndOfFile, UnexpectedToken};
use crate::new_ast::parse::node::RootNode;
use crate::new_ast::parse::precedence::Precedence;
use crate::new_ast::token::{OperatorToken, Token, TokenKind};

pub(crate) mod precedence;
pub(crate) mod node;
mod literal;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEndOfFile,
    UnexpectedToken {
        expected: TokenKind,
        got: Token,
    },
    UnsupportedToken(Token),
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub(crate) fn parse(tokens: Vec<Token>) -> Result<RootNode> {
    Parser::new(tokens).parse()
}

struct Parser {
    tokens: Vec<Token>,
    precedence_map: HashMap<TokenKind, Precedence>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        let mut precedence_map = HashMap::new();

        precedence_map.insert(TokenKind::Operator(OperatorToken::Arrow), Precedence::Assignment);

        precedence_map.insert(TokenKind::Operator(OperatorToken::DoubleEqual), Precedence::Equality);
        precedence_map.insert(TokenKind::Operator(OperatorToken::BangEqual), Precedence::Equality);

        precedence_map.insert(TokenKind::Operator(OperatorToken::LeftAngle), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(OperatorToken::LeftAngleEqual), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(OperatorToken::RightAngle), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(OperatorToken::RightAngleEqual), Precedence::Comparison);

        precedence_map.insert(TokenKind::Operator(OperatorToken::Plus), Precedence::Term);
        precedence_map.insert(TokenKind::Operator(OperatorToken::Minus), Precedence::Term);

        precedence_map.insert(TokenKind::Operator(OperatorToken::Asterisk), Precedence::Factor);
        precedence_map.insert(TokenKind::Operator(OperatorToken::Slash), Precedence::Factor);
        precedence_map.insert(TokenKind::Operator(OperatorToken::Percent), Precedence::Factor);

        precedence_map.insert(TokenKind::Operator(OperatorToken::OpenParen), Precedence::Call);
        precedence_map.insert(TokenKind::Operator(OperatorToken::Dot), Precedence::Primary);
        precedence_map.insert(TokenKind::Operator(OperatorToken::DoubleColon), Precedence::Primary);

        let mut tokens = tokens;
        tokens.pop();
        tokens.reverse();

        Self {
            tokens,
            precedence_map,
        }
    }

    fn parse(&mut self) -> Result<RootNode> {
        let nodes = vec![];
        Ok(nodes.into())
    }

    pub(crate) fn advance(&mut self) -> Result<Token> {
        self.tokens.pop().ok_or(UnexpectedEndOfFile)
    }

    pub(crate) fn current(&self) -> Result<&Token> {
        self.tokens.last().ok_or(UnexpectedEndOfFile)
    }

    pub(crate) fn current_expect(&self, expected: TokenKind) -> Result<()> {
        let got = self.current()?;

        if got.kind == expected {
            Ok(())
        } else {
            return Err(UnexpectedToken {
                expected,
                got: got.clone(),
            });
        }
    }

    pub(crate) fn peek(&self) -> Result<&Token> {
        if self.tokens.len() < 2 {
            return Err(UnexpectedEndOfFile);
        }
        self.tokens.get(self.tokens.len() - 2).ok_or(UnexpectedEndOfFile)
    }
}

#[cfg(test)]
mod tests {
    use LiteralToken::False;
    use OperatorToken::Plus;

    use crate::new_ast::lex::lex;
    use crate::new_ast::parse::{Error, Parser};
    use crate::new_ast::token::{literal, LiteralToken, OperatorToken, separator};
    use crate::new_ast::token::LiteralToken::{Number, True};
    use crate::new_ast::token::SeparatorToken::Semicolon;

    #[test]
    fn advance_without_expression() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.advance();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn advance() {
        let tokens = lex("1 + 2").unwrap();
        let mut parser = Parser::new(tokens);

        let result = parser.advance().unwrap();
        assert!(result.is_literal(Number));
        assert_eq!(result.value(), "1");

        let result = parser.advance().unwrap();
        assert!(result.is_operator(Plus));

        let result = parser.advance().unwrap();
        assert!(result.is_literal(Number));
        assert_eq!(result.value(), "2");
    }

    #[test]
    fn current_but_eof() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.current();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn current() {
        let tokens = lex("true false").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.current().unwrap();
        assert!(result.is_literal(LiteralToken::True));
        assert_eq!(result.value(), "true");

        parser.advance().unwrap();
        let result = parser.current().unwrap();
        assert!(result.is_literal(False));
        assert_eq!(result.value(), "false");
    }

    #[test]
    fn current_expect_but_eof() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.current_expect(separator(Semicolon));
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn current_expect() {
        let tokens = lex("true false").unwrap();
        let mut parser = Parser::new(tokens);

        let result = parser.current_expect(literal(True));
        assert!(result.is_ok());

        parser.advance().unwrap();

        let result = parser.current_expect(literal(False));
        assert!(result.is_ok());
    }

    #[test]
    fn current_expect_but_different() {
        let tokens = lex("true").unwrap();
        let mut parser = Parser::new(tokens);

        let result = parser.current_expect(literal(False));
        assert!(result.is_err());

        if let Error::UnexpectedToken { expected, got } = result.err().unwrap() {
            assert_eq!(expected, literal(False));
            assert!(got.is_literal(True))
        }
    }

    #[test]
    fn peek_but_eof() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.peek();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn peek_but_nothing_to_peek() {
        let tokens = lex("true").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.peek();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }

    #[test]
    fn peek() {
        let tokens = lex("true false 1").unwrap();
        let mut parser = Parser::new(tokens);

        let result = parser.peek().unwrap();
        assert!(result.is_literal(False));
        assert_eq!(result.value(), "false");

        parser.advance().unwrap();

        let result = parser.peek().unwrap();
        assert!(result.is_literal(Number));
        assert_eq!(result.value(), "1");
    }
}