use crate::core::ast::Statement;
use crate::core::token::{Separator, TokenKind};
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl Parser<'_> {
    pub(crate) fn parse_statement(&mut self) -> crate::parser::Result<Statement> {
        let result = Statement::Expression(
            self.parse_expression(Precedence::None)?
        );

        self.consume(TokenKind::Separator(Separator::NewLine))?;
        Ok(result)
    }
}