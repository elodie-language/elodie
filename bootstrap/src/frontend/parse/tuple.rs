use SeparatorToken::Comma;

use crate::frontend::lex::token::OperatorToken::CloseParen;
use crate::frontend::lex::token::TokenKind::Separator;
use crate::frontend::lex::token::{OperatorToken, SeparatorToken, Token};
use crate::frontend::parse::node::TupleNode;
use crate::frontend::parse::precedence::Precedence;
use crate::frontend::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_tuple(&mut self) -> crate::frontend::parse::Result<TupleNode> {
        let token = self.consume_operator(OperatorToken::OpenParen)?;
        self.parse_tuple_call(token)
    }

    pub(crate) fn parse_tuple_call(
        &mut self,
        operator: Token,
    ) -> crate::frontend::parse::Result<TupleNode> {
        let mut nodes = Vec::new();
        loop {
            self.skip_new_line()?;

            if self.current()?.is_operator(CloseParen) {
                break;
            }
            nodes.push(self.parse_node(Precedence::None)?);
            self.consume_if(Separator(Comma))?;
        }

        self.consume_operator(CloseParen)?;
        Ok(TupleNode {
            token: operator,
            nodes,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::frontend::context::Context;
    use crate::frontend::lex::lex;
    use crate::frontend::parse::node::LiteralNode::Number;
    use crate::frontend::parse::node::Node::{Identifier, Infix, Literal, Type};
    use crate::frontend::parse::node::{InfixNode, TypeNode};
    use crate::frontend::parse::{parse, InfixOperator, LiteralNode};

    #[test]
    fn empty_tuple() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "()").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_tuple();
        assert_eq!(node.nodes, vec![]);
    }

    #[test]
    fn tuple_with_number() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "(9924)").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_tuple();
        let Some(node) = node.nodes.first() else {
            panic!()
        };
        let Literal(Number(number)) = &node else {
            panic!()
        };
        assert_eq!(ctx.get_str(number.value()), "9924");
    }

    #[test]
    fn nested_tuple() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "(1 * ( 2 + 3 ))").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_tuple();
        let Some(node) = node.nodes.first() else {
            panic!()
        };
        let Infix(InfixNode {
            left,
            operator,
            right,
            ..
        }) = &node
        else {
            panic!()
        };

        let Literal(Number(left)) = &left.as_ref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(left.value()), "1");

        let node = right.as_tuple();
        let Some(node) = node.nodes.first() else {
            panic!()
        };
        let InfixNode {
            left,
            operator,
            right,
            ..
        } = &node.as_infix();

        let Literal(Number(left)) = &left.as_ref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(left.value()), "2");

        let Literal(Number(right)) = &right.as_ref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(right.value()), "3");
    }

    #[test]
    fn tuple_with_identifier() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "(u)").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = &result[0].as_tuple();
        let Some(node) = node.nodes.first() else {
            panic!()
        };
        let Identifier(node) = node else { panic!() };
        assert_eq!(ctx.get_str(node.value()), "u");
    }

    #[test]
    fn tuple_with_identifier_and_type() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "(u: Bool)").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_tuple();
        let Some(node) = node.nodes.first() else {
            panic!()
        };
        let Infix(InfixNode {
            left,
            operator,
            right,
            ..
        }) = &node
        else {
            panic!()
        };

        let identifier = &left.as_identifier();
        assert_eq!(ctx.get_str(identifier.value()), "u");

        let Type(TypeNode::Boolean(_)) = right.as_ref() else {
            panic!()
        };
    }

    #[test]
    fn tuple_with_multiple_identifiers() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "(u,v)").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_tuple();

        let Some(Identifier(u_node)) = &node.nodes.first() else {
            panic!()
        };
        assert_eq!(ctx.get_str(u_node.value()), "u");

        let Some(Identifier(v_node)) = &node.nodes.last() else {
            panic!()
        };
        assert_eq!(ctx.get_str(v_node.value()), "v");
    }

    #[test]
    fn tuple_with_identifiers_and_types() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "(u: Bool, v: String)").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_tuple();

        let Some(u_node) = node.nodes.first() else {
            panic!()
        };
        let Infix(InfixNode {
            left,
            operator,
            right,
            ..
        }) = &u_node
        else {
            panic!()
        };
        let Identifier(identifier) = &left.as_ref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(identifier.value()), "u");
        let Type(TypeNode::Boolean(_)) = right.as_ref() else {
            panic!()
        };

        let Some(v_node) = node.nodes.last() else {
            panic!()
        };
        let Infix(InfixNode {
            left,
            operator,
            right,
            ..
        }) = &v_node
        else {
            panic!()
        };
        let Identifier(identifier) = &left.as_ref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(identifier.value()), "v");
        let Type(TypeNode::String(_)) = right.as_ref() else {
            panic!()
        };
    }

    #[test]
    fn tuple_with_identifiers_and_declaration() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "(u = 1, v = 2)").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_tuple();

        let Some(u_node) = node.nodes.first() else {
            panic!()
        };
        let Infix(InfixNode {
            left,
            operator,
            right,
            ..
        }) = &u_node
        else {
            panic!()
        };
        let Identifier(identifier) = &left.as_ref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(identifier.value()), "u");
        assert!(matches!(operator, InfixOperator::Assign(_)));
        let Literal(LiteralNode::Number(number)) = right.as_ref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(number.value()), "1");

        let Some(v_node) = node.nodes.last() else {
            panic!()
        };
        let Infix(InfixNode {
            left,
            operator,
            right,
            ..
        }) = &v_node
        else {
            panic!()
        };
        let Identifier(identifier) = &left.as_ref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(identifier.value()), "v");
        assert!(matches!(operator, InfixOperator::Assign(_)));
        let Literal(LiteralNode::Number(number)) = right.as_ref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(number.value()), "2");
    }

    #[test]
    fn multiline_tuple() {
        let mut ctx = Context::new();
        let tokens = lex(
            &mut ctx,
            r#"(
        u: Bool,
        v: String
        )"#,
        )
        .unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_tuple();

        let Some(u_node) = node.nodes.first() else {
            panic!()
        };
        let Infix(InfixNode {
            left,
            operator,
            right,
            ..
        }) = &u_node
        else {
            panic!()
        };
        let Identifier(identifier) = &left.as_ref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(identifier.value()), "u");
        let Type(TypeNode::Boolean(_)) = right.as_ref() else {
            panic!()
        };

        let Some(v_node) = node.nodes.last() else {
            panic!()
        };
        let Infix(InfixNode {
            left,
            operator,
            right,
            ..
        }) = &v_node
        else {
            panic!()
        };
        let Identifier(identifier) = &left.as_ref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(identifier.value()), "v");
        let Type(TypeNode::String(_)) = right.as_ref() else {
            panic!()
        };
    }
}
