use SeparatorToken::Comma;

use crate::ast::parse::node::TupleNode;
use crate::ast::parse::Parser;
use crate::ast::parse::precedence::Precedence;
use crate::ast::token::{OperatorToken, SeparatorToken};
use crate::ast::token::OperatorToken::CloseParen;
use crate::ast::token::TokenKind::Separator;

impl Parser {
    pub(crate) fn parse_parenthesized(&mut self) -> crate::ast::parse::Result<TupleNode> {
        let token = self.consume_operator(OperatorToken::OpenParen)?;

        let mut nodes = Vec::new();

        loop {
            if self.current()?.is_operator(CloseParen) {
                break;
            }
            nodes.push(self.parse_node(Precedence::None)?);
            self.consume_if(Separator(Comma))?;
        }

        self.consume_operator(CloseParen)?;
        Ok(TupleNode { token, nodes })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::lex::lex;
    use crate::ast::parse::node::{InfixNode, TypeFundamentalNode, TypeNode};
    use crate::ast::parse::node::LiteralNode::Number;
    use crate::ast::parse::node::Node::{Identifier, Infix, Literal, Tuple, Type};
    use crate::ast::parse::parse;

    #[test]
    fn empty_tuple() {
        let tokens = lex("()").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Tuple(node) = &result[0] else { panic!() };
        assert_eq!(node.nodes, vec![]);
    }

    #[test]
    fn tuple_with_number() {
        let tokens = lex("(9924)").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Tuple(node) = &result[0] else { panic!() };
        let Some(node) = node.nodes.first() else { panic!() };
        let Literal(Number(number)) = &node else { panic!() };
        assert_eq!(number.value().unwrap(), 9924.);
    }

    #[test]
    fn nested_tuple() {
        let tokens = lex("(1 * ( 2 + 3 ))").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Tuple(node) = &result[0] else { panic!() };
        let Some(node) = node.nodes.first() else { panic!() };
        let Infix(InfixNode { left, operator, right }) = &node else { panic!() };

        let Literal(Number(left)) = &left.as_ref() else { panic!() };
        assert_eq!(left.value().unwrap(), 1.);

        let Tuple(node) = &right.as_ref() else { panic!() };
        let Some(node) = &node.nodes.first() else { panic!() };
        let Infix(InfixNode { left, operator, right }) = &node else { panic!() };

        let Literal(Number(left)) = &left.as_ref() else { panic!() };
        assert_eq!(left.value().unwrap(), 2.);

        let Literal(Number(right)) = &right.as_ref() else { panic!() };
        assert_eq!(right.value().unwrap(), 3.);
    }

    #[test]
    fn tuple_with_identifier() {
        let tokens = lex("(u)").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Tuple(node) = &result[0] else { panic!() };
        let Some(node) = node.nodes.first() else { panic!() };
        let Identifier(node) = node else { panic!() };
        assert_eq!(node.identifier(), "u");
    }

    #[test]
    fn tuple_with_identifier_and_type() {
        let tokens = lex("(u: Bool)").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Tuple(node) = &result[0] else { panic!() };
        let Some(node) = node.nodes.first() else { panic!() };
        let Infix(InfixNode { left, operator, right }) = &node else { panic!() };

        let Identifier(identifier) = &left.as_ref() else { panic!() };
        assert_eq!(identifier.identifier(), "u");

        let Type(TypeNode::Fundamental(TypeFundamentalNode::Boolean(_))) = right.as_ref() else { panic!() };
    }

    #[test]
    fn tuple_with_multiple_identifiers() {
        let tokens = lex("(u,v)").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Tuple(node) = &result[0] else { panic!() };

        let Some(Identifier(u_node)) = &node.nodes.first() else { panic!() };
        assert_eq!(u_node.identifier(), "u");

        let Some(Identifier(v_node)) = &node.nodes.last() else { panic!() };
        assert_eq!(v_node.identifier(), "v");
    }
    
    #[test]
    fn tuple_with_identifiers_and_types() {
        let tokens = lex("(u: Bool, v: String)").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Tuple(node) = &result[0] else { panic!() };

        let Some(u_node) = node.nodes.first() else { panic!() };
        let Infix(InfixNode { left, operator, right }) = &u_node else { panic!() };
        let Identifier(identifier) = &left.as_ref() else { panic!() };
        assert_eq!(identifier.identifier(), "u");
        let Type(TypeNode::Fundamental(TypeFundamentalNode::Boolean(_))) = right.as_ref() else { panic!() };

        let Some(v_node) = node.nodes.last() else { panic!() };
        let Infix(InfixNode { left, operator, right }) = &v_node else { panic!() };
        let Identifier(identifier) = &left.as_ref() else { panic!() };
        assert_eq!(identifier.identifier(), "v");
        let Type(TypeNode::Fundamental(TypeFundamentalNode::String(_))) = right.as_ref() else { panic!() };
    }
}