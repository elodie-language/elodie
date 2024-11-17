use std::str::FromStr;

use crate::ast::Expression;
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

mod access;
mod block;
mod call;
mod function;
mod identifier;
mod r#if;
mod infix;
mod lambda;
mod r#let;
mod r#loop;
mod prefix;
mod r#type;

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self, precedence: Precedence) -> crate::parser::Result<Expression> {
        let mut left = self.parse_prefix_expression()?;

        while precedence < self.current_precedence()? {
            left = self.parse_infix_expression(left)?;
        }
        Ok(left)
    }

}

