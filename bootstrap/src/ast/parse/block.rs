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

        let mut nodes = Vec::new();
        loop {
            self.consume_if(Separator(NewLine))?;
            if self.current()?.is_operator(CloseCurly) {
                break;
            }
            nodes.push(self.parse_node(Precedence::None)?);
        }
        self.consume_operator(CloseCurly)?;
        Ok(BlockNode { nodes })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::lex::lex;
    use crate::ast::parse::node::{BlockNode, LiteralBooleanNode, LiteralNode};
    use crate::ast::parse::node::Node::{Block, Literal};
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