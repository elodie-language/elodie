use crate::common::is_pascal_snake_case;
use crate::lex::token::OperatorToken::{Arrow, CloseParen, Colon, OpenParen};
use crate::lex::token::SeparatorToken::Comma;
use crate::lex::token::TokenKind::{Operator, Separator};
use crate::parse::{Parser, TypeCustomNode, TypeFundamentalNode};
use crate::parse::Error::InvalidType;
use crate::parse::node::{TypeFunctionArgumentNode, TypeFunctionNode, TypeNode};

impl<'a> Parser<'a> {
    pub(crate) fn parse_type(&mut self) -> crate::parse::Result<TypeNode> {
        let token = self.advance()?;
        let value = self.ctx.get_str(token.value());
        if !(is_pascal_snake_case(value) || value == "fun") {
            return Err(InvalidType(token));
        }
        match value {
            "Bool" => Ok(TypeNode::Fundamental(TypeFundamentalNode::Boolean(token))),
            "Number" => Ok(TypeNode::Fundamental(TypeFundamentalNode::Number(token))),
            "String" => Ok(TypeNode::Fundamental(TypeFundamentalNode::String(token))),
            "fun" => Ok(TypeNode::Function(self.parse_function_type()?)),
            _ => Ok(TypeNode::Custom(TypeCustomNode { token }))
        }
    }

    pub(crate) fn parse_function_type(&mut self) -> crate::parse::Result<TypeFunctionNode> {
        self.consume_operator(OpenParen)?;

        let mut arguments = vec![];
        loop {
            if self.current()?.is_operator(CloseParen) {
                self.consume_operator(CloseParen)?;
                break;
            }
            arguments.push(self.parse_function_type_argument()?);
            self.consume_if(Separator(Comma))?;
        }

        let return_type = if !self.is_eof() && self.current()?.is_operator(Arrow) {
            self.consume(Operator(Arrow))?;
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        Ok(
            TypeFunctionNode {
                arguments,
                return_type,
            }
        )
    }

    pub(crate) fn parse_function_type_argument(&mut self) -> crate::parse::Result<TypeFunctionArgumentNode> {
        let identifier = if self.peek()?.is_operator(Colon) {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.consume_if(Operator(Colon))?;

        let r#type = Box::new(self.parse_type()?);
        Ok(TypeFunctionArgumentNode { identifier, r#type })
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Context;
    use crate::lex::lex;
    use crate::parse::{Parser, TypeCustomNode};
    use crate::parse::Error::InvalidType;
    use crate::parse::node::{TypeFunctionArgumentNode, TypeFundamentalNode, TypeNode};

    #[test]
    fn not_a_type() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "something_different").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type();
        let Err(InvalidType(_)) = result else { panic!() };
    }

    #[test]
    fn custom_type_point() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "Point").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Custom(TypeCustomNode { token }) = result else { panic!() };
        assert_eq!(ctx.get_str(token.value()), "Point");
    }

    #[test]
    fn type_boolean() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "Bool").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = result else { panic!() };
    }

    #[test]
    fn type_number() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "Number").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = result else { panic!() };
    }

    #[test]
    fn type_string() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "String").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Fundamental(TypeFundamentalNode::String(_)) = result else { panic!() };
    }

    #[test]
    fn type_function_without_args_and_without_result() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "fun()").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else { panic!() };
        assert_eq!(node.arguments, vec![]);
        assert_eq!(node.return_type, None);
    }

    #[test]
    fn type_function_without_args_and_with_result() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "fun() -> Number").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else { panic!() };
        assert_eq!(node.arguments, vec![]);

        let Some(result_node) = node.return_type.as_deref() else { panic!() };
        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = result_node else { panic!() };
    }

    #[test]
    fn type_function_single_named_arg_and_with_result() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "fun(arg_1: Bool) -> Number").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else { panic!() };
        assert_eq!(node.arguments.len(), 1);

        let Some(TypeFunctionArgumentNode { identifier, r#type }) = &node.arguments.first() else { panic!() };
        let Some(identifier) = identifier else { panic!() };
        assert_eq!(ctx.get_str(identifier.value()), "arg_1");

        let arg_type = r#type.as_ref();
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = arg_type else { panic!() };

        let Some(result_node) = node.return_type.as_deref() else { panic!() };
        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = result_node else { panic!() };
    }

    #[test]
    fn type_function_single_arg_and_with_result() {
        let mut ctx = Context::default();
        let tokens = lex(&mut ctx, "fun(Bool) -> Number").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else { panic!() };
        assert_eq!(node.arguments.len(), 1);

        let Some(TypeFunctionArgumentNode { identifier, r#type }) = &node.arguments.first() else { panic!() };
        assert_eq!(*identifier, None);

        let arg_type = r#type.as_ref();
        let TypeNode::Fundamental(TypeFundamentalNode::Boolean(_)) = arg_type else { panic!() };

        let Some(result_node) = node.return_type.as_deref() else { panic!() };
        let TypeNode::Fundamental(TypeFundamentalNode::Number(_)) = result_node else { panic!() };
    }
}