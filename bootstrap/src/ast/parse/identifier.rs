use crate::ast::parse::node::IdentifierNode;
use crate::ast::parse::Parser;
use crate::ast::token::TokenKind;

impl Parser {
    pub(crate) fn parse_identifier(&mut self) -> crate::ast::parse::Result<IdentifierNode> {
        let token = self.consume(TokenKind::Identifier)?;
        Ok(IdentifierNode(token))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::lex::lex;
    use crate::ast::parse::node::Node::Identifier;
    use crate::ast::parse::parse;

    #[test]
    fn identifier() {
        let tokens = lex("x").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Identifier(node) = &result[0] else { panic!() };
        assert_eq!(node.identifier(), "x")
    }

    #[test]
    fn identifier_with_underscore() {
        let tokens = lex("some_identifier").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Identifier(node) = &result[0] else { panic!() };
        assert_eq!(node.identifier(), "some_identifier")
    }
}