use KeywordToken::Export;

use crate::ir::{Modifier, Modifiers};
use crate::frontend::lex::token::KeywordToken;
use crate::frontend::lex::token::KeywordToken::{Define, External, Function, Package, Type};
use crate::frontend::parse::{Node, Parser};

impl<'a> Parser<'a> {

    pub(crate) fn parse_export(&mut self) -> crate::frontend::parse::Result<Node> {
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

        if current.is_keyword(Define) {
            return Ok(Node::DefineDeclaration(self.parse_define_with_modifiers(Modifiers(vec![modifier]))?));
        }

        if current.is_keyword(External) {
            return Ok(Node::ExternalFunctionDeclaration(self.parse_external_with_modifiers(Modifiers(vec![modifier]))?));
        }

        unimplemented!();
    }
}