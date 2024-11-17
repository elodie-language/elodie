use crate::ast::parse::{Error, Parser};
use crate::ast::parse::Error::UnsupportedToken;
use crate::ast::parse::node::{Node, PrefixNode, PrefixOperator};
use crate::ast::parse::node::Node::{Break, Continue, FunctionDeclaration, If, Let, Loop, Return};
use crate::ast::parse::precedence::Precedence;
use crate::ast::token::{KeywordToken, OperatorToken};
use crate::ast::token::LiteralToken::{False, Number, String, True};
use crate::ast::token::TokenKind::{Keyword, Operator};
use crate::core::{is_pascal_snake_case, is_snake_case};

impl Parser {
    pub(crate) fn parse_prefix(&mut self) -> crate::ast::parse::Result<Node> {
        let current = self.current()?;
        match &current.kind {
            Operator(operator) => {
                match operator {
                    OperatorToken::Plus | OperatorToken::Minus | OperatorToken::Bang => {
                        let operator = self.parse_prefix_operator()?;
                        Ok(Node::Prefix(PrefixNode {
                            operator,
                            node: Box::new(self.parse_node(Precedence::None)?),
                        }))
                    }
                    OperatorToken::OpenCurly => Ok(Node::Block(self.parse_block()?)),
                    OperatorToken::OpenParen => Ok(Node::Parenthesized(self.parse_parenthesized()?)),
                    _ => Err(Error::unsupported(self.advance()?))
                }
            }
            Keyword(keyword) => {
                match keyword {
                    KeywordToken::Break => Ok(Break(self.parse_break()?)),
                    KeywordToken::Continue => Ok(Continue(self.parse_continue()?)),
                    KeywordToken::Function => Ok(FunctionDeclaration(self.parse_function_declaration()?)),
                    KeywordToken::If => Ok(If(self.parse_if()?)),
                    KeywordToken::Let => Ok(Let(self.parse_let()?)),
                    KeywordToken::Loop => Ok(Loop(self.parse_loop()?)),
                    KeywordToken::Return => Ok(Return(self.parse_return()?)),
                    _ => Err(Error::unsupported(self.advance()?))
                }
            }
            _ => match current {
                _ if current.is_literal(Number) => Ok(Node::Literal(self.parse_literal_number()?)),
                _ if current.is_literal(True) => Ok(Node::Literal(self.parse_literal_true()?)),
                _ if current.is_literal(False) => Ok(Node::Literal(self.parse_literal_false()?)),
                _ if current.is_literal(String) => Ok(Node::Literal(self.parse_literal_string()?)),
                _ if current.is_identifier() => {
                    if is_snake_case(current.value()) {
                        Ok(Node::Identifier(self.parse_identifier()?))
                    } else if is_pascal_snake_case(current.value()) {
                        Ok(Node::Type(self.parse_type()?))
                    } else {
                        unreachable!()
                    }
                }
                _ => Err(Error::unsupported(self.advance()?))
            }
        }
    }

    pub(crate) fn parse_prefix_operator(&mut self) -> crate::ast::parse::Result<PrefixOperator> {
        let token = self.advance()?;
        match &token.kind {
            Operator(operator) => match operator {
                OperatorToken::Plus => Ok(PrefixOperator::Plus(token)),
                OperatorToken::Minus => Ok(PrefixOperator::Negate(token)),
                OperatorToken::Bang => Ok(PrefixOperator::Not(token)),
                _ => Err(UnsupportedToken(token))
            }
            _ => Err(UnsupportedToken(token))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::lex;
    use crate::ast::parse::node::{PrefixNode, PrefixOperator};
    use crate::ast::parse::Node;
    use crate::ast::parse::Parser;
    use crate::ast::token::{operator, test_token};
    use crate::ast::token::OperatorToken::{Bang, Minus, Plus};

    macro_rules! parse_prefix {
    ($($name:ident, $input:expr => $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                println!("Test input: {:?}", $input);
                let tokens = lex($input).unwrap();
                let mut parser = Parser::new(tokens);
                let result = parser.parse().unwrap();
                assert_eq!(result.len(), 1);

                let Node::Prefix(PrefixNode{ ref operator, ref node }) = result[0] else { panic!() };
                assert_eq!(*operator, $ expected);
            }
        )*
        };
    }

    parse_prefix! {
        plus, "+2" => PrefixOperator::Plus(test_token(operator(Plus), "+")),
        negate, "-2" => PrefixOperator::Negate(test_token(operator(Minus), "-")),
        notl, "!true" => PrefixOperator::Not(test_token(operator(Bang), "!")),
    }


    macro_rules! parse_prefix_operator_test {
    ($($name:ident, $input:expr => $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                println!("Test input: {:?}", $input);
                let tokens = lex($input).unwrap();
                let mut parser = Parser::new(tokens);
                let result = parser.parse_prefix_operator().unwrap();
                assert_eq!(result, $expected);
            }
        )*
        };
    }

    parse_prefix_operator_test! {
        operator_plus, "+" => PrefixOperator::Plus(test_token(operator(Plus), "+")),
        operator_negate, "-" => PrefixOperator::Negate(test_token(operator(Minus), "-")),
        operator_not, "!" => PrefixOperator::Not(test_token(operator(Bang), "!")),
    }
}