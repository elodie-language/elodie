use crate::ast::Expression;
use crate::core::token::TokenKind;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_identifier(&mut self) -> crate::parser::Result<Expression> {
        let identifier = self.consume(TokenKind::Identifier)?;
        Ok(Expression::Identifier(identifier.span.text.clone()))
    }
}