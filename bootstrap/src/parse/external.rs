use KeywordToken::{External, Function};

use crate::ir::Modifiers;
use crate::lex::token::KeywordToken;
use crate::lex::token::OperatorToken::{Arrow, CloseParen, OpenParen};
use crate::lex::token::SeparatorToken::Comma;
use crate::lex::token::TokenKind::{Operator, Separator};
use crate::parse::{ExternalFunctionDeclarationNode, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_external(&mut self) -> crate::parse::Result<ExternalFunctionDeclarationNode> {
        self.parse_external_with_modifiers(Modifiers(vec![]))
    }

    pub(crate) fn parse_external_with_modifiers(&mut self, modifiers: Modifiers) -> crate::parse::Result<ExternalFunctionDeclarationNode> {
        let external = self.consume_keyword(External)?;
        let _ = self.consume_keyword(Function)?;
        let identifier = self.parse_identifier()?;
        self.consume_operator(OpenParen)?;

        let mut arguments = vec![];
        loop {
            if self.current()?.is_operator(CloseParen) {
                self.consume_operator(CloseParen)?;
                break;
            }
            arguments.push(self.parse_function_declaration_argument()?);
            self.consume_if(Separator(Comma))?;
        }

        let return_type = if !self.is_eof() && self.current()?.is_operator(Arrow) {
            self.consume(Operator(Arrow))?;
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };


        Ok(ExternalFunctionDeclarationNode {
            token: external,
            identifier,
            arguments,
            return_type,
            modifiers,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::common::Context;
    use crate::lex::lex;
    use crate::parse::{parse, TypeFundamentalNode, TypeNode};

    #[test]
    fn external_function_without_args_and_with_return() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "external function magic()").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_external_function_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert_eq!(node.arguments, vec![]);
        assert_eq!(node.return_type, None);
        assert!(!node.modifiers.is_exported());
    }

    #[test]
    fn external_function_with_single_arg() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "external function magic(arg_1: String)").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_external_function_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert!(!node.modifiers.is_exported());
        assert_eq!(node.arguments.len(), 1);

        let arg = &node.arguments[0];
        assert_eq!(ctx.get_str(arg.identifier.value()), "arg_1");

        let TypeNode::Fundamental(TypeFundamentalNode::String(_)) = arg.as_type() else { panic!("not string") };
        assert_eq!(node.return_type, None);
    }

    #[test]
    fn external_function_with_multiple_args() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "external function magic(arg_1: String, arg_2: Number)").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_external_function_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert!(!node.modifiers.is_exported());
        assert_eq!(node.arguments.len(), 2);

        let arg_1 = &node.arguments[0];
        assert_eq!(ctx.get_str(arg_1.identifier.value()), "arg_1");

        let TypeNode::Fundamental(TypeFundamentalNode::String(_)) = arg_1.as_type() else { panic!("not string") };

        let arg_2 = node.arguments.last().unwrap();
        assert_eq!(ctx.get_str(arg_2.identifier.value()), "arg_2");

        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = arg_2.as_type() else { panic!("not number") };

        assert_eq!(node.return_type, None);
    }

    #[test]
    fn exported_external_function() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "export external function magic(arg_1: String, arg_2: Number) -> Bool").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_external_function_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert!(node.modifiers.is_exported());
        assert_eq!(node.arguments.len(), 2);

        let arg_1 = &node.arguments[0];
        assert_eq!(ctx.get_str(arg_1.identifier.value()), "arg_1");

        let TypeNode::Fundamental(TypeFundamentalNode::String(_)) = arg_1.as_type() else { panic!("not string") };

        let arg_2 = node.arguments.last().unwrap();
        assert_eq!(ctx.get_str(arg_2.identifier.value()), "arg_2");

        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = arg_2.as_type() else { panic!("not number") };

        let type_node = node.return_type.as_deref().unwrap();
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = type_node else { panic!("not bool") };

    }
}