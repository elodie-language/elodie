use crate::ast::TypeExpression;
use crate::core::token::TokenKind;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_type_expression(&mut self) -> crate::parser::Result<TypeExpression> {
        let type_identifier = self.consume(TokenKind::Identifier)?;
        return Ok(TypeExpression::Fundamentals(type_identifier.span.text.clone()));
    }
}