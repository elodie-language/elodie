use KeywordToken::Return;
use OperatorToken::OpenParen;
use SeparatorToken::Comma;
use TokenKind::{Operator, Separator};

use crate::ast::modifier::Modifiers;
use crate::parse::node::{FunctionDeclarationArgumentNode, FunctionDeclarationNode, ReturnNode};
use crate::parse::Parser;
use crate::parse::precedence::Precedence;
use crate::lex::token::{KeywordToken, OperatorToken, SeparatorToken, TokenKind};
use crate::lex::token::OperatorToken::{Arrow, CloseParen};

impl Parser {
    pub(crate) fn parse_function_declaration(&mut self) -> crate::parse::Result<FunctionDeclarationNode> {
        self.parse_function_declaration_with_modifiers(Modifiers(vec![]))
    }

    pub(crate) fn parse_function_declaration_with_modifiers(&mut self, modifiers: Modifiers) -> crate::parse::Result<FunctionDeclarationNode> {
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

    pub(crate) fn parse_function_declaration_argument(&mut self) -> crate::parse::Result<FunctionDeclarationArgumentNode> {
        let identifier = self.parse_identifier()?;
        let r#type = if self.current()?.is_operator(OperatorToken::Colon) {
            self.advance()?;
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        Ok(FunctionDeclarationArgumentNode { identifier, r#type })
    }

    pub(crate) fn parse_return(&mut self) -> crate::parse::Result<ReturnNode> {
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
    use crate::parse::node::{LiteralNode, TypeFundamentalNode, TypeNode};
    use crate::parse::node::Node::Literal;
    use crate::parse::parse;
    use crate::lex::lex;

    #[test]
    fn return_without_result() {
        let tokens = lex("return").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = &result.nodes[0].as_return();
        assert_eq!(node.result, None);
    }

    #[test]
    fn return_with_result() {
        let tokens = lex("return 9924").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_return().as_result();
        let Literal(LiteralNode::Number(node)) = node else { panic!() };
        assert_eq!(node.value().unwrap(), 9924.0);
    }

    #[test]
    fn export_function_without_args_and_without_return() {
        let tokens = lex("export fun magic(){ }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_function_declaration();
        assert_eq!(node.identifier.value(), "magic");
        assert!(node.modifiers.is_exported());
        assert_eq!(node.block.nodes, vec![]);
        assert_eq!(node.arguments, vec![]);
        assert_eq!(node.return_type, None);
    }

    #[test]
    fn function_without_args_and_without_return() {
        let tokens = lex("fun magic(){ }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_function_declaration();
        assert_eq!(node.identifier.value(), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert_eq!(node.arguments, vec![]);
        assert_eq!(node.return_type, None);
        assert!(!node.modifiers.is_exported());
    }

    #[test]
    fn function_without_args_and_with_return() {
        let tokens = lex("fun magic() -> Bool { }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_function_declaration();
        assert_eq!(node.identifier.value(), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert_eq!(node.arguments, vec![]);
        assert!(!node.modifiers.is_exported());

        let type_node = node.as_return_type();
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = type_node else { panic!("not bool") };
    }


    #[test]
    fn function_with_single_arg() {
        let tokens = lex("fun magic(arg_1: String){ }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_function_declaration();
        assert_eq!(node.identifier.value(), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert!(!node.modifiers.is_exported());
        assert_eq!(node.arguments.len(), 1);

        let arg = &node.arguments[0];
        assert_eq!(arg.identifier.value(), "arg_1");

        let TypeNode::Fundamental(TypeFundamentalNode::String(_)) = arg.as_type() else { panic!("not string") };
        assert_eq!(node.return_type, None);
    }

    #[test]
    fn function_with_multiple_args() {
        let tokens = lex("fun magic(arg_1: String, arg_2: Number){ }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_function_declaration();
        assert_eq!(node.identifier.value(), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert!(!node.modifiers.is_exported());
        assert_eq!(node.arguments.len(), 2);

        let arg_1 = &node.arguments[0];
        assert_eq!(arg_1.identifier.value(), "arg_1");

        let TypeNode::Fundamental(TypeFundamentalNode::String(_)) = arg_1.as_type() else { panic!("not string") };

        let arg_2 = node.arguments.last().unwrap();
        assert_eq!(arg_2.identifier.value(), "arg_2");

        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = arg_2.as_type() else { panic!("not number") };

        assert_eq!(node.return_type, None);
    }

    #[test]
    fn function_with_function_arg() {
        let tokens = lex("fun magic(test_case: fun() -> Bool){ }").unwrap();
        let result = parse(tokens).unwrap();
        assert_eq!(result.len(), 1);

        let node = result.nodes[0].as_function_declaration();
        assert_eq!(node.identifier.value(), "magic");
        assert_eq!(node.block.nodes, vec![]);
        assert!(!node.modifiers.is_exported());
        assert_eq!(node.arguments.len(), 1);
        assert_eq!(node.return_type, None);


        let arg_1 = &node.arguments[0];
        assert_eq!(arg_1.identifier.value(), "test_case");
        let TypeNode::Function(function_node) = arg_1.as_type() else { panic!("not function") };
        assert_eq!(function_node.arguments, vec![]);

        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = function_node.as_return_type() else { panic!("not bool") };
    }
}