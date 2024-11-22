use std::str::FromStr;

use crate::parse::node::{LiteralBooleanNode, LiteralNode, LiteralNumberNode, LiteralStringNode};
use crate::parse::Parser;
use crate::lex::token::LiteralToken;

impl Parser {
    pub(crate) fn parse_literal_string(&mut self) -> crate::parse::Result<LiteralNode> {
        let token = self.consume_literal(LiteralToken::String)?;
        return Ok(LiteralNode::String(LiteralStringNode(token)));
    }

    pub(crate) fn parse_literal_number(&mut self) -> crate::parse::Result<LiteralNode> {
        let token = self.consume_literal(LiteralToken::Number)?;
        return Ok(LiteralNode::Number(LiteralNumberNode(token)));
    }

    pub(crate) fn parse_literal_true(&mut self) -> crate::parse::Result<LiteralNode> {
        let token = self.consume_literal(LiteralToken::True)?;
        return Ok(LiteralNode::Boolean(LiteralBooleanNode(token)));
    }

    pub(crate) fn parse_literal_false(&mut self) -> crate::parse::Result<LiteralNode> {
        let token = self.consume_literal(LiteralToken::False)?;
        return Ok(LiteralNode::Boolean(LiteralBooleanNode(token)));
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::node::LiteralNode;
    use crate::parse::node::Node::Literal;
    use crate::parse::parse;
    use crate::lex::lex;

    #[test]
    fn string() {
        let tokens = lex("'Elodie'").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Literal(LiteralNode::String(node)) = &result[0] else { panic!() };
        assert_eq!(node.value(), "Elodie");
    }

    macro_rules! parse_number_test {
    ($($name:ident, $input:expr => $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                println!("Test input: {:?}", $input);
                let tokens = lex($input).unwrap();
                let result = parse(tokens).unwrap();
                assert_eq!(result.len(), 1);

                let Literal(LiteralNode::Number(node)) = &result[0] else { panic!() };
                assert_eq!(node.value().unwrap(), $expected);
            }
        )*
    };
    }

    parse_number_test! {
        number_42, "42" =>  42.0f64,
        number_42_dot_0, "42.0" => 42.0f64,
    }

    #[test]
    fn r#true() {
        let tokens = lex("true").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Literal(LiteralNode::Boolean(node)) = &result[0] else { panic!() };
        assert_eq!(node.value(), true);
    }

    #[test]
    fn r#false() {
        let tokens = lex("false").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Literal(LiteralNode::Boolean(node)) = &result[0] else { panic!() };
        assert_eq!(node.value(), false);
    }
}