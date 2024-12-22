use KeywordToken::Define;

use crate::frontend::lex::token::KeywordToken;
use crate::frontend::modifier::Modifiers;
use crate::frontend::parse::{DefineDeclarationNode, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_define(&mut self) -> crate::frontend::parse::Result<DefineDeclarationNode> {
        self.parse_define_with_modifiers(Modifiers(vec![]))
    }

    pub(crate) fn parse_define_with_modifiers(
        &mut self,
        modifiers: Modifiers,
    ) -> crate::frontend::parse::Result<DefineDeclarationNode> {
        let token = self.consume_keyword(Define)?;
        let identifier = self.parse_type_identifier()?;
        let block = self.parse_block()?;

        Ok(DefineDeclarationNode {
            token,
            identifier,
            block,
            modifiers,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::common::context::Context;
    use crate::frontend::lex::lex;
    use crate::frontend::parse::parse;

    #[test]
    fn empty_definition() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "define Magic{ }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_define_declaration();
        assert_eq!(ctx.str_get(node.identifier.value()), "Magic");
        assert_eq!(node.block.nodes, vec![]);
        assert!(!node.modifiers.is_exported());
    }

    #[test]
    fn define_with_fun() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "define Magic { export function some_fn() {} }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_define_declaration();
        assert_eq!(ctx.str_get(node.identifier.value()), "Magic");
        assert_eq!(node.block.nodes.len(), 1);
        assert!(!node.modifiers.is_exported());

        let fn_decl = unsafe { node.block.nodes.get_unchecked(0) }.as_function_declaration();
        assert_eq!(ctx.str_get(fn_decl.identifier.value()), "some_fn");
        assert!(fn_decl.modifiers.is_exported());
    }

    #[test]
    fn exported_definition_with_exported_function() {
        let mut ctx = Context::testing();
        let tokens = lex(
            &mut ctx,
            "export define Magic { export function some_fn() {} }",
        )
        .unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_define_declaration();
        assert_eq!(ctx.str_get(node.identifier.value()), "Magic");
        assert_eq!(node.block.nodes.len(), 1);
        assert!(node.modifiers.is_exported());

        let fn_decl = unsafe { node.block.nodes.get_unchecked(0) }.as_function_declaration();
        assert_eq!(ctx.str_get(fn_decl.identifier.value()), "some_fn");
        assert!(fn_decl.modifiers.is_exported());
    }
}
