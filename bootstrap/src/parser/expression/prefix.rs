use std::str::FromStr;

use crate::ast;
use crate::ast::{Expression, IdentifierExpression, ParenthesizedExpression, UnaryExpression, UnaryOperator};
use crate::core::token::{Keyword, Literal, Operator, TokenKind};
use crate::parser::Error::UnsupportedToken;
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_prefix_expression(&mut self) -> crate::parser::Result<Expression> {
        let token = self.advance()?;

        let expression = match &token.kind {
            TokenKind::Identifier => {
                let identifier = token.span.value.clone();
                Expression::Identifier(IdentifierExpression(identifier))
            }
            TokenKind::Literal(literal) => {
                match literal {
                    Literal::Number => {
                        let value = f64::from_str(&token.span.value).unwrap();
                        Expression::Literal(ast::Literal::Number(value))
                    }
                    Literal::String => {
                        let value = token.span.value.clone();
                        Expression::Literal(ast::Literal::String(value))
                    }
                    Literal::True => Expression::Literal(ast::Literal::Boolean(true)),
                    Literal::False => Expression::Literal(ast::Literal::Boolean(false))
                }
            }
            TokenKind::Operator(operator) => {
                match operator {
                    Operator::OpenParen => {
                        if self.current_token_kind()? == &TokenKind::Operator(Operator::CloseParen) {
                            self.consume(TokenKind::Operator(Operator::CloseParen))?;
                            Expression::Parenthesized(ParenthesizedExpression { expr: None })
                        } else {
                            let expr = Some(Box::new(self.parse_expression(Precedence::None)?));
                            self.consume(TokenKind::Operator(Operator::CloseParen))?;
                            Expression::Parenthesized(ParenthesizedExpression { expr })
                        }
                    }
                    Operator::Minus => {
                        let right = self.parse_expression(Precedence::Unary)?;
                        Expression::Unary(UnaryExpression {
                            op: UnaryOperator::Minus,
                            expr: Box::new(right),
                        })
                    }
                    Operator::OpenCurly => {
                        // { () => return true }
                        // self.line_has(TokenKind::Operator(Operator::Arrow)) //

                        if self.current_token_kind()? == &TokenKind::Operator(Operator::OpenParen){
                            // parse lambda function
                            todo!()
                        }
                        Expression::Block(self.parse_block_expression()?)
                    }
                    _ => return Err(UnsupportedToken(token.clone())),
                }
            }
            TokenKind::Keyword(keyword) => {
                match keyword {
                    Keyword::Break => self.parse_break_expression()?,
                    Keyword::Continue => self.parse_continue_expression()?,
                    Keyword::Function => Expression::FunctionDeclaration(self.function_declaration()?),
                    Keyword::Let => self.parse_let_expression()?,
                    Keyword::If => self.parse_if_expression()?,
                    Keyword::Loop => self.parse_loop_expression()?,
                    Keyword::Return => Expression::Return(self.return_expression()?),
                    _ => return Err(UnsupportedToken(token.clone()))
                }
            }
            _ => return Err(UnsupportedToken(token.clone()))
        };

        Ok(expression)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinaryExpression, BinaryOperator, Expression, IdentifierExpression, ParameterExpression, ParenthesizedExpression, Statement, TypeExpression};
    use crate::ast::Expression::{Binary, Identifier, Literal, Parameter, Parenthesized};
    use crate::ast::Literal::Number;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn empty_parenthesis() {
        let tokens = Lexer::lex("()").unwrap();
        let mut result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);

        let stmt = result.block.statements.pop().unwrap();

        if let Statement::Expression(Expression::Parenthesized(ParenthesizedExpression { expr })) = stmt {
            assert_eq!(expr, None);
        } else {
            panic!("Expected single statement with parenthesis, got {:?}", stmt)
        }
    }

    #[test]
    fn parenthesis_with_expression() {
        let tokens = Lexer::lex("(1)").unwrap();
        let mut result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);

        let stmt = result.block.statements.pop().unwrap();

        if let Statement::Expression(Expression::Parenthesized(ParenthesizedExpression { expr })) = stmt {
            assert_eq!(*expr.unwrap(), Literal(Number(1.0)));
        } else {
            panic!("Expected single statement with parenthesis, got {:?}", stmt)
        }
    }

    #[test]
    fn parenthesis_nested() {
        let tokens = Lexer::lex("(1 * ( 2 + 3))").unwrap();
        let mut result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);

        let stmt = result.block.statements.pop().unwrap();

        if let Statement::Expression(Expression::Parenthesized(ParenthesizedExpression { expr })) = stmt {
            assert_eq!(*expr.unwrap(), Binary(BinaryExpression {
                left: Box::new(Literal(Number(1.0))),
                operator: BinaryOperator::Multiply,
                right: Box::new(Parenthesized(ParenthesizedExpression {
                    expr: Some(Box::new(Binary(BinaryExpression {
                        left: Box::new(Literal(Number(2.0))),
                        operator: BinaryOperator::Add,
                        right: Box::new(Literal(Number(3.0))),
                    })))
                })),
            }));
        } else {
            panic!("Expected single statement with parenthesis, got {:?}", stmt)
        }
    }

    #[test]
    fn parenthesis_identifier() {
        let tokens = Lexer::lex("(u)").unwrap();
        let mut result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);

        let stmt = result.block.statements.pop().unwrap();

        if let Statement::Expression(Expression::Parenthesized(ParenthesizedExpression { expr })) = stmt {
            assert_eq!(*expr.unwrap(), Identifier(IdentifierExpression("u".to_string())));
        } else {
            panic!("Expected single statement with parenthesis, got {:?}", stmt)
        }
    }

    // #[test]
    // fn parenthesis_multiple_identifier() {
    //     let tokens = Lexer::lex("(u,v)").unwrap();
    //     let mut result = Parser::parse(&tokens).unwrap();
    //     assert_eq!(result.block.statements.len(), 1);
    //
    //     let stmt = result.block.statements.pop().unwrap();
    //
    //     if let Statement::Expression(Expression::Parenthesized(ParenthesizedExpression { expr })) = stmt {
    //         assert_eq!(*expr.unwrap(), Identifier(IdentifierExpression("u".to_string())));
    //     } else {
    //         panic!("Expected single statement with parenthesis, got {:?}", stmt)
    //     }
    // }
    //
    // #[test]
    // fn parenthesis_identifier_with_type() {
    //     let tokens = Lexer::lex("(u: Bool)").unwrap();
    //     let mut result = Parser::parse(&tokens).unwrap();
    //     assert_eq!(result.block.statements.len(), 1);
    //
    //     let stmt = result.block.statements.pop().unwrap();
    //
    //     if let Statement::Expression(Expression::Parenthesized(ParenthesizedExpression { expr })) = stmt {
    //
    //         assert_eq!(*expr.unwrap(), Parameter(ParameterExpression{name: IdentifierExpression("u".to_string()), r#type: Some(TypeExpression::Fundamentals("Bool".to_string()))}));
    //     } else {
    //         panic!("Expected single statement with parenthesis, got {:?}", stmt)
    //     }
    // }
    //
    // #[test]
    // fn parenthesis_multiple_identifier_with_type() {
    //     let tokens = Lexer::lex("(u: Bool, v: String)").unwrap();
    //     let mut result = Parser::parse(&tokens).unwrap();
    //     assert_eq!(result.block.statements.len(), 1);
    //
    //     let stmt = result.block.statements.pop().unwrap();
    //
    //     if let Statement::Expression(Expression::Parenthesized(ParenthesizedExpression { expr })) = stmt {
    //
    //         assert_eq!(*expr.unwrap(), Parameter(ParameterExpression{name: IdentifierExpression("u".to_string()), r#type: Some(TypeExpression::Fundamentals("Bool".to_string()))}));
    //     } else {
    //         panic!("Expected single statement with parenthesis, got {:?}", stmt)
    //     }
    // }
}