use std::ops::Deref;

use KeywordToken::{Else, If};

use crate::frontend::lex::token::KeywordToken;
use crate::frontend::parse::node::{ElseNode, IfNode};
use crate::frontend::parse::precedence::Precedence;
use crate::frontend::parse::{InfixNode, InfixOperator, Node, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_if(&mut self) -> crate::frontend::parse::Result<IfNode> {
        let token = self.consume_keyword(If)?;
        // let condition = Box::new(self.parse_node(Precedence::None)?);
        let condition = self.parse_node(Precedence::None)?;

        // TODO make this recursive and walk down the tree to the out most right node and check for lambda call
        let (condition, then) = if let Node::Infix(InfixNode {
            token,
            left,
            operator,
            right,
        }) = condition
        {
            if let Node::Infix(InfixNode {
                left: inner_left,
                operator: inner_operator,
                right: inner_right,
                ..
            }) = *right
            {
                assert!(matches!(inner_operator, InfixOperator::LambdaCall(_)));
                (
                    Node::Infix(InfixNode {
                        token,
                        left,
                        operator,
                        right: inner_left,
                    }),
                    *inner_right,
                )
            } else {
                (*left, *right)
            }
        } else {
            (condition, Node::Block(self.parse_block()?))
        };

        let condition = Box::new(condition);
        let Node::Block(then) = then else {
            panic!("not block node")
        };

        let otherwise = if !self.is_eof() && self.current()?.is_keyword(Else) {
            let token = self.consume_keyword(Else)?;
            Some(ElseNode {
                token,
                block: self.parse_block()?,
            })
        } else {
            None
        };

        Ok(IfNode {
            token,
            condition,
            then,
            otherwise,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::common::Context;
    use crate::frontend::lex::lex;
    use crate::frontend::parse::node::Node::Literal;
    use crate::frontend::parse::node::{IfNode, LiteralNode};
    use crate::frontend::parse::{parse, InfixNode, InfixOperator};

    #[test]
    fn empty_if_no_else() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "if true {}").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode {
            condition,
            then,
            otherwise,
            ..
        } = result[0].as_if();

        let Literal(LiteralNode::Boolean(condition)) = condition.deref() else {
            panic!("not boolean node")
        };
        assert_eq!(condition.value(), true);
        assert_eq!(then.nodes, vec![]);
        assert_eq!(*otherwise, None);
    }

    #[test]
    fn if_true_equals_true() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "if true == false {}").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode {
            condition,
            then,
            otherwise,
            ..
        } = result[0].as_if();

        let InfixNode {
            left,
            operator,
            right,
            ..
        } = condition.as_infix()
        else {
            panic!("not infix")
        };

        let Literal(LiteralNode::Boolean(left)) = left.deref() else {
            panic!("not boolean node")
        };
        assert_eq!(left.value(), true);

        let InfixOperator::Equal(_) = operator else {
            panic!("not equals operator")
        };

        let Literal(LiteralNode::Boolean(right)) = right.deref() else {
            panic!("not boolean node")
        };
        assert_eq!(right.value(), false);

        assert_eq!(then.nodes, vec![]);
        assert_eq!(*otherwise, None);
    }

    #[test]
    fn if_multiple_then_nodes() {
        let mut ctx = Context::testing();
        let tokens = lex(
            &mut ctx,
            r#"if true {
            99
            24
        }"#,
        )
        .unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode {
            condition,
            then,
            otherwise,
            ..
        } = result[0].as_if();

        let Literal(LiteralNode::Boolean(condition)) = condition.deref() else {
            panic!("not boolean node")
        };
        assert_eq!(condition.value(), true);
        assert_eq!(then.nodes.len(), 2);

        let Some(Literal(LiteralNode::Number(first))) = then.nodes.first() else {
            panic!("not a number node")
        };
        assert_eq!(ctx.str_get(first.value()), "99");

        let Some(Literal(LiteralNode::Number(last))) = then.nodes.last() else {
            panic!("not a number node")
        };
        assert_eq!(ctx.str_get(last.value()), "24");

        assert_eq!(*otherwise, None);
    }

    #[test]
    fn empty_if_and_else() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "if true {} else {}").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode {
            condition,
            then,
            otherwise,
            ..
        } = result[0].as_if();

        let Literal(LiteralNode::Boolean(condition)) = condition.deref() else {
            panic!("not boolean node")
        };
        assert_eq!(condition.value(), true);
        assert_eq!(then.nodes, vec![]);

        let Some(otherwise) = otherwise else {
            panic!("no else node")
        };
        assert_eq!(otherwise.block.nodes, vec![]);
    }

    #[test]
    fn if_else_multiple_nodes() {
        let mut ctx = Context::testing();
        let tokens = lex(
            &mut ctx,
            r#"if true {
            1
            2
        }else{
            3
            4
        }"#,
        )
        .unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode {
            condition,
            then,
            otherwise,
            ..
        } = result[0].as_if();

        let Literal(LiteralNode::Boolean(condition)) = condition.deref() else {
            panic!("not boolean node")
        };
        assert_eq!(condition.value(), true);
        assert_eq!(then.nodes.len(), 2);

        let Some(Literal(LiteralNode::Number(first))) = then.nodes.first() else {
            panic!("not a number node")
        };
        assert_eq!(ctx.str_get(first.value()), "1");

        let Some(Literal(LiteralNode::Number(last))) = then.nodes.last() else {
            panic!("not a number node")
        };
        assert_eq!(ctx.str_get(last.value()), "2");

        let Some(otherwise) = otherwise else {
            panic!("no else node")
        };
        assert_eq!(otherwise.block.nodes.len(), 2);

        let Some(Literal(LiteralNode::Number(first))) = otherwise.block.nodes.first() else {
            panic!("not a number node")
        };
        assert_eq!(ctx.str_get(first.value()), "3");

        let Some(Literal(LiteralNode::Number(last))) = otherwise.block.nodes.last() else {
            panic!("not a number node")
        };
        assert_eq!(ctx.str_get(last.value()), "4");
    }
}
