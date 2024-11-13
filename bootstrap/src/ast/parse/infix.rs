use crate::ast::parse::Error::UnsupportedToken;
use crate::ast::parse::node::{InfixOperator, Node};
use crate::ast::parse::Parser;
use crate::ast::token::OperatorToken;
use crate::ast::token::TokenKind::Operator;

impl Parser {

    pub(crate) fn parse_infix(&mut self, left: Node) -> crate::ast::parse::Result<Node> {
        todo!()
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
    use crate::ast::lex;
    use crate::ast::parse::node::InfixOperator;
    use crate::ast::parse::Parser;
    use crate::ast::token::{operator, OperatorToken::*, test_token};

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