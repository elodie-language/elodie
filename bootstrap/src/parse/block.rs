use OperatorToken::{CloseCurly, OpenCurly};
use TokenKind::Separator;

use crate::lex::token::{OperatorToken, TokenKind};
use crate::lex::token::SeparatorToken::NewLine;
use crate::parse::node::BlockNode;
use crate::parse::Parser;
use crate::parse::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_block(&mut self) -> crate::parse::Result<BlockNode> {
        self.consume_operator(OpenCurly)?;
        let result = self.parse_block_inner()?;
        self.consume_operator(CloseCurly)?;
        Ok(result)
    }

    pub(crate) fn parse_block_inner(&mut self) -> crate::parse::Result<BlockNode> {
        let mut nodes = Vec::new();
        loop {
            self.consume_while(Separator(NewLine))?;
            if self.current()?.is_operator(CloseCurly) {
                break;
            }
            nodes.push(self.parse_node(Precedence::None)?);
        }
        Ok(BlockNode { nodes })
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Context;
    use crate::lex::lex;
    use crate::lex::token::{LiteralToken, test_token_with_offset, TokenKind};
    use crate::parse::node::{BlockNode, InfixNode, InfixOperator, LiteralBooleanNode, LiteralNode, TupleNode};
    use crate::parse::node::Node::{Block, Literal};
    use crate::parse::parse;

    #[test]
    fn empty_block() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "{}").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = result[0].as_block();
        assert_eq!(block.nodes, vec![]);
    }


    #[test]
    fn empty_lambda() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "{ () -> }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = result.nodes[0].as_block();
        let InfixNode { left, operator, right } = &block.nodes[0].as_infix();
        let TupleNode { nodes, .. } = left.as_tuple();
        assert_eq!(*nodes, vec![]);

        let InfixOperator::Arrow(_) = operator else { panic!() };

        let block = right.as_block();
        assert_eq!(block.nodes, vec![]);
    }

    #[test]
    fn lambda_with_single_argument() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "{ (arg_1) -> true }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = result.nodes[0].as_block();
        let InfixNode { left, operator, right } = &block.nodes[0].as_infix();
        let TupleNode { nodes, .. } = left.as_tuple();
        assert_eq!(nodes.len(), 1);

        let arg_1 = nodes[0].as_identifier();
        assert_eq!(ctx.get_str(arg_1.value()), "arg_1");

        let InfixOperator::Arrow(_) = operator else { panic!() };

        let block = right.as_block();
        assert_eq!(block.nodes.len(), 1);

        let Literal(LiteralNode::Boolean(boolean_node)) = &block.nodes[0] else { panic!() };
        assert!(boolean_node.value())
    }


    #[test]
    fn block_with_white_spaces() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "{    \t     }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = &result[0].as_block();
        assert_eq!(block.nodes, vec![]);
    }

    #[test]
    fn block_with_new_lines() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, r#"{


        }"#).unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = &result[0].as_block();
        assert_eq!(block.nodes, vec![]);
    }

    #[test]
    fn block_nested() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, r#"{
        {      }
        }"#).unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = &result[0].as_block();
        assert_eq!(block.nodes, vec![Block(BlockNode { nodes: vec![] })]);
    }

    #[test]
    fn block_with_comments() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, r#"{
        // before
        {}
        // after
        }"#).unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = &result[0].as_block();
        assert_eq!(block.nodes, vec![Block(BlockNode { nodes: vec![] })]);
    }

    #[test]
    fn block_multilayer_nested() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, r#"{{   {  true }   }}"#).unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = result[0].as_block();
        assert_eq!(block.nodes, vec![
            Block(BlockNode {
                nodes: vec![
                    Block(BlockNode {
                        nodes: vec![
                            Literal(LiteralNode::Boolean(
                                LiteralBooleanNode(test_token_with_offset(&mut ctx, TokenKind::Literal(LiteralToken::True), "true", 8)))
                            )
                        ]
                    })
                ]
            })
        ]);
    }
}