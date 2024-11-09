use std::str::FromStr;

use crate::ast::Expression;
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

mod access;
mod call;
mod function;
mod identifier;
mod infix;
mod r#let;
mod prefix;
mod r#type;
mod r#if;
mod block;
mod r#loop;

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self, precedence: Precedence) -> crate::parser::Result<Expression> {
        let mut left = self.parse_prefix_expression()?;

        while precedence < self.current_precedence()? {
            left = self.parse_infix_expression(left)?;
        }
        Ok(left)
    }

}

