use crate::ast::{Expression, IdentifierExpression};
use crate::core::token::TokenKind;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_identifier(&mut self) -> crate::parser::Result<IdentifierExpression> {
        let identifier = self.consume(TokenKind::Identifier)?;
        Ok(IdentifierExpression(identifier.span.value.clone()))
    }
}