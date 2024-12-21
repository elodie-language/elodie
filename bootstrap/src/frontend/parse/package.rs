use crate::frontend::lex::token::KeywordToken;
use crate::frontend::modifier::Modifiers;
use crate::frontend::parse::{PackageDeclarationNode, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_package_declaration(
        &mut self,
    ) -> crate::frontend::parse::Result<PackageDeclarationNode> {
        self.parse_package_declaration_with_modifiers(Modifiers(vec![]))
    }

    pub(crate) fn parse_package_declaration_with_modifiers(
        &mut self,
        modifiers: Modifiers,
    ) -> crate::frontend::parse::Result<PackageDeclarationNode> {
        let token = self.consume_keyword(KeywordToken::Package)?;
        let identifier = self.parse_identifier()?;
        let block = self.parse_block()?;

        Ok(PackageDeclarationNode {
            token,
            identifier,
            block,
            modifiers,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::frontend::context::Context;
    use crate::frontend::lex::lex;
    use crate::frontend::parse::parse;

    #[test]
    fn empty_package() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "package magic{ }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_package_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert!(!node.modifiers.is_exported());
    }

    #[test]
    fn package_with_exported_function() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "package magic { export function some_fn() {} }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_package_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert_eq!(node.block.nodes.len(), 1);
        assert!(!node.modifiers.is_exported());

        let fn_decl = unsafe { node.block.nodes.get_unchecked(0) }.as_function_declaration();
        assert_eq!(ctx.get_str(fn_decl.identifier.value()), "some_fn");
        assert!(fn_decl.modifiers.is_exported());
    }

    #[test]
    fn exported_package_with_exported_function() {
        let mut ctx = Context::new();
        let tokens = lex(
            &mut ctx,
            "export package magic{ export function some_fn() {} }",
        )
        .unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_package_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert_eq!(node.block.nodes.len(), 1);
        assert!(node.modifiers.is_exported());

        let fn_decl = unsafe { node.block.nodes.get_unchecked(0) }.as_function_declaration();
        assert_eq!(ctx.get_str(fn_decl.identifier.value()), "some_fn");
        assert!(fn_decl.modifiers.is_exported());
    }
}
