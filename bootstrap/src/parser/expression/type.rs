use crate::ast::{FunctionParameterType, FunctionType, TypeExpression};
use crate::core::token::{Operator, Separator, TokenKind};
use crate::core::token::Keyword::Function;
use crate::core::token::Operator::{Arrow, Colon};
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_type_expression(&mut self) -> crate::parser::Result<TypeExpression> {
        if self.current_token_kind()? == &TokenKind::Keyword(Function) {
            self.consume(TokenKind::Keyword(Function))?;
            self.consume(TokenKind::Operator(Operator::OpenParen))?;

            let mut parameters = vec![];
            loop {
                if self.current_token_kind()? == &TokenKind::Operator(Operator::CloseParen) {
                    break;
                }
                parameters.push(self.function_parameter()?);
                self.consume_if(TokenKind::Separator(Separator::Comma))?;
            }

            self.consume(TokenKind::Operator(Operator::CloseParen))?;

            let return_type = if self.current_token_kind()? == &TokenKind::Operator(Operator::Arrow) {
                self.consume(TokenKind::Operator(Arrow))?;
                Some(Box::new(self.parse_type_expression()?))
            } else {
                None
            };

            return Ok(TypeExpression::Function(FunctionType { parameters, return_type }));
        }

        let type_identifier = self.consume(TokenKind::Identifier)?;
        return Ok(TypeExpression::Fundamentals(type_identifier.span.value.clone()));
    }

    fn function_parameter(&mut self) -> crate::parser::Result<FunctionParameterType> {
        let name = if self.current_token_kind()? == &TokenKind::Identifier {
            Some(self.parse_identifier().map(|i| i.0)?)
        } else {
            None
        };
        let r#type = if self.current_token_kind()? == &TokenKind::Operator(Operator::Colon) {
            self.consume(TokenKind::Operator(Operator::Colon))?;
            Some(Box::new(self.parse_type_expression()?))
        } else {
            None
        };

        // anonymous parameter
        if name.is_none() && r#type.is_none() {
            return Ok(FunctionParameterType {
                name: None,
                r#type: Some(Box::new(self.parse_type_expression()?)),
            });
        };

        return Ok(FunctionParameterType {
            name,
            r#type,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{FunctionParameterType, FunctionType, TypeExpression};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn string_type() {
        let tokens = Lexer::lex("String").unwrap();
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_type_expression().unwrap();
        assert_eq!(result, TypeExpression::Fundamentals("String".to_string()));
    }

    #[test]
    fn implicit_unit_function_without_parameters() {
        let tokens = Lexer::lex("function()").unwrap();
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_type_expression().unwrap();
        assert_eq!(result, TypeExpression::Function(FunctionType { parameters: vec![], return_type: None }));
    }

    #[test]
    fn function_with_single_parameter() {
        let tokens = Lexer::lex("function(arg_1: String)").unwrap();
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_type_expression().unwrap();
        assert_eq!(result, TypeExpression::Function(FunctionType {
            parameters: vec![
                FunctionParameterType { name: Some("arg_1".to_string()), r#type: Some(Box::new(TypeExpression::Fundamentals("String".to_string()))) }
            ],
            return_type: None,
        }));
    }

    #[test]
    fn function_with_multiple_parameter() {
        let tokens = Lexer::lex("function(arg_1: String, arg_2: Bool, arg_3: Number, arg_4)").unwrap();
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_type_expression().unwrap();
        assert_eq!(result, TypeExpression::Function(FunctionType {
            parameters: vec![
                FunctionParameterType { name: Some("arg_1".to_string()), r#type: Some(Box::new(TypeExpression::Fundamentals("String".to_string()))) },
                FunctionParameterType { name: Some("arg_2".to_string()), r#type: Some(Box::new(TypeExpression::Fundamentals("Bool".to_string()))) },
                FunctionParameterType { name: Some("arg_3".to_string()), r#type: Some(Box::new(TypeExpression::Fundamentals("Number".to_string()))) },
                FunctionParameterType { name: Some("arg_4".to_string()), r#type: None },
            ],
            return_type: None,
        }));
    }

    #[test]
    fn function_without_parameter_returning_bool() {
        let tokens = Lexer::lex("function()->Bool").unwrap();
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_type_expression().unwrap();
        assert_eq!(result, TypeExpression::Function(FunctionType { parameters: vec![], return_type: Some(Box::new(TypeExpression::Fundamentals("Bool".to_string()))) }));
    }

    #[test]
    fn function_with_function_parameter() {
        let tokens = Lexer::lex("function(f: function() -> Bool)").unwrap();
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_type_expression().unwrap();
        assert_eq!(result, TypeExpression::Function(FunctionType {
            parameters: vec![
                FunctionParameterType {
                    name: Some("f".to_string()),
                    r#type: Some(Box::new(TypeExpression::Function(FunctionType {
                        parameters: vec![],
                        return_type: Some(Box::new(TypeExpression::Fundamentals("Bool".to_string()))),
                    }))),
                }
            ],
            return_type: None,
        }));
    }

    #[test]
    fn function_with_function_parameter_returning_function() {
        let tokens = Lexer::lex("function(f: function() -> Bool) -> function( function() -> Number) -> String").unwrap();
        let mut parser = Parser::new(&tokens);

        let result = parser.parse_type_expression().unwrap();
        assert_eq!(result, TypeExpression::Function(FunctionType {
            parameters: vec![
                FunctionParameterType {
                    name: Some("f".to_string()),
                    r#type: Some(Box::new(TypeExpression::Function(FunctionType {
                        parameters: vec![],
                        return_type: Some(Box::new(TypeExpression::Fundamentals("Bool".to_string()))),
                    }))),
                }
            ],
            return_type: Some(Box::new(
                TypeExpression::Function(FunctionType {
                    parameters: vec![
                        FunctionParameterType {
                            name: None,
                            r#type: Some(Box::new(
                                TypeExpression::Function(FunctionType { parameters: vec![], return_type: Some(Box::new(TypeExpression::Fundamentals("Number".to_string()))) })
                            )),
                        }
                    ],
                    return_type: Some(Box::new(TypeExpression::Fundamentals("String".to_string()))),
                })
            )),
        }));
    }
}