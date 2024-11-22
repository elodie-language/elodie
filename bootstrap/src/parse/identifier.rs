use crate::common::{is_pascal_snake_case, is_snake_case};
use crate::lex::token::TokenKind;
use crate::parse::Error::InvalidIdentifier;
use crate::parse::node::IdentifierNode;
use crate::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_identifier(&mut self) -> crate::parse::Result<IdentifierNode> {
        let token = self.consume(TokenKind::Identifier)?;
        if !is_snake_case(self.ctx.get_str(token.value())) {
            Err(InvalidIdentifier(token))
        } else {
            Ok(IdentifierNode(token))
        }
    }

    pub(crate) fn parse_type_identifier(&mut self) -> crate::parse::Result<IdentifierNode> {
        let token = self.consume(TokenKind::Identifier)?;
        if !is_pascal_snake_case(self.ctx.get_str(token.value())) {
            Err(InvalidIdentifier(token))
        } else {
            Ok(IdentifierNode(token))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Context;
    use crate::lex::lex;
    use crate::parse::parse;

    #[test]
    fn identifier() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "x").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_identifier();
        assert_eq!(ctx.get_str(node.value()), "x")
    }

    #[test]
    fn identifier_with_underscore() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "some_identifier").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_identifier();
        assert_eq!(ctx.get_str(node.value()), "some_identifier")
    }
}