use OperatorToken::CloseCurly;
use SeparatorToken::NewLine;

use crate::ast::parse::node::{BreakNode, ContinueNode, LoopNode};
use crate::ast::parse::Parser;
use crate::ast::parse::precedence::Precedence;
use crate::ast::token::{KeywordToken, OperatorToken, SeparatorToken};

impl Parser {
    pub(crate) fn parse_loop(&mut self) -> crate::ast::parse::Result<LoopNode> {
        let token = self.consume_keyword(KeywordToken::Loop)?;
        Ok(LoopNode { token, block: self.parse_block()? })
    }

    pub(crate) fn parse_continue(&mut self) -> crate::ast::parse::Result<ContinueNode> {
        let token = self.consume_keyword(KeywordToken::Continue)?;
        Ok(ContinueNode { token })
    }

    pub(crate) fn parse_break(&mut self) -> crate::ast::parse::Result<BreakNode> {
        let token = self.consume_keyword(KeywordToken::Break)?;

        let current = self.current()?;
        let has_result = !current.is_operator(CloseCurly) && !current.is_separator(NewLine);

        let result = if has_result { Some(Box::new(self.parse_node(Precedence::None)?)) } else { None };
        Ok(BreakNode { token, result })
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::ast::lex::lex;
    use crate::ast::parse::node::LiteralNode;
    use crate::ast::parse::node::Node::{Break, Continue, Literal, Loop};
    use crate::ast::parse::parse;

    #[test]
    fn empty_loop() {
        let tokens = lex("loop{}").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Loop(node) = &result[0] else { panic!() };
        assert_eq!(node.block.nodes, vec![]);
    }

    #[test]
    fn loop_with_single_node() {
        let tokens = lex("loop{ 42 }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Loop(node) = &result[0] else { panic!() };
        assert_eq!(node.block.nodes.len(), 1);

        let Literal(LiteralNode::Number(number)) = &node.block.nodes[0] else { panic!() };
        assert_eq!(number.value().unwrap(), 42.0);
    }

    #[test]
    fn nested_loop() {
        let tokens = lex("loop{ loop { 42 } }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Loop(outer_loop) = &result[0] else { panic!() };
        assert_eq!(outer_loop.block.nodes.len(), 1);

        let Loop(inner_loop) = &outer_loop.block.nodes[0] else { panic!() };
        assert_eq!(inner_loop.block.nodes.len(), 1);

        let Literal(LiteralNode::Number(number)) = &inner_loop.block.nodes[0] else { panic!() };
        assert_eq!(number.value().unwrap(), 42.0);
    }

    #[test]
    fn loop_continue() {
        let tokens = lex("loop{ continue }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Loop(node) = &result[0] else { panic!() };
        assert_eq!(node.block.nodes.len(), 1);

        let Continue(node) = &node.block.nodes[0] else { panic!("not continue") };
    }

    #[test]
    fn loop_break() {
        let tokens = lex("loop{ break }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Loop(node) = &result[0] else { panic!() };
        assert_eq!(node.block.nodes.len(), 1);

        let Break(node) = &node.block.nodes[0] else { panic!("not break") };
        assert_eq!(node.result, None);
    }

    #[test]
    fn loop_break_with_result() {
        let tokens = lex("loop{ break 9924 }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Loop(node) = &result[0] else { panic!() };
        assert_eq!(node.block.nodes.len(), 1);

        let Break(node) = &node.block.nodes[0] else { panic!("not break") };
        let Some(ref node) = node.result else { panic!() };

        let Literal(LiteralNode::Number(node)) = &node.deref() else { panic!() };
        assert_eq!(node.value().unwrap(), 9924.0);
    }
}