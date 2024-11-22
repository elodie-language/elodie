use KeywordToken::Let;

use crate::lex::token::{KeywordToken, OperatorToken};
use crate::parse::node::LetNode;
use crate::parse::Parser;
use crate::parse::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_let(&mut self) -> crate::parse::Result<LetNode> {
        let token = self.consume_keyword(Let)?;
        let identifier = self.parse_identifier()?;

        let r#type = if self.current()?.is_operator(OperatorToken::Colon) {
            self.advance()?;
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume_operator(OperatorToken::Equal)?;
        let value = Box::new(self.parse_node(Precedence::None)?);

        Ok(LetNode {
            token,
            identifier,
            node: value,
            r#type,
        })
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::common::Context;
    use crate::lex::lex;
    use crate::parse::node::{LiteralNode, TypeFundamentalNode, TypeNode};
    use crate::parse::node::Node::Literal;
    use crate::parse::parse;

    #[test]
    fn let_without_type_string() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "let value = 'Elodie'").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_let();
        assert_eq!(ctx.get_str(node.identifier.value()), "value");

        assert_eq!(node.r#type, None);

        let Literal(LiteralNode::String(result)) = &node.node.deref() else { panic!() };
        assert_eq!(ctx.get_str(result.value()), "Elodie");
    }

    #[test]
    fn let_with_type_string() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "let value : String = 'Elodie'").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_let();
        assert_eq!(ctx.get_str(node.identifier.value()), "value");

        let Some(TypeNode::Fundamental(TypeFundamentalNode::String(_))) = node.r#type else { panic!() };

        let Literal(LiteralNode::String(result)) = &node.node.deref() else { panic!() };
        assert_eq!(ctx.get_str(result.value()), "Elodie");
    }

    #[test]
    fn let_without_type_number() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "let value = 9924").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_let();
        assert_eq!(ctx.get_str(node.identifier.value()), "value");

        assert_eq!(node.r#type, None);

        let Literal(LiteralNode::Number(result)) = &node.node.deref() else { panic!() };
        assert_eq!(ctx.get_str(result.value()), "9924");
    }

    #[test]
    fn let_without_type_boolean() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "let value = false").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = &result[0].as_let();
        assert_eq!(ctx.get_str(node.identifier.value()), "value");
        assert_eq!(node.r#type, None);

        let Literal(LiteralNode::Boolean(result)) = &node.node.deref() else { panic!() };
        assert_eq!(result.value(), false);
    }
}