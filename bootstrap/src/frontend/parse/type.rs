use crate::common::is_pascal_snake_case;
use crate::frontend::lex::token::OperatorToken::{Arrow, CloseParen, Colon, OpenParen};
use crate::frontend::lex::token::SeparatorToken::Comma;
use crate::frontend::lex::token::TokenKind::{Operator, Separator};
use crate::frontend::parse::Error::InvalidType;
use crate::frontend::parse::node::{TypeFunctionArgumentNode, TypeFunctionNode, TypeNode};
use crate::frontend::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_type(&mut self) -> crate::frontend::parse::Result<TypeNode> {
        let token = self.advance()?;
        let value = self.ctx.str_get(token.value());
        if !(is_pascal_snake_case(value) || value == "function") {
            return Err(InvalidType(token));
        }
        match value {
            "Bool" => Ok(TypeNode::Boolean(token)),
            "Number" => Ok(TypeNode::Number(token)),
            "String" => Ok(TypeNode::String(token)),
            "function" => Ok(TypeNode::Function(self.parse_function_type()?)),
            _ => Ok(TypeNode::Type( token )),
        }
    }

    pub(crate) fn parse_function_type(
        &mut self,
    ) -> crate::frontend::parse::Result<TypeFunctionNode> {
        let token = self.consume_operator(OpenParen)?;

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

        Ok(TypeFunctionNode {
            token,
            arguments,
            return_type,
        })
    }

    pub(crate) fn parse_function_type_argument(
        &mut self,
    ) -> crate::frontend::parse::Result<TypeFunctionArgumentNode> {
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
    use crate::frontend::lex::lex;
    use crate::frontend::parse::{Parser, TypeNode};
    use crate::frontend::parse::Error::InvalidType;
    use crate::frontend::parse::node::{TypeFunctionArgumentNode};

    #[test]
    fn not_a_type() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "something_different").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type();
        let Err(InvalidType(_)) = result else {
            panic!()
        };
    }

    #[test]
    fn object_type_point() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "Point").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Type(token) = result else {
            panic!()
        };
        assert_eq!(ctx.str_get(token.value()), "Point");
    }

    #[test]
    fn type_boolean() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "Bool").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Boolean(_) = result else {
            panic!()
        };
    }

    #[test]
    fn type_number() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "Number").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::Number(_) = result else {
            panic!()
        };
    }

    #[test]
    fn type_string() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "String").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();
        let TypeNode::String(_) = result else {
            panic!()
        };
    }

    #[test]
    fn type_function_without_args_and_without_result() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "function()").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else {
            panic!()
        };
        assert_eq!(node.arguments, vec![]);
        assert_eq!(node.return_type, None);
    }

    #[test]
    fn type_function_without_args_and_with_result() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "function() -> Number").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else {
            panic!()
        };
        assert_eq!(node.arguments, vec![]);

        let Some(result_node) = node.return_type.as_deref() else {
            panic!()
        };
        let TypeNode::Number(_) = result_node else {
            panic!()
        };
    }

    #[test]
    fn type_function_single_named_arg_and_with_result() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "function(arg_1: Bool) -> Number").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else {
            panic!()
        };
        assert_eq!(node.arguments.len(), 1);

        let Some(TypeFunctionArgumentNode { identifier, r#type }) = &node.arguments.first() else {
            panic!()
        };
        let Some(identifier) = identifier else {
            panic!()
        };
        assert_eq!(ctx.str_get(identifier.value()), "arg_1");

        let arg_type = r#type.as_ref();
        let TypeNode::Boolean(_) = arg_type else {
            panic!()
        };

        let Some(result_node) = node.return_type.as_deref() else {
            panic!()
        };
        let TypeNode::Number(_) = result_node else {
            panic!()
        };
    }

    #[test]
    fn type_function_single_arg_and_with_result() {
        let mut ctx = Context::testing();
        let tokens = lex(&mut ctx, "function(Bool) -> Number").unwrap();
        let mut parser = Parser::new(&mut ctx, tokens);
        let result = parser.parse_type().unwrap();

        let TypeNode::Function(node) = result else {
            panic!()
        };
        assert_eq!(node.arguments.len(), 1);

        let Some(TypeFunctionArgumentNode { identifier, r#type }) = &node.arguments.first() else {
            panic!()
        };
        assert_eq!(*identifier, None);

        let arg_type = r#type.as_ref();
        let TypeNode::Boolean(_) = arg_type else {
            panic!()
        };

        let Some(result_node) = node.return_type.as_deref() else {
            panic!()
        };
        let TypeNode::Number(_) = result_node else {
            panic!()
        };
    }
}
