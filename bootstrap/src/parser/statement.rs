use crate::core::ast;
use crate::parser::Parser;

impl Parser<'_> {
    pub(crate) fn parse_statement(&self) -> crate::parser::Result<ast::Statement> {
        unimplemented!()
    }
}