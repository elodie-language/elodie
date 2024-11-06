use crate::ast::BinaryOperator;
use crate::core::token::{Operator, TokenKind};
use crate::parser::Error::UnexpectedToken;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_binary_operator(&mut self) -> crate::parser::Result<BinaryOperator> {
        let token = self.advance()?;
        return match &token.kind {
            TokenKind::Operator(operator) => {
                match operator {
                    Operator::LeftAngle => Ok(BinaryOperator::LessThan),
                    Operator::LeftAngleEqual => Ok(BinaryOperator::LessThanOrEqual),
                    Operator::RightAngle => Ok(BinaryOperator::GreaterThan),
                    Operator::RightAngleEqual => Ok(BinaryOperator::GreaterThanOrEqual),
                    Operator::Plus => Ok(BinaryOperator::Add),
                    Operator::Minus => Ok(BinaryOperator::Subtract),
                    Operator::Asterisk => Ok(BinaryOperator::Multiply),
                    Operator::Slash => Ok(BinaryOperator::Divide),
                    Operator::Percent => Ok(BinaryOperator::Modulo),
                    Operator::DoubleEqual => Ok(BinaryOperator::Equal),
                    Operator::BangEqual => Ok(BinaryOperator::NotEqual),
                    _ => Err(UnexpectedToken(token.clone()))
                }
            }
            _ => Err(UnexpectedToken(token.clone()))
        };
    }
}