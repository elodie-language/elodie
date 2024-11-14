use KeywordToken::Let;

use crate::ast::parse::node::LetNode;
use crate::ast::parse::Parser;
use crate::ast::parse::precedence::Precedence;
use crate::ast::token::{KeywordToken, OperatorToken};

impl Parser {
    pub(crate) fn parse_let(&mut self) -> crate::ast::parse::Result<LetNode> {
        let token = self.consume_keyword(Let)?;
        let identifier = self.parse_identifier()?;
        self.consume_operator(OperatorToken::Equal)?;
        let value = Box::new(self.parse_node(Precedence::None)?);

        Ok(LetNode {
            token,
            identifier,
            value,
            r#type: None,
        })
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::ast::lex::lex;
    use crate::ast::parse::node::LiteralNode;
    use crate::ast::parse::node::Node::{Let, Literal};
    use crate::ast::parse::parse;

    #[test]
    fn let_without_type_string() {
        let tokens = lex("let value = 'Elodie'").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Let(node) = &result[0] else { panic!() };
        assert_eq!(node.identifier.identifier(), "value");

        assert_eq!(node.r#type, None);

        let Literal(LiteralNode::String(result)) = &node.value.deref() else { panic!() };
        assert_eq!(result.value(), "Elodie");
    }

    #[test]
    fn let_without_type_number() {
        let tokens = lex("let value = 9924").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Let(node) = &result[0] else { panic!() };
        assert_eq!(node.identifier.identifier(), "value");

        assert_eq!(node.r#type, None);

        let Literal(LiteralNode::Number(result)) = &node.value.deref() else { panic!() };
        assert_eq!(result.value().unwrap(), 9924.0);
    }

    #[test]
    fn let_without_type_boolean() {
        let tokens = lex("let value = false").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Let(node) = &result[0] else { panic!() };
        assert_eq!(node.identifier.identifier(), "value");

        assert_eq!(node.r#type, None);

        let Literal(LiteralNode::Boolean(result)) = &node.value.deref() else { panic!() };
        assert_eq!(result.value(), false);
    }
}