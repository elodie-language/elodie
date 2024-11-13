use std::str::FromStr;

use crate::ast::parse::{Error, Parser};
use crate::ast::parse::node::LiteralNode;
use crate::ast::token::LiteralToken;

impl Parser {
    pub(crate) fn parse_literal_string(&mut self) -> crate::ast::parse::Result<LiteralNode> {
        let token = self.consume_literal(LiteralToken::String)?;
        let value = token.value().to_string();
        return Ok(LiteralNode::String { token, value });
    }

    pub(crate) fn parse_literal_number(&mut self) -> crate::ast::parse::Result<LiteralNode> {
        let token = self.consume_literal(LiteralToken::Number)?;
        let value = f64::from_str(token.value())
            .map_err(|_| Error::UnsupportedNumber(token.value().to_string()))?;

        return Ok(LiteralNode::Number { token, value });
    }

    pub(crate) fn parse_literal_true(&mut self) -> crate::ast::parse::Result<LiteralNode> {
        let token = self.consume_literal(LiteralToken::True)?;
        return Ok(LiteralNode::Boolean { token, value: true });
    }

    pub(crate) fn parse_literal_false(&mut self) -> crate::ast::parse::Result<LiteralNode> {
        let token = self.consume_literal(LiteralToken::False)?;
        return Ok(LiteralNode::Boolean { token, value: false });
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::lex::lex;
    use crate::ast::parse::node::LiteralNode;
    use crate::ast::parse::node::Node::Literal;
    use crate::ast::parse::parse;
    use crate::ast::token::LiteralToken;
    use crate::ast::token::LiteralToken::{False, String, True};

    #[test]
    fn string() {
        let tokens = lex("'Elodie'").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Literal(LiteralNode::String { value, token }) = &result[0] else { panic!() };
        assert_eq!(value.as_str(), "Elodie");
        assert!(token.is_literal(String));
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

                let Literal(LiteralNode::Number { value, token }) = &result[0] else { panic!() };
                assert_eq!(*value, $expected);
                assert!(token.is_literal(LiteralToken::Number));
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
        let Literal(LiteralNode::Boolean { value, token }) = &result[0] else { panic!() };
        assert_eq!(*value, true);
        assert!(token.is_literal(True));
    }

    #[test]
    fn r#false() {
        let tokens = lex("false").unwrap();
        let result = parse(tokens).unwrap();
        let Literal(LiteralNode::Boolean { value, token }) = &result[0] else { panic!() };
        assert_eq!(*value, false);
        assert!(token.is_literal(False));
    }
}