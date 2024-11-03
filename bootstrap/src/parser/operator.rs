use crate::core::ast::BinaryOperator;
use crate::core::token::{Operator, TokenKind};
use crate::parser::Error::UnexpectedToken;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_binary_operator(&mut self) -> crate::parser::Result<BinaryOperator> {
        let token = self.advance()?;
        return match &token.kind {
            TokenKind::Identifier => Err(UnexpectedToken(token.clone())),
            TokenKind::Keyword(_) => Err(UnexpectedToken(token.clone())),
            TokenKind::Literal(_) => Err(UnexpectedToken(token.clone())),
            TokenKind::Operator(operator) => {
                match operator {
                    Operator::OpenParen => Err(UnexpectedToken(token.clone())),
                    Operator::CloseParen => Err(UnexpectedToken(token.clone())),
                    Operator::OpenCurly => Err(UnexpectedToken(token.clone())),
                    Operator::CloseCurly => Err(UnexpectedToken(token.clone())),
                    Operator::OpenBracket => Err(UnexpectedToken(token.clone())),
                    Operator::CloseBracket => Err(UnexpectedToken(token.clone())),
                    Operator::LeftAngle => Err(UnexpectedToken(token.clone())),
                    Operator::DoubleLeftAngle => Err(UnexpectedToken(token.clone())),
                    Operator::LeftAngleEquals => Err(UnexpectedToken(token.clone())),
                    Operator::RightAngle => Err(UnexpectedToken(token.clone())),
                    Operator::DoubleRightAngle => Err(UnexpectedToken(token.clone())),
                    Operator::RightAngleEquals => Err(UnexpectedToken(token.clone())),
                    Operator::Dot => Err(UnexpectedToken(token.clone())),
                    Operator::DoubleDot => Err(UnexpectedToken(token.clone())),
                    Operator::Colon => Err(UnexpectedToken(token.clone())),
                    Operator::DoubleColon => Err(UnexpectedToken(token.clone())),
                    Operator::Arrow => Err(UnexpectedToken(token.clone())),
                    Operator::Plus => Ok(BinaryOperator::Add),
                    Operator::Minus => Ok(BinaryOperator::Subtract),
                    Operator::Asterisk => Ok(BinaryOperator::Multiply),
                    Operator::Slash => Err(UnexpectedToken(token.clone())),
                    Operator::Ampersand => Err(UnexpectedToken(token.clone())),
                    Operator::DoubleAmpersand => Err(UnexpectedToken(token.clone())),
                    Operator::Pipe => Err(UnexpectedToken(token.clone())),
                    Operator::DoublePipe => Err(UnexpectedToken(token.clone())),
                    Operator::Caret => Err(UnexpectedToken(token.clone())),
                    Operator::Percent => Err(UnexpectedToken(token.clone())),
                    Operator::Equals => Err(UnexpectedToken(token.clone())),
                    Operator::DoubleEquals => Err(UnexpectedToken(token.clone())),
                    Operator::Bang => Err(UnexpectedToken(token.clone())),
                    Operator::BangEquals => Err(UnexpectedToken(token.clone())),
                    Operator::QuestionMark => Err(UnexpectedToken(token.clone())),
                }
            }
            TokenKind::Separator(_) => Err(UnexpectedToken(token.clone())),
            TokenKind::Comment => Err(UnexpectedToken(token.clone())),
            TokenKind::EOF => Err(UnexpectedToken(token.clone())),
        };
    }
}