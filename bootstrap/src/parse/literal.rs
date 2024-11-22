use std::str::FromStr;

use crate::lex::token::LiteralToken;
use crate::parse::node::{LiteralBooleanNode, LiteralNode, LiteralNumberNode, LiteralStringNode};
use crate::parse::Parser;

impl<'a> Parser<'a> {
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
    use crate::common::Context;
    use crate::lex::lex;
    use crate::parse::node::LiteralNode;
    use crate::parse::node::Node::Literal;
    use crate::parse::parse;

    #[test]
    fn string() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "'Elodie'").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Literal(LiteralNode::String(node)) = &result[0] else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "Elodie");
    }

    #[test]
    fn number_42() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "42").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Literal(LiteralNode::Number(node)) = &result[0] else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "42");
    }

    #[test]
    fn r#true() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "true").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Literal(LiteralNode::Boolean(node)) = &result[0] else { panic!() };
        assert_eq!(node.value(), true);
    }

    #[test]
    fn r#false() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "false").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Literal(LiteralNode::Boolean(node)) = &result[0] else { panic!() };
        assert_eq!(node.value(), false);
    }
}