use crate::ast::parse::node::ParenthesizedNode;
use crate::ast::parse::Parser;
use crate::ast::parse::precedence::Precedence;
use crate::ast::token::OperatorToken;
use crate::ast::token::OperatorToken::CloseParen;

impl Parser {
    pub(crate) fn parse_parenthesized(&mut self) -> crate::ast::parse::Result<ParenthesizedNode> {
        let token = self.consume_operator(OperatorToken::OpenParen)?;

        let node = if self.current()?.is_operator(CloseParen) {
            None
        } else {
            Some(Box::new(self.parse_node(Precedence::None)?))
        };

        self.consume_operator(CloseParen)?;
        Ok(ParenthesizedNode { token, node })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::lex::lex;
    use crate::ast::parse::node::{InfixNode, TypeFundamentalNode, TypeNode};
    use crate::ast::parse::node::LiteralNode::Number;
    use crate::ast::parse::node::Node::{Identifier, Infix, Literal, Parenthesized, Type};
    use crate::ast::parse::parse;

    #[test]
    fn empty_parenthesis() {
        let tokens = lex("()").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Parenthesized(node) = &result[0] else { panic!() };
        assert_eq!(node.node, None);
    }

    #[test]
    fn parenthesis_with_number() {
        let tokens = lex("(9924)").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Parenthesized(node) = &result[0] else { panic!() };
        let Some(Literal(Number(number))) = &node.node.as_deref() else { panic!() };
        assert_eq!(number.value().unwrap(), 9924.);
    }

    #[test]
    fn parenthesis_nested() {
        let tokens = lex("(1 * ( 2 + 3 ))").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Parenthesized(node) = &result[0] else { panic!() };
        let Some(Infix(InfixNode { left, operator, right })) = &node.node.as_deref() else { panic!() };

        let Literal(Number(left)) = &left.as_ref() else { panic!() };
        assert_eq!(left.value().unwrap(), 1.);

        let Parenthesized(node) = &right.as_ref() else { panic!() };
        let Some(Infix(InfixNode { left, operator, right })) = &node.node.as_deref() else { panic!() };

        let Literal(Number(left)) = &left.as_ref() else { panic!() };
        assert_eq!(left.value().unwrap(), 2.);

        let Literal(Number(right)) = &right.as_ref() else { panic!() };
        assert_eq!(right.value().unwrap(), 3.);
    }

    #[test]
    fn parenthesis_with_identifier() {
        let tokens = lex("(u)").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Parenthesized(node) = &result[0] else { panic!() };
        let Some(Identifier(node)) = &node.node.as_deref() else { panic!() };
        assert_eq!(node.identifier(), "u");
    }

    #[test]
    fn parenthesis_with_identifier_and_type() {
        let tokens = lex("(u: Bool)").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Parenthesized(node) = &result[0] else { panic!() };
        let Some(Infix(InfixNode { left, operator, right })) = &node.node.as_deref() else { panic!() };

        let Identifier(identifier) = &left.as_ref() else { panic!() };
        assert_eq!(identifier.identifier(), "u");

        let Type(TypeNode::Fundamental(TypeFundamentalNode::Boolean(_))) = right.as_ref() else { panic!() };
    }


    #[ignore]
    #[test]
    fn parenthesis_with_multiple_identifier() {
        let tokens = lex("(u,v)").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Parenthesized(node) = &result[0] else { panic!() };
        let Some(Identifier(node)) = &node.node.as_deref() else { panic!() };
        assert_eq!(node.identifier(), "u");
    }
}