use crate::ast::{FunctionDeclarationExpression, ParameterExpression, ReturnExpression};
use crate::core::token::{Keyword, Operator, Separator, TokenKind};
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn function_declaration(&mut self) -> crate::parser::Result<FunctionDeclarationExpression> {
        self.previous_expect(TokenKind::Keyword(Keyword::Function))?;
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Operator(Operator::OpenParen))?;

        let mut parameters = vec![];
        loop {
            if self.current_token_kind()? == &TokenKind::Operator(Operator::CloseParen) {
                break;
            }
            parameters.push(self.parameter_expression()?);
            self.consume_if(TokenKind::Separator(Separator::Comma))?;
        }

        self.consume(TokenKind::Operator(Operator::CloseParen))?;

        let return_type = if self.current_token_kind()? == &TokenKind::Operator(Operator::Colon) {
            self.consume(TokenKind::Operator(Operator::Colon))?;
            Some(self.parse_type_expression()?)
        } else {
            None
        };

        self.consume(TokenKind::Operator(Operator::OpenCurly))?;
        let body = self.parse_block_expression()?;

        Ok(
            FunctionDeclarationExpression {
                name: Some(name),
                parameters,
                return_type,
                body,
            }
        )
    }

    fn parameter_expression(&mut self) -> crate::parser::Result<ParameterExpression> {
        let name = self.parse_identifier()?;
        let r#type = if self.current_token_kind()? == &TokenKind::Operator(Operator::Colon) {
            self.consume(TokenKind::Operator(Operator::Colon))?;
            Some(self.parse_type_expression()?)
        } else {
            None
        };

        Ok(ParameterExpression {
            name,
            r#type,
        })
    }

    pub(crate) fn return_expression(&mut self) -> crate::parser::Result<ReturnExpression> {
        self.previous_expect(TokenKind::Keyword(Keyword::Return))?;
        let current = self.current_token_kind()?;

        let expr = if current != &TokenKind::Separator(Separator::NewLine) && current != &TokenKind::EOF {
            Some(Box::new(self.parse_expression(Precedence::None)?))
        } else {
            None
        };

        Ok(ReturnExpression {
            label: None,
            expr,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::ast::{BlockExpression, Expression, FunctionDeclarationExpression, IdentifierExpression, ParameterExpression, ReturnExpression, Statement, TypeExpression};
    use crate::ast::Expression::Literal;
    use crate::ast::Literal::Boolean;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn parse_return_unit() {
        let tokens = Lexer::lex("return").unwrap();
        let mut result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);

        let stmt = result.block.statements.pop().unwrap();

        if let Statement::Expression(Expression::Return(ReturnExpression { expr, .. })) = stmt {
            assert!(expr.is_none())
        } else {
            panic!("Expected single statement with return expression, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_return_boolean() {
        let tokens = Lexer::lex("return true").unwrap();
        let mut result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);

        let stmt = result.block.statements.pop().unwrap();

        if let Statement::Expression(Expression::Return(ReturnExpression { expr, .. })) = stmt {
            assert_eq!(expr.unwrap().deref(), &Expression::Literal(Boolean(true)));
        } else {
            panic!("Expected single statement with return expression, got {:?}", stmt)
        }
    }


    #[test]
    fn parse_function_without_args_and_result_type() {
        let tokens = Lexer::lex("function test(){}").unwrap();
        let mut result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);

        let stmt = result.block.statements.pop().unwrap();

        if let Statement::Expression(Expression::FunctionDeclaration(FunctionDeclarationExpression {
                                                                         name,
                                                                         parameters,
                                                                         return_type,
                                                                         body
                                                                     })) = stmt {
            assert_eq!(name.unwrap(), IdentifierExpression("test".to_string()));
            assert_eq!(parameters, vec![]);
            assert_eq!(return_type, None);
            assert_eq!(body, BlockExpression { body: vec![] })
        } else {
            panic!("Expected single statement with function declaration, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_function_with_result_type() {
        let tokens = Lexer::lex("function test() : Bool {}").unwrap();
        let mut result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);

        let stmt = result.block.statements.pop().unwrap();

        if let Statement::Expression(Expression::FunctionDeclaration(FunctionDeclarationExpression {
                                                                         name,
                                                                         parameters,
                                                                         return_type,
                                                                         body
                                                                     })) = stmt {
            assert_eq!(name.unwrap(), IdentifierExpression("test".to_string()));
            assert_eq!(parameters, vec![]);
            assert_eq!(return_type, Some(TypeExpression::Fundamentals("Bool".to_string())));
            assert_eq!(body, BlockExpression { body: vec![] })
        } else {
            panic!("Expected single statement with function declaration, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_function_with_single_arg_without_type() {
        let tokens = Lexer::lex("function test(arg_1){}").unwrap();
        let mut result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);

        let stmt = result.block.statements.pop().unwrap();

        if let Statement::Expression(Expression::FunctionDeclaration(FunctionDeclarationExpression {
                                                                         name,
                                                                         parameters,
                                                                         return_type,
                                                                         body
                                                                     })) = stmt {
            assert_eq!(name.unwrap(), IdentifierExpression("test".to_string()));
            assert_eq!(parameters, vec![
                ParameterExpression {
                    name: IdentifierExpression("arg_1".to_string()),
                    r#type: None,
                }
            ]);
            assert_eq!(return_type, None);
            assert_eq!(body, BlockExpression { body: vec![] })
        } else {
            panic!("Expected single statement with function declaration, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_function_with_single_arg_with_type() {
        let tokens = Lexer::lex("function test(arg_1: String){}").unwrap();
        let mut result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);

        let stmt = result.block.statements.pop().unwrap();

        if let Statement::Expression(Expression::FunctionDeclaration(FunctionDeclarationExpression {
                                                                         name,
                                                                         parameters,
                                                                         return_type,
                                                                         body
                                                                     })) = stmt {
            assert_eq!(name.unwrap(), IdentifierExpression("test".to_string()));
            assert_eq!(parameters, vec![
                ParameterExpression {
                    name: IdentifierExpression("arg_1".to_string()),
                    r#type: Some(TypeExpression::Fundamentals("String".to_string())),
                }
            ]);
            assert_eq!(return_type, None);
            assert_eq!(body, BlockExpression { body: vec![] })
        } else {
            panic!("Expected single statement with function declaration, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_function_with_multiple_args() {
        let tokens = Lexer::lex("function test(arg_1: String, arg_2: Bool) : String {}").unwrap();
        let mut result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);

        let stmt = result.block.statements.pop().unwrap();

        if let Statement::Expression(Expression::FunctionDeclaration(FunctionDeclarationExpression {
                                                                         name,
                                                                         parameters,
                                                                         return_type,
                                                                         body
                                                                     })) = stmt {
            assert_eq!(name.unwrap(), IdentifierExpression("test".to_string()));
            assert_eq!(parameters, vec![
                ParameterExpression {
                    name: IdentifierExpression("arg_1".to_string()),
                    r#type: Some(TypeExpression::Fundamentals("String".to_string())),
                },
                ParameterExpression {
                    name: IdentifierExpression("arg_2".to_string()),
                    r#type: Some(TypeExpression::Fundamentals("Bool".to_string())),
                },
            ]);
            assert_eq!(return_type, Some(TypeExpression::Fundamentals("String".to_string())));
            assert_eq!(body, BlockExpression { body: vec![] })
        } else {
            panic!("Expected single statement with function declaration, got {:?}", stmt)
        }
    }
}