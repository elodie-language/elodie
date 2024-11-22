use crate::ast::parse::Error::InvalidIdentifier;
use crate::ast::parse::node::IdentifierNode;
use crate::ast::parse::Parser;
use crate::ast::lex::token::TokenKind;
use crate::common::{is_pascal_snake_case, is_snake_case};

impl Parser {
    pub(crate) fn parse_identifier(&mut self) -> crate::ast::parse::Result<IdentifierNode> {
        let token = self.consume(TokenKind::Identifier)?;
        if !is_snake_case(token.value()) {
            Err(InvalidIdentifier(token))
        } else {
            Ok(IdentifierNode(token))
        }
    }

    pub(crate) fn parse_type_identifier(&mut self) -> crate::ast::parse::Result<IdentifierNode> {
        let token = self.consume(TokenKind::Identifier)?;
        if !is_pascal_snake_case(token.value()) {
            Err(InvalidIdentifier(token))
        } else {
            Ok(IdentifierNode(token))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::lex::lex;
    use crate::ast::parse::parse;

    #[test]
    fn identifier() {
        let tokens = lex("x").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_identifier();
        assert_eq!(node.value(), "x")
    }

    #[test]
    fn identifier_with_underscore() {
        let tokens = lex("some_identifier").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_identifier();
        assert_eq!(node.value(), "some_identifier")
    }
}