use std::str::FromStr;

use crate::ast;
use crate::ast::{Expression, IdentifierExpression, UnaryOperation, UnaryOperator};
use crate::core::token::{Keyword, Literal, Operator, TokenKind};
use crate::parser::Error::UnexpectedToken;
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_prefix_expression(&mut self) -> crate::parser::Result<Expression> {
        self.skip_whitespace()?;

        let token = self.advance()?;

        let expression = match &token.kind {
            TokenKind::Identifier => {
                let identifier = token.span.text.clone();
                Expression::Identifier(IdentifierExpression(identifier))
            }
            TokenKind::Literal(literal) => {
                match literal {
                    Literal::Number => {
                        let value = f64::from_str(&token.span.text).unwrap();
                        Expression::Literal(ast::Literal::Number(value))
                    }
                    Literal::String => {
                        let value = token.span.text.clone();
                        Expression::Literal(ast::Literal::String(value))
                    }
                    Literal::True => Expression::Literal(ast::Literal::Boolean(true)),
                    Literal::False => Expression::Literal(ast::Literal::Boolean(false))
                }
            }
            TokenKind::Operator(operator) => {
                match operator {
                    Operator::OpenParen => {
                        let expr = self.parse_expression(Precedence::None)?;
                        self.consume(TokenKind::Operator(Operator::CloseParen))?;
                        expr
                    }
                    Operator::Minus => {
                        let right = self.parse_expression(Precedence::Unary)?;
                        Expression::UnaryOp(UnaryOperation {
                            op: UnaryOperator::Minus,
                            expr: Box::new(right),
                        })
                    }
                    _ => unimplemented!(),
                }
            }
            TokenKind::Keyword(keyword) => {
                match keyword {
                    Keyword::Let => self.parse_let_expression()?,
                    _ => return Err(UnexpectedToken(token.clone()))
                }
            }
            _ => return Err(UnexpectedToken(token.clone()))
        };

        Ok(expression)
    }
}