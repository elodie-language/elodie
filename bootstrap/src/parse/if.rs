use KeywordToken::{Else, If};

use crate::parse::node::{ElseNode, IfNode};
use crate::parse::Parser;
use crate::parse::precedence::Precedence;
use crate::lex::token::KeywordToken;

impl Parser {
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

    use crate::parse::node::{IfNode, LiteralNode};
    use crate::parse::node::Node::Literal;
    use crate::parse::parse;
    use crate::lex::lex;

    #[test]
    fn empty_if_no_else() {
        let tokens = lex("if true {}").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode { condition, then, otherwise, .. } = result[0].as_if();

        let Literal(LiteralNode::Boolean(condition)) = condition.deref() else { panic!("not boolean node") };
        assert_eq!(condition.value(), true);
        assert_eq!(then.nodes, vec![]);
        assert_eq!(*otherwise, None);
    }

    #[test]
    fn if_multiple_then_nodes() {
        let tokens = lex(r#"if true {
            99
            24
        }"#).unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode { condition, then, otherwise, .. } = result[0].as_if();

        let Literal(LiteralNode::Boolean(condition)) = condition.deref() else { panic!("not boolean node") };
        assert_eq!(condition.value(), true);
        assert_eq!(then.nodes.len(), 2);

        let Some(Literal(LiteralNode::Number(first))) = then.nodes.first() else { panic!("not a number node") };
        assert_eq!(first.value().unwrap(), 99.0);

        let Some(Literal(LiteralNode::Number(last))) = then.nodes.last() else { panic!("not a number node") };
        assert_eq!(last.value().unwrap(), 24.0);

        assert_eq!(*otherwise, None);
    }

    #[test]
    fn empty_if_and_else() {
        let tokens = lex("if true {} else {}").unwrap();
        let result = parse(tokens).unwrap();
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
        let tokens = lex(r#"if true {
            1
            2
        }else{
            3
            4
        }"#).unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let IfNode { condition, then, otherwise, .. } = result[0].as_if();

        let Literal(LiteralNode::Boolean(condition)) = condition.deref() else { panic!("not boolean node") };
        assert_eq!(condition.value(), true);
        assert_eq!(then.nodes.len(), 2);

        let Some(Literal(LiteralNode::Number(first))) = then.nodes.first() else { panic!("not a number node") };
        assert_eq!(first.value().unwrap(), 1.0);

        let Some(Literal(LiteralNode::Number(last))) = then.nodes.last() else { panic!("not a number node") };
        assert_eq!(last.value().unwrap(), 2.0);

        let Some(otherwise) = otherwise else { panic!("no else node") };
        assert_eq!(otherwise.block.nodes.len(), 2);

        let Some(Literal(LiteralNode::Number(first))) = otherwise.block.nodes.first() else { panic!("not a number node") };
        assert_eq!(first.value().unwrap(), 3.0);

        let Some(Literal(LiteralNode::Number(last))) = otherwise.block.nodes.last() else { panic!("not a number node") };
        assert_eq!(last.value().unwrap(), 4.0);
    }
}