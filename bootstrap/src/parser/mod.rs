use std::collections::HashMap;

use crate::ast::{Block, ElodieFile};
use crate::core::token::{Operator, Separator, Token, TokenKind};
use crate::parser::Error::UnexpectedEndOfFile;
use crate::parser::precedence::Precedence;

mod expression;
mod statement;
mod precedence;
mod operator;
mod call;
mod access;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfFile,
    UnexpectedToken(Token),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    precedence_map: HashMap<TokenKind, Precedence>,
}

impl<'a> Parser<'a> {
    pub fn parse(tokens: &'a [Token]) -> Result<ElodieFile> {
        let mut parser = Parser::new(tokens);
        parser.parse_file()
    }

    pub(crate) fn new(tokens: &'a [Token]) -> Self {
        let mut precedence_map = HashMap::new();

        precedence_map.insert(TokenKind::Operator(Operator::Plus), Precedence::Term);
        precedence_map.insert(TokenKind::Operator(Operator::Minus), Precedence::Term);
        precedence_map.insert(TokenKind::Operator(Operator::Asterisk), Precedence::Factor);
        precedence_map.insert(TokenKind::Operator(Operator::Slash), Precedence::Factor);
        precedence_map.insert(TokenKind::Operator(Operator::OpenParen), Precedence::Call);
        precedence_map.insert(TokenKind::Operator(Operator::Dot), Precedence::Primary);

        Self {
            tokens,
            current: 0,
            precedence_map,
        }
    }


    pub(crate) fn parse_file(&mut self) -> Result<ElodieFile> {
        let mut result = ElodieFile {
            imports: vec![],
            declarations: vec![],
            block: Block {
                statements: vec![]
            },
        };

        loop {
            let current_kind = self.current_token_kind()?;
            if current_kind == &TokenKind::EOF {
                break;
            }

            result.block.statements.push(self.parse_statement()?);
        }

        Ok(result)
    }

    pub(crate) fn advance(&mut self) -> Result<&Token> {
        let result = &self.tokens[self.current];
        self.current += 1;
        self.skip_whitespace()?;
        Ok(result)
    }

    pub(crate) fn skip_whitespace(&mut self) -> Result<()> {
        loop {
            let token = self.current_token()?;
            if token.kind == TokenKind::Separator(Separator::Whitespace) || token.kind == TokenKind::Comment {
                self.advance()?;
            } else {
                return Ok(());
            }
        }
    }

    pub(crate) fn previous(&self) -> Result<&Token> {
        Ok(&self.tokens[self.current - 1])
    }

    pub(crate) fn consume(&mut self, expected: TokenKind) -> Result<()> {
        self.skip_whitespace()?;
        let current = self.current_token_kind()?;
        if current == &TokenKind::EOF { return Ok(()); }

        if current == &expected {
            self.advance()?;
            Ok(())
        } else {
            panic!("Expected token {:?} but was {:?}", expected, current);
        }
    }

    pub(crate) fn current_token(&self) -> Result<&Token> {
        if self.current < self.tokens.len() {
            Ok(&self.tokens[self.current])
        } else {
            Err(UnexpectedEndOfFile)
        }
    }

    pub(crate) fn peek_token(&self) -> Result<&Token> {
        if self.current + 1 < self.tokens.len() {
            Ok(&self.tokens[self.current + 1])
        } else {
            Err(UnexpectedEndOfFile)
        }
    }

    pub(crate) fn current_token_kind(&self) -> Result<&TokenKind> {
        Ok(&self.current_token()?.kind)
    }
    pub(crate) fn peek_token_kind(&self) -> Result<&TokenKind> {
        Ok(&self.peek_token()?.kind)
    }

    pub(crate) fn current_precedence(&self) -> Result<Precedence> {
        let current = self.current_token_kind()?;
        let precedence = self.precedence_map.get(current).cloned();
        Ok(precedence.unwrap_or(Precedence::None))
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{Block, Expression};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn parse_empty_string() {
        let tokens = Lexer::lex("").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.imports, vec![]);
        assert_eq!(result.declarations, vec![]);
        assert_eq!(result.block, Block { statements: vec![] })
    }
}