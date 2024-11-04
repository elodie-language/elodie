use crate::core::ast;
use crate::core::ast::Statement;
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl Parser<'_> {
    pub(crate) fn parse_statement(&mut self) -> crate::parser::Result<ast::Statement> {
        Ok(Statement::Expression(
            self.parse_expression(Precedence::None)?
        ))
    }
}