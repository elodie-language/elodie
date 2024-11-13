use crate::new_ast::parse::{Error, Parser};
use crate::new_ast::parse::node::Node;
use crate::new_ast::token::LiteralToken::Number;

impl Parser {
    pub(crate) fn parse_prefix(&mut self) -> crate::new_ast::parse::Result<Node> {
        let current = self.current()?;
        match current {
            _ if current.is_literal(Number) => Ok(Node::Literal(self.parse_literal_number()?)),
            _ => Err(Error::unsupported(self.advance()?))
        }
    }
}