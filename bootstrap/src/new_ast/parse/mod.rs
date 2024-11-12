use std::collections::HashMap;

use crate::new_ast::parse::Error::UnexpectedEndOfFile;
use crate::new_ast::parse::node::RootNode;
use crate::new_ast::parse::precedence::Precedence;
use crate::new_ast::token::{Operator, Token, TokenKind};

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
    current: usize,
    precedence_map: HashMap<TokenKind, Precedence>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        let mut precedence_map = HashMap::new();

        precedence_map.insert(TokenKind::Operator(Operator::Arrow), Precedence::Assignment);

        precedence_map.insert(TokenKind::Operator(Operator::DoubleEqual), Precedence::Equality);
        precedence_map.insert(TokenKind::Operator(Operator::BangEqual), Precedence::Equality);

        precedence_map.insert(TokenKind::Operator(Operator::LeftAngle), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(Operator::LeftAngleEqual), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(Operator::RightAngle), Precedence::Comparison);
        precedence_map.insert(TokenKind::Operator(Operator::RightAngleEqual), Precedence::Comparison);

        precedence_map.insert(TokenKind::Operator(Operator::Plus), Precedence::Term);
        precedence_map.insert(TokenKind::Operator(Operator::Minus), Precedence::Term);

        precedence_map.insert(TokenKind::Operator(Operator::Asterisk), Precedence::Factor);
        precedence_map.insert(TokenKind::Operator(Operator::Slash), Precedence::Factor);
        precedence_map.insert(TokenKind::Operator(Operator::Percent), Precedence::Factor);

        precedence_map.insert(TokenKind::Operator(Operator::OpenParen), Precedence::Call);
        precedence_map.insert(TokenKind::Operator(Operator::Dot), Precedence::Primary);
        precedence_map.insert(TokenKind::Operator(Operator::DoubleColon), Precedence::Primary);

        Self {
            tokens,
            current: 0,
            precedence_map,
        }
    }

    fn parse(&mut self) -> Result<RootNode> {
        let nodes = vec![];
        Ok(nodes.into())
    }

    pub(crate) fn advance(&mut self) -> Result<Token> {
        let result = self.tokens.pop().ok_or(Error::UnexpectedEndOfFile)?;
        if result.kind == TokenKind::EOF {
            return Err(UnexpectedEndOfFile);
        }
        self.current += 1;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::new_ast::lex::lex;
    use crate::new_ast::parse::{Error, Parser};

    #[test]
    fn advance_without_expression() {
        let tokens = lex("").unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.advance();
        assert_eq!(result, Err(Error::UnexpectedEndOfFile))
    }
}