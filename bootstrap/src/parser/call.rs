use crate::ast::{CallArg, CallExpression, Expression};
use crate::core::token::{Operator, Separator, TokenKind};
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_arguments(&mut self) -> crate::parser::Result<Vec<CallArg>> {
        let mut result = Vec::new();

        self.consume(TokenKind::Operator(Operator::OpenParen))?;

        loop {
            {
                let current = self.current_token()?;
                if current.kind == TokenKind::Operator(Operator::CloseParen) {
                    break;
                }
            }

            result.push(CallArg {
                name: None,
                value: Box::new(self.parse_expression(Precedence::None)?),
            });

            {
                let current = self.current_token()?;
                if current.kind == TokenKind::Operator(Operator::CloseParen) {
                    break;
                }
            }

            self.consume(TokenKind::Separator(Separator::Comma))?;
        }

        Ok(result)
    }

    pub(crate) fn parse_call_expression(&mut self, callee: Expression) -> crate::parser::Result<Expression> {
        let arguments = self.parse_arguments()?;

        self.consume(TokenKind::Operator(Operator::CloseParen))?;

        Ok(Expression::Call(CallExpression {
            expression: Box::new(callee),
            arguments,
            type_args: vec![],
            lambda: None,
        }))
    }
}