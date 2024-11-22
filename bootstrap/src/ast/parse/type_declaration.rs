use crate::ast::modifier::Modifiers;
use crate::ast::parse::{Parser, TypeDeclarationNode};
use crate::ast::token::KeywordToken::Type;

impl Parser {
    pub(crate) fn parse_type_declaration(&mut self) -> crate::ast::parse::Result<TypeDeclarationNode> {
        self.parse_type_declaration_with_modifiers(Modifiers(vec![]))
    }

    pub(crate) fn parse_type_declaration_with_modifiers(&mut self, modifiers: Modifiers) -> crate::ast::parse::Result<TypeDeclarationNode> {
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
    use crate::ast::lex::lex;
    use crate::ast::parse::{Error, InfixOperator, parse, TypeFundamentalNode, TypeNode};

    #[test]
    fn parse_empty_type_declaration() {
        let tokens = lex("type New_Type()").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let decl = result[0].as_type_declaration();
        assert_eq!(decl.identifier.value(), "New_Type");
        assert_eq!(decl.properties.nodes, vec![]);
        assert!(!decl.modifiers.is_exported());
    }

    #[test]
    fn parse_invalid_type_declaration() {
        let tokens = lex("type new_type()").unwrap();
        let result = parse(tokens);
        assert!(result.is_err());
        let Error::InvalidIdentifier { .. } = result.err().unwrap() else { panic!() };
    }

    #[test]
    fn parse_type_declaration_with_single_property() {
        let tokens = lex("type New_Type(p_1: Number)").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let decl = result[0].as_type_declaration();
        assert_eq!(decl.identifier.value(), "New_Type");
        assert_eq!(decl.properties.nodes.len(), 1);

        let prop = decl.properties.nodes.get(0).unwrap().as_infix();
        let prop_ident = prop.left.as_identifier();
        assert_eq!(prop_ident.value(), "p_1");
        assert!(matches!(prop.operator, InfixOperator::TypeAscription(_)));
        assert!(matches!(prop.right.as_type(), TypeNode::Fundamental(TypeFundamentalNode::Number(_))));

        assert!(!decl.modifiers.is_exported());
    }

    #[test]
    fn parse_type_declaration_with_multiple_properties() {
        let tokens = lex("export type New_Type(p_1: Number, p_2: Bool)").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let decl = result[0].as_type_declaration();
        assert_eq!(decl.identifier.value(), "New_Type");
        assert_eq!(decl.properties.nodes.len(), 2);

        let prop = decl.properties.nodes.get(0).unwrap().as_infix();
        let prop_ident = prop.left.as_identifier();
        assert_eq!(prop_ident.value(), "p_1");
        assert!(matches!(prop.operator, InfixOperator::TypeAscription(_)));
        assert!(matches!(prop.right.as_type(), TypeNode::Fundamental(TypeFundamentalNode::Number(_))));

        let prop = decl.properties.nodes.get(1).unwrap().as_infix();
        let prop_ident = prop.left.as_identifier();
        assert_eq!(prop_ident.value(), "p_2");
        assert!(matches!(prop.operator, InfixOperator::TypeAscription(_)));
        assert!(matches!(prop.right.as_type(), TypeNode::Fundamental(TypeFundamentalNode::Boolean(_))));

        assert!(decl.modifiers.is_exported());
    }

    #[test]
    fn parse_multiline_type_declaration() {
        let tokens = lex(r#"export type New_Type(
            p_1: Number,
            p_2: Bool
        )"#).unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let decl = result[0].as_type_declaration();
        assert_eq!(decl.identifier.value(), "New_Type");
        assert_eq!(decl.properties.nodes.len(), 2);

        let prop = decl.properties.nodes.get(0).unwrap().as_infix();
        let prop_ident = prop.left.as_identifier();
        assert_eq!(prop_ident.value(), "p_1");
        assert!(matches!(prop.operator, InfixOperator::TypeAscription(_)));
        assert!(matches!(prop.right.as_type(), TypeNode::Fundamental(TypeFundamentalNode::Number(_))));

        let prop = decl.properties.nodes.get(1).unwrap().as_infix();
        let prop_ident = prop.left.as_identifier();
        assert_eq!(prop_ident.value(), "p_2");
        assert!(matches!(prop.operator, InfixOperator::TypeAscription(_)));
        assert!(matches!(prop.right.as_type(), TypeNode::Fundamental(TypeFundamentalNode::Boolean(_))));

        assert!(decl.modifiers.is_exported());
    }
}