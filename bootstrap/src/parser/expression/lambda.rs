use crate::ast::{Expression, LambdaDeclarationExpression};
use crate::core::token::{Operator, TokenKind};
use crate::core::token::Operator::Arrow;
use crate::parser::Parser;

impl<'a> Parser<'a> {

    pub(crate) fn lambda_declaration(&mut self, left: Expression) -> crate::parser::Result<LambdaDeclarationExpression> {
        self.consume(TokenKind::Operator(Arrow))?;
        println!("{:?}", left);

        self.consume(TokenKind::Operator(Operator::OpenCurly))?;
        let body = self.parse_block_expression()?;

        Ok(
            LambdaDeclarationExpression {
                parameters: vec![],
                return_type: None,
                body,
            }
        )
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::ast::{BlockExpression, Expression, LambdaDeclarationExpression, ReturnExpression, Statement};
//     use crate::ast::Expression::Literal;
//     use crate::ast::Literal::Boolean;
//     use crate::new_ast::lex::Lexer;
//     use crate::parser::Parser;
//
//     #[test]
//     fn lambda_most_simple() {
//         let tokens = Lexer::lex("{ return true }").unwrap();
//         let mut result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.pop().unwrap();
//
//         if let Statement::Expression(Expression::LambdaDeclaration(LambdaDeclarationExpression {
//                                                                        parameters,
//                                                                        return_type,
//                                                                        body
//                                                                    })) = stmt {
//             assert_eq!(parameters, vec![]);
//             assert_eq!(return_type, None);
//             assert_eq!(body, BlockExpression {
//                 body: vec![
//                     Expression::Return(ReturnExpression {
//                         label: None,
//                         expr: Some(Box::new(Literal(Boolean(true)))),
//                     })
//                 ]
//             })
//         } else {
//             panic!("Expected single statement with lambda declaration, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn lambda_without_parameters_and_result_type() {
//         let tokens = Lexer::lex("{ () -> return true }").unwrap();
//         let mut result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.pop().unwrap();
//
//         if let Statement::Expression(Expression::LambdaDeclaration(LambdaDeclarationExpression {
//                                                                        parameters,
//                                                                        return_type,
//                                                                        body
//                                                                    })) = stmt {
//             assert_eq!(parameters, vec![]);
//             assert_eq!(return_type, None);
//             assert_eq!(body, BlockExpression {
//                 body: vec![
//                     Expression::Return(ReturnExpression {
//                         label: None,
//                         expr: Some(Box::new(Literal(Boolean(true)))),
//                     })
//                 ]
//             })
//         } else {
//             panic!("Expected single statement with lambda declaration, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn lambda_with_single_parameter_without_type() {
//         let tokens = Lexer::lex("{ (v) =>  return true }").unwrap();
//         let mut result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.pop().unwrap();
//
//         if let Statement::Expression(Expression::LambdaDeclaration(LambdaDeclarationExpression {
//                                                                        parameters,
//                                                                        return_type,
//                                                                        body
//                                                                    })) = stmt {
//             assert_eq!(parameters, vec![]);
//             assert_eq!(return_type, None);
//             assert_eq!(body, BlockExpression {
//                 body: vec![
//                     Expression::Return(ReturnExpression {
//                         label: None,
//                         expr: Some(Box::new(Literal(Boolean(true)))),
//                     })
//                 ]
//             })
//         } else {
//             panic!("Expected single statement with lambda declaration, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn lambda_with_multiple_parameters_without_type() {
//         let tokens = Lexer::lex("{ (v, w) => return true }").unwrap();
//         let mut result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.pop().unwrap();
//
//         if let Statement::Expression(Expression::LambdaDeclaration(LambdaDeclarationExpression {
//                                                                        parameters,
//                                                                        return_type,
//                                                                        body
//                                                                    })) = stmt {
//             assert_eq!(parameters, vec![]);
//             assert_eq!(return_type, None);
//             assert_eq!(body, BlockExpression {
//                 body: vec![
//                     Expression::Return(ReturnExpression {
//                         label: None,
//                         expr: Some(Box::new(Literal(Boolean(true)))),
//                     })
//                 ]
//             })
//         } else {
//             panic!("Expected single statement with lambda declaration, got {:?}", stmt)
//         }
//     }
// }