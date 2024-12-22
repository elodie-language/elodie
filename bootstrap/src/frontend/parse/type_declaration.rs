use crate::frontend::lex::token::KeywordToken::Type;
use crate::frontend::modifier::Modifiers;
use crate::frontend::parse::{Parser, TypeDeclarationNode};

impl<'a> Parser<'a> {
    pub(crate) fn parse_type_declaration(
        &mut self,
    ) -> crate::frontend::parse::Result<TypeDeclarationNode> {
        self.parse_type_declaration_with_modifiers(Modifiers(vec![]))
    }

    pub(crate) fn parse_type_declaration_with_modifiers(
        &mut self,
        modifiers: Modifiers,
    ) -> crate::frontend::parse::Result<TypeDeclarationNode> {
        let token = self.consume_keyword(Type)?;
        let identifier = self.parse_type_identifier()?;
        let properties = self.parse_tuple()?;
        Ok(TypeDeclarationNode {
            token,
            identifier,
            properties,
            modifiers,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::common::context::Context;
    use crate::frontend::lex::lex;
    use crate::frontend::parse::{parse, Error, InfixOperator, TypeNode};

    #[test]
    fn parse_empty_type_declaration() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "type New_Type()").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let decl = result[0].as_type_declaration();
        assert_eq!(ctx.get_str(decl.identifier.value()), "New_Type");
        assert_eq!(decl.properties.nodes, vec![]);
        assert!(!decl.modifiers.is_exported());
    }

    #[test]
    fn parse_invalid_type_declaration() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "type new_type()").unwrap();
        let result = parse(&mut ctx, tokens);
        assert!(result.is_err());
        let Error::InvalidIdentifier { .. } = result.err().unwrap() else {
            panic!()
        };
    }

    #[test]
    fn parse_type_declaration_with_single_property() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "type New_Type(p_1: Number)").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let decl = result[0].as_type_declaration();
        assert_eq!(ctx.get_str(decl.identifier.value()), "New_Type");
        assert_eq!(decl.properties.nodes.len(), 1);

        let prop = decl.properties.nodes.get(0).unwrap().as_infix();
        let prop_ident = prop.left.as_identifier();
        assert_eq!(ctx.get_str(prop_ident.value()), "p_1");
        assert!(matches!(prop.operator, InfixOperator::TypeAscription(_)));
        assert!(matches!(prop.right.as_type(), TypeNode::Number(_)));

        assert!(!decl.modifiers.is_exported());
    }

    #[test]
    fn parse_type_declaration_with_multiple_properties() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "export type New_Type(p_1: Number, p_2: Bool)").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let decl = result[0].as_type_declaration();
        assert_eq!(ctx.get_str(decl.identifier.value()), "New_Type");
        assert_eq!(decl.properties.nodes.len(), 2);

        let prop = decl.properties.nodes.get(0).unwrap().as_infix();
        let prop_ident = prop.left.as_identifier();
        assert_eq!(ctx.get_str(prop_ident.value()), "p_1");
        assert!(matches!(prop.operator, InfixOperator::TypeAscription(_)));
        assert!(matches!(prop.right.as_type(), TypeNode::Number(_)));

        let prop = decl.properties.nodes.get(1).unwrap().as_infix();
        let prop_ident = prop.left.as_identifier();
        assert_eq!(ctx.get_str(prop_ident.value()), "p_2");
        assert!(matches!(prop.operator, InfixOperator::TypeAscription(_)));
        assert!(matches!(prop.right.as_type(), TypeNode::Boolean(_)));

        assert!(decl.modifiers.is_exported());
    }

    #[test]
    fn parse_multiline_type_declaration() {
        let mut ctx = Context::new();
        let tokens = lex(
            &mut ctx,
            r#"export type New_Type(
            p_1: Number,
            p_2: Bool
        )"#,
        )
        .unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let decl = result[0].as_type_declaration();
        assert_eq!(ctx.get_str(decl.identifier.value()), "New_Type");
        assert_eq!(decl.properties.nodes.len(), 2);

        let prop = decl.properties.nodes.get(0).unwrap().as_infix();
        let prop_ident = prop.left.as_identifier();
        assert_eq!(ctx.get_str(prop_ident.value()), "p_1");
        assert!(matches!(prop.operator, InfixOperator::TypeAscription(_)));
        assert!(matches!(prop.right.as_type(), TypeNode::Number(_)));

        let prop = decl.properties.nodes.get(1).unwrap().as_infix();
        let prop_ident = prop.left.as_identifier();
        assert_eq!(ctx.get_str(prop_ident.value()), "p_2");
        assert!(matches!(prop.operator, InfixOperator::TypeAscription(_)));
        assert!(matches!(prop.right.as_type(), TypeNode::Boolean(_)));

        assert!(decl.modifiers.is_exported());
    }
}
