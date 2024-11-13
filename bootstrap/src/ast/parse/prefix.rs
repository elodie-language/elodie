use crate::ast::parse::{Error, Parser};
use crate::ast::parse::node::Node;
use crate::ast::token::LiteralToken::{False, Number, String, True};

impl Parser {
    pub(crate) fn parse_prefix(&mut self) -> crate::ast::parse::Result<Node> {
        let current = self.current()?;
        match current {
            _ if current.is_literal(Number) => Ok(Node::Literal(self.parse_literal_number()?)),
            _ if current.is_literal(True) => Ok(Node::Literal(self.parse_literal_true()?)),
            _ if current.is_literal(False) => Ok(Node::Literal(self.parse_literal_false()?)),
            _ if current.is_literal(String) => Ok(Node::Literal(self.parse_literal_string()?)),
            _ => Err(Error::unsupported(self.advance()?))
        }
    }
}