use std::collections::HashMap;

use crate::ast::{Block, ElodieFile};
use crate::core::token::{Operator, Token, TokenKind};
use crate::parser::Error::{UnexpectedEndOfFile, UnexpectedToken};
use crate::parser::precedence::Precedence;

mod expression;
mod statement;
mod precedence;
mod operator;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfFile,
    UnexpectedToken {
        expected: TokenKind,
        got: Token,
    },
    UnsupportedToken(Token),
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

        Self {
            tokens,
            current: 0,
            precedence_map,
        }
    }


    pub(crate) fn parse_file(&mut self) -> Result<ElodieFile> {
        let mut result = ElodieFile {
            imports: vec![],
            // exports: vec![],
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

        // FIXME handle exports
        // for stmt in &result.block.statements {
        //     if let Statement::Expression(Expression::FunctionDeclaration(expression)) = stmt {
        //         if expression.name.is_none() {
        //             continue;
        //         }
        //
        //         result.exports.push(Export::Function(
        //             FunctionExport {
        //                 name: expression.name.clone().unwrap(),
        //                 parameters: expression.parameters.clone(),
        //                 return_type: expression.return_type.clone(),
        //             }
        //         ))
        //     }
        // }

        Ok(result)
    }

    pub(crate) fn advance(&mut self) -> Result<&Token> {
        let result = &self.tokens[self.current];
        self.current += 1;
        Ok(result)
    }


    pub(crate) fn previous(&self) -> Result<&Token> {
        Ok(&self.tokens[self.current - 1])
    }

    pub(crate) fn consume(&mut self, expected: TokenKind) -> Result<&Token> {
        let current = self.current_token_kind()?;
        if current == &TokenKind::EOF { return Err(UnexpectedEndOfFile); }
        self.current_expect(expected)?;

        self.advance()
    }

    pub(crate) fn consume_if(&mut self, expected: TokenKind) -> Result<Option<&Token>> {
        let current = self.current_token_kind()?;
        if current == &TokenKind::EOF { return Err(UnexpectedEndOfFile); }

        if current == &expected {
            Ok(Some(self.advance()?))
        } else {
            Ok(None)
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

    pub(crate) fn current_expect(&self, expected: TokenKind) -> Result<()> {
        let got = self.current_token()?;

        if got.kind == expected {
            Ok(())
        } else {
            return Err(UnexpectedToken {
                expected,
                got: got.clone(),
            });
        }
    }

    pub(crate) fn previous_expect(&self, expected: TokenKind) -> Result<()> {
        let got = self.previous()?;

        if got.kind == expected {
            Ok(())
        } else {
            return Err(UnexpectedToken {
                expected,
                got: got.clone(),
            });
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Block;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn parse_empty_string() {
        let tokens = Lexer::lex("").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.imports, vec![]);
        assert_eq!(result.block, Block { statements: vec![] })
    }
}