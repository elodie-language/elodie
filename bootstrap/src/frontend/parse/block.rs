use OperatorToken::{CloseCurly, OpenCurly};

use crate::frontend::lex::token::{OperatorToken, Token};
use crate::frontend::parse::node::BlockNode;
use crate::frontend::parse::precedence::Precedence;
use crate::frontend::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_block(&mut self) -> crate::frontend::parse::Result<BlockNode> {
        let token = self.consume_operator(OpenCurly)?;
        let result = self.parse_block_inner(token)?;
        self.consume_operator(CloseCurly)?;
        Ok(result)
    }

    pub(crate) fn parse_block_inner(
        &mut self,
        token: Token,
    ) -> crate::frontend::parse::Result<BlockNode> {
        let mut nodes = Vec::new();
        loop {
            self.skip_new_line()?;
            if self.current()?.is_operator(CloseCurly) {
                break;
            }
            nodes.push(self.parse_node(Precedence::None)?);
        }
        Ok(BlockNode { token, nodes })
    }
}

#[cfg(test)]
mod tests {
    use crate::common::context::Context;
    use crate::frontend::lex::lex;
    use crate::frontend::lex::token::{test_token_with_offset, LiteralToken, TokenKind};
    use crate::frontend::parse::node::Node::Literal;
    use crate::frontend::parse::node::{
        InfixNode, InfixOperator, LiteralBooleanNode, LiteralNode, TupleNode,
    };
    use crate::frontend::parse::parse;

    #[test]
    fn empty_block() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "{}").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = result[0].as_block();
        assert_eq!(block.nodes, vec![]);
    }

    #[test]
    fn empty_lambda() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "{ () -> }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = result[0].as_block();
        let InfixNode {
            left,
            operator,
            right,
            ..
        } = &block.nodes[0].as_infix();
        let TupleNode { nodes, .. } = left.as_tuple();
        assert_eq!(*nodes, vec![]);

        let InfixOperator::Arrow(_) = operator else {
            panic!()
        };

        let block = right.as_block();
        assert_eq!(block.nodes, vec![]);
    }

    #[test]
    fn lambda_with_single_argument() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "{ (arg_1) -> true }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = result[0].as_block();
        let InfixNode {
            left,
            operator,
            right,
            ..
        } = &block.nodes[0].as_infix();
        let TupleNode { nodes, .. } = left.as_tuple();
        assert_eq!(nodes.len(), 1);

        let arg_1 = nodes[0].as_identifier();
        assert_eq!(ctx.str_get(arg_1.value()), "arg_1");

        let InfixOperator::Arrow(_) = operator else {
            panic!()
        };

        let block = right.as_block();
        assert_eq!(block.nodes.len(), 1);

        let Literal(LiteralNode::Boolean(boolean_node)) = &block.nodes[0] else {
            panic!()
        };
        assert!(boolean_node.value())
    }

    #[test]
    fn block_with_white_spaces() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "{    \t     }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = &result[0].as_block();
        assert_eq!(block.nodes, vec![]);
    }

    #[test]
    fn block_with_new_lines() {
        let mut ctx = Context::testing();
        let tokens = lex(
            &mut ctx,
            r#"{


        }"#,
        )
        .unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = &result[0].as_block();
        assert_eq!(block.nodes, vec![]);
    }

    #[test]
    fn block_nested() {
        let mut ctx = Context::testing();
        let tokens = lex(
            &mut ctx,
            r#"{
        {      }
        }"#,
        )
        .unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = result[0].as_block();
        assert_eq!(block.nodes.len(), 1);

        let block = block.nodes[0].as_block();
        assert_eq!(block.nodes.len(), 0);
    }

    #[test]
    fn block_with_comments() {
        let mut ctx = Context::testing();
        let tokens = lex(
            &mut ctx,
            r#"{
        // before
        {}
        // after
        }"#,
        )
        .unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = result[0].as_block();
        assert_eq!(block.nodes.len(), 1);

        let block = block.nodes[0].as_block();
        assert_eq!(block.nodes.len(), 0);
    }

    #[test]
    fn block_multilayer_nested() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, r#"{{   {  true }   }}"#).unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let block = result[0].as_block();
        assert_eq!(block.nodes.len(), 1);

        let block = block.nodes[0].as_block();
        assert_eq!(block.nodes.len(), 1);

        let block = block.nodes[0].as_block();
        assert_eq!(block.nodes.len(), 1);

        assert_eq!(
            block.nodes,
            vec![Literal(LiteralNode::Boolean(LiteralBooleanNode(
                test_token_with_offset(&mut ctx, TokenKind::Literal(LiteralToken::True), "true", 8)
            )))]
        );
    }
}
