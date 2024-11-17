use OperatorToken::{CloseCurly, OpenCurly};
use TokenKind::Separator;

use crate::ast::parse::node::BlockNode;
use crate::ast::parse::Parser;
use crate::ast::parse::precedence::Precedence;
use crate::ast::token::{OperatorToken, TokenKind};
use crate::ast::token::SeparatorToken::NewLine;

impl Parser {
    pub(crate) fn parse_block(&mut self) -> crate::ast::parse::Result<BlockNode> {
        self.consume_operator(OpenCurly)?;
        let result = self.parse_block_inner()?;
        self.consume_operator(CloseCurly)?;
        Ok(result)
    }

    pub(crate) fn parse_block_inner(&mut self) -> crate::ast::parse::Result<BlockNode> {
        let mut nodes = Vec::new();
        loop {
            self.consume_if(Separator(NewLine))?;
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
    use std::ops::Deref;

    use crate::ast::lex::lex;
    use crate::ast::parse::node::{BlockNode, InfixNode, InfixOperator, LiteralBooleanNode, LiteralNode, TupleNode};
    use crate::ast::parse::node::Node::{Block, Identifier, Infix, Literal, Tuple};
    use crate::ast::parse::parse;
    use crate::ast::token::{LiteralToken, test_token_with_offset, TokenKind};

    #[test]
    fn empty_block() {
        let tokens = lex("{}").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Block(node) = &result[0] else { panic!() };
        assert_eq!(node.nodes, vec![]);
    }


    #[test]
    fn empty_lambda() {
        let tokens = lex("{ () -> }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Some(Block(block)) = result.nodes.first() else { panic!() };
        let Infix(InfixNode { left, operator, right }) = &block.nodes[0] else { panic!() };
        let Tuple(TupleNode { nodes, .. }) = left.deref() else { panic!() };
        assert_eq!(*nodes, vec![]);

        let InfixOperator::Arrow(_) = operator else { panic!() };

        let Block(block) = right.deref() else { panic!() };
        assert_eq!(*block.nodes, vec![]);
    }

    #[test]
    fn lambda_with_single_argument() {
        let tokens = lex("{ (arg_1) -> true }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Some(Block(block)) = result.nodes.first() else { panic!() };
        let Infix(InfixNode { left, operator, right }) = &block.nodes[0] else { panic!() };
        let Tuple(TupleNode { nodes, .. }) = left.deref() else { panic!() };
        assert_eq!(nodes.len(), 1);
        let Identifier(arg_1_node) = nodes.first().unwrap() else { panic!() };
        assert_eq!(arg_1_node.identifier(), "arg_1");

        let InfixOperator::Arrow(_) = operator else { panic!() };

        let Block(block) = right.deref() else { panic!() };
        assert_eq!(block.nodes.len(), 1);

        let Literal(LiteralNode::Boolean(boolean_node)) = block.nodes.first().unwrap() else { panic!() };
        assert!(boolean_node.value())
    }


    #[test]
    fn block_with_white_spaces() {
        let tokens = lex("{    \t     }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Block(node) = &result[0] else { panic!() };
        assert_eq!(node.nodes, vec![]);
    }

    #[test]
    fn block_with_new_lines() {
        let tokens = lex(r#"{


        }"#).unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Block(node) = &result[0] else { panic!() };
        assert_eq!(node.nodes, vec![]);
    }

    #[test]
    fn block_nested() {
        let tokens = lex(r#"{
        {      }
        }"#).unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Block(node) = &result[0] else { panic!() };
        assert_eq!(node.nodes, vec![Block(BlockNode { nodes: vec![] })]);
    }

    #[test]
    fn block_multilayer_nested() {
        let tokens = lex(r#"{{   {  true }   }}"#).unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Block(node) = &result[0] else { panic!() };
        assert_eq!(node.nodes, vec![
            Block(BlockNode {
                nodes: vec![
                    Block(BlockNode {
                        nodes: vec![
                            Literal(LiteralNode::Boolean(
                                LiteralBooleanNode(test_token_with_offset(TokenKind::Literal(LiteralToken::True), "true", 8)))
                            )
                        ]
                    })
                ]
            })
        ]);
    }
}