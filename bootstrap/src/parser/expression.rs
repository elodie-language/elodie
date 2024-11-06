use std::str::FromStr;

use crate::ast;
use crate::ast::{BinaryOperation, Expression, UnaryOperation, UnaryOperator};
use crate::core::token::{Literal, Operator, TokenKind};
use crate::parser::Error::UnexpectedToken;
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self, precedence: Precedence) -> crate::parser::Result<Expression> {
        self.skip_whitespace()?;

        let mut left = self.parse_prefix_expression()?;

        while precedence < self.current_precedence()? {
            left = self.parse_infix_expression(left)?;
        }
        Ok(left)
    }

    pub(crate) fn parse_prefix_expression(&mut self) -> crate::parser::Result<Expression> {
        self.skip_whitespace()?;

        let token = self.advance()?;

        let expression = match &token.kind {
            TokenKind::Identifier => {
                let identifier = token.span.text.clone();
                Expression::Identifier(identifier)
            }
            TokenKind::Keyword(_) => return Err(UnexpectedToken(token.clone())),
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
                    Literal::True => unimplemented!(),
                    Literal::False => unimplemented!()
                }
            }
            TokenKind::Operator(operator) => {
                match operator {
                    Operator::OpenParen => {
                        let expr = self.parse_expression(Precedence::None)?;
                        self.consume(TokenKind::Operator(Operator::CloseParen))?;
                        expr
                    }
                    Operator::CloseParen => unimplemented!(),
                    Operator::OpenCurly => unimplemented!(),
                    Operator::CloseCurly => unimplemented!(),
                    Operator::OpenBracket => unimplemented!(),
                    Operator::CloseBracket => unimplemented!(),
                    Operator::LeftAngle => unimplemented!(),
                    Operator::DoubleLeftAngle => unimplemented!(),
                    Operator::LeftAngleEqual => unimplemented!(),
                    Operator::RightAngle => unimplemented!(),
                    Operator::DoubleRightAngle => unimplemented!(),
                    Operator::RightAngleEqual => unimplemented!(),
                    Operator::Dot => unimplemented!(),
                    Operator::Colon => unimplemented!(),
                    Operator::DoubleColon => unimplemented!(),
                    Operator::Arrow => unimplemented!(),
                    Operator::DoubleDot => unimplemented!(),
                    Operator::Plus => unimplemented!(),
                    Operator::Minus => {
                        let right = self.parse_expression(Precedence::Unary)?;
                        Expression::UnaryOp(UnaryOperation {
                            op: UnaryOperator::Minus,
                            expr: Box::new(right),
                        })
                    }
                    Operator::Asterisk => unimplemented!(),
                    Operator::Slash => unimplemented!(),
                    Operator::Ampersand => unimplemented!(),
                    Operator::DoubleAmpersand => unimplemented!(),
                    Operator::Pipe => unimplemented!(),
                    Operator::DoublePipe => unimplemented!(),
                    Operator::Caret => unimplemented!(),
                    Operator::Percent => unimplemented!(),
                    Operator::Equals => unimplemented!(),
                    Operator::DoubleEqual => unimplemented!(),
                    Operator::Bang => unimplemented!(),
                    Operator::BangEqual => unimplemented!(),
                    Operator::QuestionMark => unimplemented!()
                }
            }
            TokenKind::Separator(_) => return Err(UnexpectedToken(token.clone())),
            TokenKind::Comment => return Err(UnexpectedToken(token.clone())),
            TokenKind::EOF => return Err(UnexpectedToken(token.clone()))
        };

        Ok(expression)
    }

    pub(crate) fn parse_infix_expression(&mut self, left: Expression) -> crate::parser::Result<Expression> {
        let current = self.current_token()?;
        if current.kind == TokenKind::Operator(Operator::OpenParen) {
            return self.parse_call_expression(left);
        }

        if current.kind == TokenKind::Operator(Operator::Dot) {
            let previous = self.previous()?;
            assert_eq!(previous.kind, TokenKind::Identifier);
            let _ = self.consume(TokenKind::Operator(Operator::Dot));
            return self.parse_property_access(left);
        }

        let operator = self.parse_binary_operator()?;

        let precedence = self.current_precedence()?;
        let right = self.parse_expression(precedence)?;

        Ok(Expression::BinaryOperation(BinaryOperation {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinaryOperator, Expression, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    macro_rules! parameterized_parse_infix_expression {
    ($($name:ident, $input:expr => $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let tokens = Lexer::lex($input).unwrap();
                let result = Parser::parse(&tokens).unwrap();
                let stmt = result.block.statements.first().unwrap();

                if let Statement::Expression(Expression::BinaryOperation(got)) = stmt {
                     assert_eq!(
                        &got.operator, &$expected,
                        "Failed on input '{}', expected {:?} but got {:?}",
                        $input, $expected, got.operator
                     );
                } else{
                    panic!("Expected binary expression");
                }
            }
        )*
    };
}

    parameterized_parse_infix_expression! {
        parse_infix_add, "5 + 5" => BinaryOperator::Add,
        parse_infix_subtract, "5 - 5" => BinaryOperator::Subtract,
        parse_infix_multiply, "5 * 5" => BinaryOperator::Multiply,
        parse_infix_divide, "5 / 5" => BinaryOperator::Divide,
        parse_infix_modulo, "5 % 5" => BinaryOperator::Modulo,
        parse_infix_greater_than, "5 > 5" => BinaryOperator::GreaterThan,
        parse_infix_greater_than_or_equal, "5 >= 5" => BinaryOperator::GreaterThanOrEqual,
        parse_infix_less_than, "5 < 5" => BinaryOperator::LessThan,
        parse_infix_less_than_or_equal, "5 <= 5" => BinaryOperator::LessThanOrEqual,
        parse_infix_equal, "5 == 5" => BinaryOperator::Equal,
        parse_infix_not_equal, "5 != 5" => BinaryOperator::NotEqual,
    }
}