use std::str::FromStr;

use crate::ast::Expression;
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

mod access;
mod call;
mod identifier;
mod infix;
mod r#let;
mod prefix;

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self, precedence: Precedence) -> crate::parser::Result<Expression> {
        self.skip_whitespace()?;

        let mut left = self.parse_prefix_expression()?;

        while precedence < self.current_precedence()? {
            left = self.parse_infix_expression(left)?;
        }
        Ok(left)
    }

}

