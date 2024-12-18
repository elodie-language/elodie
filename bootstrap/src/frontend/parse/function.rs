use KeywordToken::Return;
use OperatorToken::OpenParen;
use SeparatorToken::Comma;
use TokenKind::{Operator, Separator};

use crate::frontend::lex::token::OperatorToken::{Arrow, CloseParen};
use crate::frontend::lex::token::{KeywordToken, OperatorToken, SeparatorToken, TokenKind};
use crate::frontend::modifier::Modifiers;
use crate::frontend::parse::node::{
    FunctionDeclarationArgumentNode, FunctionDeclarationNode, ReturnNode,
};
use crate::frontend::parse::precedence::Precedence;
use crate::frontend::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_function_declaration(
        &mut self,
    ) -> crate::frontend::parse::Result<FunctionDeclarationNode> {
        self.parse_function_declaration_with_modifiers(Modifiers(vec![]))
    }

    pub(crate) fn parse_function_declaration_with_modifiers(
        &mut self,
        modifiers: Modifiers,
    ) -> crate::frontend::parse::Result<FunctionDeclarationNode> {
        let fun_token = self.consume_keyword(KeywordToken::Function)?;
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

        let block = self.parse_block()?;

        Ok(FunctionDeclarationNode {
            token: fun_token,
            identifier,
            arguments,
            return_type,
            block,
            modifiers,
        })
    }

    pub(crate) fn parse_function_declaration_argument(
        &mut self,
    ) -> crate::frontend::parse::Result<FunctionDeclarationArgumentNode> {
        let identifier = self.parse_identifier()?;
        let r#type = if self.current()?.is_operator(OperatorToken::Colon) {
            self.advance()?;
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        Ok(FunctionDeclarationArgumentNode { identifier, r#type })
    }

    pub(crate) fn parse_return(&mut self) -> crate::frontend::parse::Result<ReturnNode> {
        let token = self.consume_keyword(Return)?;
        let result = if !self.is_eof() && !self.current()?.is_separator(SeparatorToken::NewLine) {
            Some(Box::new(self.parse_node(Precedence::None)?))
        } else {
            None
        };

        Ok(ReturnNode { token, result })
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Context;
    use crate::frontend::lex::lex;
    use crate::frontend::parse::node::Node::Literal;
    use crate::frontend::parse::node::{LiteralNode, TypeNode};
    use crate::frontend::parse::parse;

    #[test]
    fn return_without_result() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "return").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = &result[0].as_return();
        assert_eq!(node.result, None);
    }

    #[test]
    fn return_with_result() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "return 9924").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_return().as_result();
        let Literal(LiteralNode::Number(node)) = node else {
            panic!()
        };
        assert_eq!(ctx.get_str(node.value()), "9924");
    }

    #[test]
    fn export_function_without_args_and_without_return() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "export function magic(){ }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_function_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert!(node.modifiers.is_exported());
        assert_eq!(node.block.nodes, vec![]);
        assert_eq!(node.arguments, vec![]);
        assert_eq!(node.return_type, None);
    }

    #[test]
    fn function_without_args_and_without_return() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "function magic(){ }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_function_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert_eq!(node.arguments, vec![]);
        assert_eq!(node.return_type, None);
        assert!(!node.modifiers.is_exported());
    }

    #[test]
    fn function_without_args_and_with_return() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "function magic() -> Bool { }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_function_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert_eq!(node.arguments, vec![]);
        assert!(!node.modifiers.is_exported());

        let type_node = node.as_return_type();
        let TypeNode::Boolean(_) = type_node else {
            panic!("not bool")
        };
    }

    #[test]
    fn function_with_single_arg() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "function magic(arg_1: String){ }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_function_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert!(!node.modifiers.is_exported());
        assert_eq!(node.arguments.len(), 1);

        let arg = &node.arguments[0];
        assert_eq!(ctx.get_str(arg.identifier.value()), "arg_1");

        let TypeNode::String(_) = arg.as_type() else {
            panic!("not string")
        };
        assert_eq!(node.return_type, None);
    }

    #[test]
    fn function_with_multiple_args() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "function magic(arg_1: String, arg_2: Number){ }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_function_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert!(!node.modifiers.is_exported());
        assert_eq!(node.arguments.len(), 2);

        let arg_1 = &node.arguments[0];
        assert_eq!(ctx.get_str(arg_1.identifier.value()), "arg_1");

        let TypeNode::String(_) = arg_1.as_type() else {
            panic!("not string")
        };

        let arg_2 = node.arguments.last().unwrap();
        assert_eq!(ctx.get_str(arg_2.identifier.value()), "arg_2");

        let TypeNode::Number(_) = arg_2.as_type() else {
            panic!("not number")
        };

        assert_eq!(node.return_type, None);
    }

    #[test]
    fn function_with_function_arg() {
        let mut ctx = Context::new();
        let tokens = lex(&mut ctx, "function magic(test_case: function() -> Bool){ }").unwrap();
        let result = parse(&mut ctx, tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result[0].as_function_declaration();
        assert_eq!(ctx.get_str(node.identifier.value()), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert!(!node.modifiers.is_exported());
        assert_eq!(node.arguments.len(), 1);
        assert_eq!(node.return_type, None);

        let arg_1 = &node.arguments[0];
        assert_eq!(ctx.get_str(arg_1.identifier.value()), "test_case");
        let TypeNode::Function(function_node) = arg_1.as_type() else {
            panic!("not function")
        };
        assert_eq!(function_node.arguments, vec![]);

        let TypeNode::Boolean(_) = function_node.as_return_type() else {
            panic!("not bool")
        };
    }
}
