use KeywordToken::{Else, If};

use crate::lex::token::KeywordToken;
use crate::parse::node::{ElseNode, IfNode};
use crate::parse::Parser;
use crate::parse::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_if(&mut self) -> crate::parse::Result<IfNode> {
        let token = self.consume_keyword(If)?;
        let condition = Box::new(self.parse_node(Precedence::None)?);
        let then = self.parse_block()?;

        let otherwise = if !self.is_eof() && self.current()?.is_keyword(Else) {
            let token = self.consume_keyword(Else)?;
            Some(ElseNode { token, block: self.parse_block()? })
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
    use crate::lex::lex;
    use crate::parse::node::{IfNode, LiteralNode};
    use crate::parse::node::Node::Literal;
    use crate::parse::parse;

    #[test]
    fn empty_if_no_else() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "if true {}").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode { condition, then, otherwise, .. } = result[0].as_if();

        let Literal(LiteralNode::Boolean(condition)) = condition.deref() else { panic!("not boolean node") };
        assert_eq!(condition.value(), true);
        assert_eq!(then.nodes, vec![]);
        assert_eq!(*otherwise, None);
    }

    #[test]
    fn if_multiple_then_nodes() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, r#"if true {
            99
            24
        }"#).unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode { condition, then, otherwise, .. } = result[0].as_if();

        let Literal(LiteralNode::Boolean(condition)) = condition.deref() else { panic!("not boolean node") };
        assert_eq!(condition.value(), true);
        assert_eq!(then.nodes.len(), 2);

        let Some(Literal(LiteralNode::Number(first))) = then.nodes.first() else { panic!("not a number node") };
        assert_eq!(ctx.get_str(first.value()), "99");

        let Some(Literal(LiteralNode::Number(last))) = then.nodes.last() else { panic!("not a number node") };
        assert_eq!(ctx.get_str(last.value()), "24");

        assert_eq!(*otherwise, None);
    }

    #[test]
    fn empty_if_and_else() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "if true {} else {}").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode { condition, then, otherwise, .. } = result[0].as_if();

        let Literal(LiteralNode::Boolean(condition)) = condition.deref() else { panic!("not boolean node") };
        assert_eq!(condition.value(), true);
        assert_eq!(then.nodes, vec![]);

        let Some(otherwise) = otherwise else { panic!("no else node") };
        assert_eq!(otherwise.block.nodes, vec![]);
    }

    #[test]
    fn if_else_multiple_nodes() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, r#"if true {
            1
            2
        }else{
            3
            4
        }"#).unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode { condition, then, otherwise, .. } = result[0].as_if();

        let Literal(LiteralNode::Boolean(condition)) = condition.deref() else { panic!("not boolean node") };
        assert_eq!(condition.value(), true);
        assert_eq!(then.nodes.len(), 2);

        let Some(Literal(LiteralNode::Number(first))) = then.nodes.first() else { panic!("not a number node") };
        assert_eq!(ctx.get_str(first.value()), "1");

        let Some(Literal(LiteralNode::Number(last))) = then.nodes.last() else { panic!("not a number node") };
        assert_eq!(ctx.get_str(last.value()), "2");

        let Some(otherwise) = otherwise else { panic!("no else node") };
        assert_eq!(otherwise.block.nodes.len(), 2);

        let Some(Literal(LiteralNode::Number(first))) = otherwise.block.nodes.first() else { panic!("not a number node") };
        assert_eq!(ctx.get_str(first.value()), "3");

        let Some(Literal(LiteralNode::Number(last))) = otherwise.block.nodes.last() else { panic!("not a number node") };
        assert_eq!(ctx.get_str(last.value()), "4");
    }
}