use KeywordToken::Export;

use crate::ast::modifier::{Modifier, Modifiers};
use crate::parse::{Node, Parser};
use crate::lex::token::KeywordToken;
use crate::lex::token::KeywordToken::{Function, Package, Type};

impl<'a> Parser<'a> {
    pub(crate) fn parse_export(&mut self) -> crate::parse::Result<Node> {
        let token = self.consume_keyword(Export)?;
        let modifier = Modifier::Export(token.clone());

        let current = self.current()?.clone();

        if current.is_keyword(Function) {
            return Ok(Node::FunctionDeclaration(self.parse_function_declaration_with_modifiers(Modifiers(vec![modifier]))?));
        }

        if current.is_keyword(Package) {
            return Ok(Node::PackageDeclaration(self.parse_package_declaration_with_modifiers(Modifiers(vec![modifier]))?));
        }

        if current.is_keyword(Type) {
            return Ok(Node::TypeDeclaration(self.parse_type_declaration_with_modifiers(Modifiers(vec![modifier]))?));
        }

        unimplemented!();
    }
}