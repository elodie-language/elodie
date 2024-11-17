use crate::ast::parse::Error::UnsupportedToken;
use crate::ast::parse::node::{InfixNode, InfixOperator, Node};
use crate::ast::parse::Parser;
use crate::ast::token::OperatorToken;
use crate::ast::token::TokenKind::Operator;

impl Parser {
    pub(crate) fn parse_infix(&mut self, left: Node) -> crate::ast::parse::Result<InfixNode> {
        let operator = self.parse_infix_operator()?;

        let precedence = self.current_precedence()?;

        let right = if let InfixOperator::Call(token) = &operator {
            Node::Tuple(self.parse_tuple_call(token.clone())?)
        } else if let InfixOperator::Arrow(token) = &operator {
            Node::Block(self.parse_block_inner()?)
        } else {
            self.parse_node(precedence)?
        };

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
                OperatorToken::OpenParen => Ok(InfixOperator::Call(token)),
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
                OperatorToken::Colon => Ok(InfixOperator::TypeAscription(token)),
                OperatorToken::Arrow => Ok(InfixOperator::Arrow(token)),
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
    use crate::ast::parse::{parse, Parser};
    use crate::ast::parse::node::{InfixNode, InfixOperator, LiteralNode, TupleNode, TypeFundamentalNode, TypeNode};
    use crate::ast::parse::node::Node::{Identifier, Infix, Literal, Tuple, Type};
    use crate::ast::token::{operator, OperatorToken::*, test_token, test_token_with_offset};

    #[test]
    fn identifier_with_type() {
        let tokens = lex("u: Bool").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Infix(InfixNode { left, operator, right }) = &result[0] else { panic!() };
        let InfixOperator::TypeAscription(_) = operator else { panic!() };

        let Identifier(identifier) = left.as_ref() else { panic!() };
        assert_eq!(identifier.identifier(), "u");

        let Type(type_node) = right.as_ref() else { panic!() };
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = type_node else { panic!() };
    }

    macro_rules! parse_infix {
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

                let Literal(LiteralNode::Number(node)) = left.deref() else {panic!()};
                assert_eq!(node.value().unwrap(), 1.0);

                assert_eq!(*operator, $ expected);

                let Literal(LiteralNode::Number(node)) = right.deref() else {panic!()};
                assert_eq!(node.value().unwrap(), 2.0);
            }
        )*
    };
}

    parse_infix! {
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

    #[test]
    fn call_without_arguments() {
        let tokens = lex("test()").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Infix(InfixNode { left, operator, right }) = &result[0] else { panic!() };
        let Identifier(node) = left.deref() else { panic!() };
        assert_eq!(node.identifier(), "test");

        let InfixOperator::Call(_) = operator else { panic!() };

        let Tuple(TupleNode { nodes, .. }) = right.deref() else { panic!() };
        assert_eq!(*nodes, vec![]);
    }

    #[test]
    fn call_with_argument() {
        let tokens = lex("test('elodie')").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Infix(InfixNode { left, operator, right }) = &result[0] else { panic!() };
        let Identifier(node) = left.deref() else { panic!() };
        assert_eq!(node.identifier(), "test");

        let InfixOperator::Call(_) = operator else { panic!() };

        let Tuple(TupleNode { nodes, .. }) = right.deref() else { panic!() };
        assert_eq!(nodes.len(), 1);

        let Some(Literal(LiteralNode::String(arg_1))) = &nodes.first() else { panic!() };
        assert_eq!(arg_1.value(), "elodie");
    }
}