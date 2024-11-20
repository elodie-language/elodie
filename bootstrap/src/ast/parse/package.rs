use crate::ast::modifier::Modifiers;
use crate::ast::parse::{PackageDeclarationNode, Parser};
use crate::ast::token::KeywordToken;

impl Parser {

    pub(crate) fn parse_package_declaration(&mut self) -> crate::ast::parse::Result<PackageDeclarationNode> {
        self.parse_package_declaration_with_modifiers(Modifiers(vec![]))
    }

    pub(crate) fn parse_package_declaration_with_modifiers(&mut self, modifiers: Modifiers) -> crate::ast::parse::Result<PackageDeclarationNode> {
        let fun_token = self.consume_keyword(KeywordToken::Package)?;
        let identifier = self.parse_identifier()?;
        let block = self.parse_block()?;

        Ok(PackageDeclarationNode {
            token: fun_token,
            identifier,
            block,
            modifiers,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::lex::lex;
    use crate::ast::parse::parse;

    #[test]
    fn empty_package() {
        let tokens = lex("package magic{ }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_package_declaration();
        assert_eq!(node.identifier.value(), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert!(!node.modifiers.is_exported());
    }

    #[test]
    fn package_with_exported_function() {
        let tokens = lex("package magic{ export fun some_fn() {} }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_package_declaration();
        assert_eq!(node.identifier.value(), "magic");
        assert_eq!(node.block.nodes.len(), 1);
        assert!(!node.modifiers.is_exported());

        let fn_decl = unsafe { node.block.nodes.get_unchecked(0) }.as_function_declaration();
        assert_eq!(fn_decl.identifier.value(), "some_fn");
        assert!(fn_decl.modifiers.is_exported());
    }

    #[test]
    fn exported_package_with_exported_function() {
        let tokens = lex("export package magic{ export fun some_fn() {} }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_package_declaration();
        assert_eq!(node.identifier.value(), "magic");
        assert_eq!(node.block.nodes.len(), 1);
        assert!(node.modifiers.is_exported());

        let fn_decl = unsafe { node.block.nodes.get_unchecked(0) }.as_function_declaration();
        assert_eq!(fn_decl.identifier.value(), "some_fn");
        assert!(fn_decl.modifiers.is_exported());
    }
}