use std::rc::Rc;

use KeywordToken::Let;

use crate::frontend::lex::token::{KeywordToken, OperatorToken};
use crate::frontend::parse::node::VariableDeclarationNode;
use crate::frontend::parse::precedence::Precedence;
use crate::frontend::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_variable_declaration(
        &mut self,
    ) -> crate::frontend::parse::Result<VariableDeclarationNode> {
        let token = self.consume_keyword(Let)?;
        let identifier = self.parse_identifier()?;

        let r#type = if self.current()?.is_operator(OperatorToken::Colon) {
            self.advance()?;
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume_operator(OperatorToken::Equal)?;
        let value = Rc::new(self.parse_node(Precedence::None)?);

        Ok(VariableDeclarationNode {
            token,
            identifier,
            node: value,
            r#type,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::common::context::Context;
    use crate::frontend::lex::lex;
    use crate::frontend::parse::node::Node::Literal;
    use crate::frontend::parse::node::{LiteralNode, TypeNode};
    use crate::frontend::parse::parse;

    #[test]
    fn let_without_type_string() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "let value = 'Elodie'").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_declare_variable();
        assert_eq!(ctx.str_get(node.identifier.value()), "value");

        assert_eq!(node.r#type, None);

        let Literal(LiteralNode::String(result)) = &node.node.deref() else {
            panic!()
        };
        assert_eq!(ctx.str_get(result.value()), "Elodie");
    }

    #[test]
    fn let_with_type_string() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "let value : String = 'Elodie'").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_declare_variable();
        assert_eq!(ctx.str_get(node.identifier.value()), "value");

        let Some(TypeNode::String(_)) = node.r#type else {
            panic!()
        };

        let Literal(LiteralNode::String(result)) = &node.node.deref() else {
            panic!()
        };
        assert_eq!(ctx.str_get(result.value()), "Elodie");
    }

    #[test]
    fn let_without_type_number() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "let value = 9924").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_declare_variable();
        assert_eq!(ctx.str_get(node.identifier.value()), "value");

        assert_eq!(node.r#type, None);

        let Literal(LiteralNode::Number(result)) = &node.node.deref() else {
            panic!()
        };
        assert_eq!(ctx.str_get(result.value()), "9924");
    }

    #[test]
    fn let_without_type_boolean() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "let value = false").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = &result[0].as_declare_variable();
        assert_eq!(ctx.str_get(node.identifier.value()), "value");
        assert_eq!(node.r#type, None);

        let Literal(LiteralNode::Boolean(result)) = &node.node.deref() else {
            panic!()
        };
        assert_eq!(result.value(), false);
    }
}
