use KeywordToken::Export;

use crate::ast::parse::{Modifier, Modifiers, Node, Parser};
use crate::ast::token::KeywordToken;
use crate::ast::token::KeywordToken::Function;

impl Parser {
    pub(crate) fn parse_modifier_export(&mut self) -> crate::ast::parse::Result<Node> {
        let token = self.consume_keyword(Export)?;
        let modifier = Modifier::Export(token);

        let current = self.current()?;

        if current.is_keyword(Function) {
            return Ok(Node::FunctionDeclaration(self.parse_function_declaration_with_modifiers(Modifiers(vec![modifier]))?));
        }
        unimplemented!()
    }
}