use crate::ast::{Expression, IdentifierExpression, PropertyAccessExpression};
use crate::core::token::TokenKind;
use crate::parser::Error::UnexpectedToken;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_property_access(&mut self, object: Expression) -> crate::parser::Result<Expression> {
        let next = self.advance()?;
        if next.kind != TokenKind::Identifier {
            return Err(UnexpectedToken(next.clone()));
        }

        Ok(Expression::PropertyAccess(PropertyAccessExpression {
            lhs: Some(Box::new(object)),
            rhs: Box::new(Expression::Identifier(IdentifierExpression(next.span.text.clone()))),
        }))
    }
}