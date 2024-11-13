use std::str::FromStr;

use crate::new_ast::parse::{Error, Parser};
use crate::new_ast::parse::node::LiteralNode;
use crate::new_ast::token::LiteralToken;

impl Parser {
    pub(crate) fn parse_literal_number(&mut self) -> crate::new_ast::parse::Result<LiteralNode> {
        let token = self.consume_literal(LiteralToken::Number)?;
        return Ok(LiteralNode::Number(f64::from_str(token.value())
            .map_err(|_| Error::UnsupportedNumber(token.value().to_string()))?));
    }
}

#[cfg(test)]
mod tests {
    use crate::new_ast::lex::lex;
    use crate::new_ast::parse::node::LiteralNode;
    use crate::new_ast::parse::node::Node::Literal;
    use crate::new_ast::parse::parse;

    macro_rules! parse_number_test {
    ($($name:ident, $input:expr => $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                println!("Test input: {:?}", $input);
                let tokens = lex($input).unwrap();
                let result = parse(tokens).unwrap();
                assert_eq!(result.len(), 1);
                assert_eq!(result[0], Literal($expected))
            }
        )*
    };
    }

    parse_number_test! {
        number_42, "42" => LiteralNode::Number(42.0),
        number_42_dot_0, "42.0" => LiteralNode::Number(42.0),
    }
}