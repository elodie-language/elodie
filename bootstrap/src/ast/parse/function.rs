use KeywordToken::Return;
use OperatorToken::OpenParen;
use SeparatorToken::Comma;
use TokenKind::{Operator, Separator};

use crate::ast::parse::node::{FunctionDeclarationArgumentNode, FunctionDeclarationNode, ReturnNode};
use crate::ast::parse::Parser;
use crate::ast::parse::precedence::Precedence;
use crate::ast::token::{KeywordToken, OperatorToken, SeparatorToken, TokenKind};
use crate::ast::token::OperatorToken::{Arrow, CloseParen};

impl Parser {

    pub(crate) fn parse_function_declaration(&mut self) -> crate::ast::parse::Result<FunctionDeclarationNode> {
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
        })
    }

    pub(crate) fn parse_function_declaration_argument(&mut self) -> crate::ast::parse::Result<FunctionDeclarationArgumentNode> {
        let identifier = self.parse_identifier()?;
        let r#type = if self.current()?.is_operator(OperatorToken::Colon) {
            self.advance()?;
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        Ok(FunctionDeclarationArgumentNode { identifier, r#type })
    }

    pub(crate) fn parse_return(&mut self) -> crate::ast::parse::Result<ReturnNode> {
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
    use std::ops::Deref;

    use crate::ast::lex::lex;
    use crate::ast::parse::node::{LiteralNode, TypeFundamentalNode, TypeNode};
    use crate::ast::parse::node::Node::{FunctionDeclaration, Literal, Return};
    use crate::ast::parse::parse;

    #[test]
    fn return_without_result() {
        let tokens = lex("return").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Return(node) = &result.nodes[0] else { panic!("not return") };
        assert_eq!(node.result, None);
    }

    #[test]
    fn return_with_result() {
        let tokens = lex("return 9924").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let Return(node) = &result.nodes[0] else { panic!("not return") };
        let Some(ref node) = node.result else { panic!() };

        let Literal(LiteralNode::Number(node)) = &node.deref() else { panic!() };
        assert_eq!(node.value().unwrap(), 9924.0);
    }

    #[test]
    fn function_without_args_and_without_return() {
        let tokens = lex("fun magic(){ }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let FunctionDeclaration(node) = &result.nodes[0] else { panic!("not return") };
        assert_eq!(node.identifier.identifier(), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert_eq!(node.arguments, vec![]);
        assert_eq!(node.return_type, None);
    }

    #[test]
    fn function_without_args_and_with_return() {
        let tokens = lex("fun magic() -> Bool { }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let FunctionDeclaration(node) = &result.nodes[0] else { panic!("not return") };
        assert_eq!(node.identifier.identifier(), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert_eq!(node.arguments, vec![]);

        let Some(type_node) = &node.return_type.as_deref() else { panic!("no result") };
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = type_node else { panic!("not bool") };
    }


    #[test]
    fn function_with_single_arg() {
        let tokens = lex("fun magic(arg_1: String){ }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let FunctionDeclaration(node) = &result.nodes[0] else { panic!("not return") };
        assert_eq!(node.identifier.identifier(), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert_eq!(node.arguments.len(), 1);

        let arg = node.arguments.first().unwrap();
        assert_eq!(arg.identifier.identifier(), "arg_1");

        let Some(arg_1_node) = &arg.r#type.as_deref() else { panic!("no arg type") };
        let TypeNode::Fundamental(TypeFundamentalNode::String(_)) = arg_1_node else { panic!("not string") };

        assert_eq!(node.return_type, None);
    }

    #[test]
    fn function_with_multiple_args() {
        let tokens = lex("fun magic(arg_1: String, arg_2: Number){ }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let FunctionDeclaration(node) = &result.nodes[0] else { panic!("not return") };
        assert_eq!(node.identifier.identifier(), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert_eq!(node.arguments.len(), 2);

        let arg_1 = node.arguments.first().unwrap();
        assert_eq!(arg_1.identifier.identifier(), "arg_1");

        let Some(arg_1_node) = &arg_1.r#type.as_deref() else { panic!("no arg type") };
        let TypeNode::Fundamental(TypeFundamentalNode::String(_)) = arg_1_node else { panic!("not string") };

        let arg_2 = node.arguments.last().unwrap();
        assert_eq!(arg_2.identifier.identifier(), "arg_2");

        let Some(arg_1_node) = &arg_2.r#type.as_deref() else { panic!("no arg type") };
        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = arg_1_node else { panic!("not number") };

        assert_eq!(node.return_type, None);
    }

    #[test]
    fn function_with_function_arg() {
        let tokens = lex("fun magic(test_case: fun() -> Bool){ }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let FunctionDeclaration(node) = &result.nodes[0] else { panic!("not return") };
        assert_eq!(node.identifier.identifier(), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert_eq!(node.arguments.len(), 1);
        assert_eq!(node.return_type, None);


        let arg_1 = node.arguments.first().unwrap();
        assert_eq!(arg_1.identifier.identifier(), "test_case");

        let Some(arg_1_node) = &arg_1.r#type.as_deref() else { panic!("no arg type") };

        let TypeNode::Function(function_node) = arg_1_node else { panic!("not function") };
        assert_eq!(function_node.arguments, vec![]);

        let Some(type_node) = &function_node.return_type.as_deref() else { panic!("no result") };
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = type_node else { panic!("not bool") };
    }
}