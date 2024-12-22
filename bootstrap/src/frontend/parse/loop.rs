use OperatorToken::CloseCurly;
use SeparatorToken::NewLine;

use crate::frontend::lex::token::{KeywordToken, OperatorToken, SeparatorToken};
use crate::frontend::parse::node::{BreakNode, ContinueNode, LoopNode};
use crate::frontend::parse::precedence::Precedence;
use crate::frontend::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_loop(&mut self) -> crate::frontend::parse::Result<LoopNode> {
        let token = self.consume_keyword(KeywordToken::Loop)?;
        Ok(LoopNode {
            token,
            block: self.parse_block()?,
        })
    }

    pub(crate) fn parse_continue(&mut self) -> crate::frontend::parse::Result<ContinueNode> {
        let token = self.consume_keyword(KeywordToken::Continue)?;
        Ok(ContinueNode { token })
    }

    pub(crate) fn parse_break(&mut self) -> crate::frontend::parse::Result<BreakNode> {
        let token = self.consume_keyword(KeywordToken::Break)?;

        let current = self.current()?;
        let has_result = !current.is_operator(CloseCurly) && !current.is_separator(NewLine);

        let result = if has_result {
            Some(Box::new(self.parse_node(Precedence::None)?))
        } else {
            None
        };
        Ok(BreakNode { token, result })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::common::context::Context;
    use crate::frontend::lex::lex;
    use crate::frontend::parse::node::LiteralNode;
    use crate::frontend::parse::node::Node::{Continue, Literal};
    use crate::frontend::parse::parse;

    #[test]
    fn empty_loop() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "loop{}").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_loop();
        assert_eq!(node.block.nodes, vec![]);
    }

    #[test]
    fn loop_with_single_node() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "loop{ 42 }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_loop();
        assert_eq!(node.block.nodes.len(), 1);

        let Literal(LiteralNode::Number(number)) = &node.block.nodes[0] else {
            panic!()
        };
        assert_eq!(ctx.get_str(number.value()), "42");
    }

    #[test]
    fn nested_loop() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "loop{ loop { 42 } }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let outer_loop = result[0].as_loop();
        assert_eq!(outer_loop.block.nodes.len(), 1);

        let inner_loop = &outer_loop.block.nodes[0].as_loop();
        assert_eq!(inner_loop.block.nodes.len(), 1);

        let Literal(LiteralNode::Number(number)) = &inner_loop.block.nodes[0] else {
            panic!()
        };
        assert_eq!(ctx.get_str(number.value()), "42");
    }

    #[test]
    fn loop_continue() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "loop{ continue }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_loop();
        assert_eq!(node.block.nodes.len(), 1);

        let Continue(_) = &node.block.nodes[0] else {
            panic!("not continue")
        };
    }

    #[test]
    fn loop_break() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "loop{ break }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_loop();
        assert_eq!(node.block.nodes.len(), 1);

        let node = node.block.nodes[0].as_break();
        assert_eq!(node.result, None);
    }

    #[test]
    fn loop_break_with_result() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "loop{ break 9924 }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_loop();
        assert_eq!(node.block.nodes.len(), 1);

        let node = node.block.nodes[0].as_break();
        let Some(ref node) = node.result else {
            panic!()
        };

        let Literal(LiteralNode::Number(node)) = &node.deref() else {
            panic!()
        };
        assert_eq!(ctx.get_str(node.value()), "9924");
    }
}
