use crate::ast::parse::Error::UnsupportedToken;
use crate::ast::parse::node::{InfixNode, InfixOperator, Node};
use crate::ast::parse::Parser;
use crate::ast::token::OperatorToken;
use crate::ast::token::TokenKind::Operator;

impl Parser {
    pub(crate) fn parse_infix(&mut self, left: Node) -> crate::ast::parse::Result<InfixNode> {
        let operator = self.parse_infix_operator()?;

        let precedence = self.current_precedence()?;
        let right = self.parse_node(precedence)?;

        Ok(InfixNode {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub(crate) fn parse_infix_operator(&mut self) -> crate::ast::parse::Result<InfixOperator> {
        let token = self.advance()?;
        match &token.kind {
            Operator(operator) => match operator {
                OperatorToken::Plus => Ok(InfixOperator::Add(token)),
                OperatorToken::Minus => Ok(InfixOperator::Subtract(token)),
                OperatorToken::Asterisk => Ok(InfixOperator::Multiply(token)),
                OperatorToken::Slash => Ok(InfixOperator::Divide(token)),
                OperatorToken::Percent => Ok(InfixOperator::Modulo(token)),
                OperatorToken::DoubleEqual => Ok(InfixOperator::Equal(token)),
                OperatorToken::BangEqual => Ok(InfixOperator::NotEqual(token)),
                OperatorToken::LeftAngle => Ok(InfixOperator::LessThan(token)),
                OperatorToken::LeftAngleEqual => Ok(InfixOperator::LessThanOrEqual(token)),
                OperatorToken::RightAngle => Ok(InfixOperator::GreaterThan(token)),
                OperatorToken::RightAngleEqual => Ok(InfixOperator::GreaterThanOrEqual(token)),
                _ => Err(UnsupportedToken(token))
            }
            _ => Err(UnsupportedToken(token))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::ast::lex;
    use crate::ast::parse::node::{InfixNode, InfixOperator, LiteralNode};
    use crate::ast::parse::node::Node::{Infix, Literal};
    use crate::ast::parse::Parser;
    use crate::ast::token::{operator, OperatorToken::*, test_token, test_token_with_offset};

    macro_rules! parameterized_expression {
    ($($name:ident, $input:expr => $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                println!("Test input: {:?}", $input);
                let tokens = lex($input).unwrap();
                let mut parser = Parser::new(tokens);
                let result = parser.parse().unwrap();
                assert_eq!(result.len(), 1);

                let Infix(InfixNode{ ref left, ref operator, ref right }) = result[0] else { panic!() };

                let Literal(LiteralNode::Number { ref value, ..}) = left.deref() else {panic!()};
                assert_eq!(*value, 1.0);

                assert_eq!(*operator, $ expected);

                let Literal(LiteralNode::Number { ref value, ..}) = right.deref() else {panic!()};
                assert_eq!(*value, 2.0);
            }
        )*
    };
}

    parameterized_expression! {
        add, "1 + 2" => InfixOperator::Add(test_token_with_offset(operator(Plus), "+", 2)),
        subtract, "1 - 2" => InfixOperator::Subtract(test_token_with_offset(operator(Minus), "-", 2)),
        multiply, "1 * 2" => InfixOperator::Multiply(test_token_with_offset(operator(Asterisk), "*", 2)),
        divide, "1 / 2" => InfixOperator::Divide(test_token_with_offset(operator(Slash), "/", 2)),
        modulo, "1 % 2" => InfixOperator::Modulo(test_token_with_offset(operator(Percent), "%", 2)),
        greater_than, "1 > 2" => InfixOperator::GreaterThan(test_token_with_offset(operator(RightAngle), ">", 2)),
        greater_than_or_equal, "1 >= 2" => InfixOperator::GreaterThanOrEqual(test_token_with_offset(operator(RightAngleEqual), ">=", 2)),
        less_than, "1 < 2" => InfixOperator::LessThan(test_token_with_offset(operator(LeftAngle), "<", 2)),
        less_than_or_equal, "1 <= 2" => InfixOperator::LessThanOrEqual(test_token_with_offset(operator(LeftAngleEqual), "<=", 2)),
        equal, "1 == 2" => InfixOperator::Equal(test_token_with_offset(operator(DoubleEqual), "==", 2)),
        not_equal, "1 != 2" => InfixOperator::NotEqual(test_token_with_offset(operator(BangEqual), "!=", 2)),
    }

    macro_rules! parse_infix_operator_test {
    ($($name:ident, $input:expr => $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                println!("Test input: {:?}", $input);
                let tokens = lex($input).unwrap();
                let mut parser = Parser::new(tokens);
                let result = parser.parse_infix_operator().unwrap();
                assert_eq!(result, $expected);
            }
        )*
    };
}

    parse_infix_operator_test! {
        operator_add, "+" => InfixOperator::Add(test_token(operator(Plus), "+")),
        operator_subtract, "-" => InfixOperator::Subtract(test_token(operator(Minus), "-")),
        operator_multiply, "*" => InfixOperator::Multiply(test_token(operator(Asterisk), "*")),
        operator_divide, "/" => InfixOperator::Divide(test_token(operator(Slash), "/")),
        operator_modulo, "%" => InfixOperator::Modulo(test_token(operator(Percent), "%")),
        operator_equal, "==" => InfixOperator::Equal(test_token(operator(DoubleEqual), "==")),
        operator_not_equal, "!=" => InfixOperator::NotEqual(test_token(operator(BangEqual), "!=")),
        operator_less_than, "<" => InfixOperator::LessThan(test_token(operator(LeftAngle), "<")),
        operator_less_than_or_equal, "<=" => InfixOperator::LessThanOrEqual(test_token(operator(LeftAngleEqual), "<=")),
        operator_greater_than, ">" => InfixOperator::GreaterThan(test_token(operator(RightAngle), ">")),
        operator_greater_than_or_equal, ">=" => InfixOperator::GreaterThanOrEqual(test_token(operator(RightAngleEqual), ">=")),
    }
}