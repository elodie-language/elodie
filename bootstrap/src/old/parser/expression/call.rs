use crate::ast::{CallExpression, CallParameter, Expression};
use crate::core::token::{Operator, Separator, TokenKind};
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_arguments(&mut self) -> crate::parser::Result<Vec<CallParameter>> {
        let mut result = Vec::new();

        self.consume(TokenKind::Operator(Operator::OpenParen))?;

        loop {
            {
                let current = self.current_token()?;
                if current.kind == TokenKind::Operator(Operator::CloseParen) {
                    break;
                }
            }

            result.push(CallParameter {
                name: None,
                value: Box::new(self.parse_expression(Precedence::None)?),
            });

            {
                let current = self.current_token()?;
                if current.kind == TokenKind::Operator(Operator::CloseParen) {
                    break;
                }
            }

            self.consume(TokenKind::Separator(Separator::Comma))?;
        }

        Ok(result)
    }

    pub(crate) fn parse_call_expression(&mut self, callee: Expression) -> crate::parser::Result<Expression> {
        let arguments = self.parse_arguments()?;
        self.consume(TokenKind::Operator(Operator::CloseParen))?;

        Ok(Expression::Call(CallExpression {
            expression: Box::new(callee),
            arguments,
            type_args: vec![],
        }))
    }
}

// #[cfg(test)]
// mod tests {
//     use std::ops::Deref;
//
//     use crate::ast::{BlockExpression, CallExpression, Expression, FunctionDeclarationExpression, IdentifierExpression, LambdaDeclarationExpression, ReturnExpression, Statement};
//     use crate::ast::Expression::{Literal, Return};
//     use crate::ast::Literal::Boolean;
//     use crate::ast::lex::Lexer;
//     use crate::parser::Parser;
//
//     #[test]
//     fn function_call() {
//         let tokens = Lexer::lex("test()").unwrap();
//         let mut result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.pop().unwrap();
//
//         if let Statement::Expression(Expression::Call(CallExpression { expression, arguments, type_args })) = stmt {
//             assert_eq!(expression.deref(), &Expression::Identifier(IdentifierExpression("test".to_string())));
//             assert!(arguments.is_empty());
//             assert!(type_args.is_empty());
//         } else {
//             panic!("Expected single statement with function call, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn call_function_with_named_function() {
//         let tokens = Lexer::lex("test(function some_name() { return true })").unwrap();
//         let mut result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.pop().unwrap();
//
//         if let Statement::Expression(Expression::Call(CallExpression { expression, arguments: parameters, type_args })) = stmt {
//             assert_eq!(expression.deref(), &Expression::Identifier(IdentifierExpression("test".to_string())));
//             assert_eq!(parameters.len(), 1);
//             let parameter = parameters[0];
//             assert!(parameter.name.is_none());
//
//             let expr = *parameter.value.clone();
//             if let Expression::FunctionDeclaration(FunctionDeclarationExpression { name, parameters, return_type, body }) = expr {
//                 assert_eq!(name, Some(IdentifierExpression("some_name".to_string())));
//                 assert_eq!(parameters, vec![]);
//                 assert_eq!(return_type, None);
//                 assert_eq!(body, BlockExpression { body: vec![Return(ReturnExpression { label: None, expr: Some(Box::new(Literal(Boolean(true)))) })] })
//             } else {
//                 panic!("Expected function declaration as first parameter");
//             }
//
//             assert!(type_args.is_empty());
//         } else {
//             panic!("Expected single statement with function call, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn call_function_with_anonymous_function() {
//         let tokens = Lexer::lex("test(function(){ return true })").unwrap();
//         let mut result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.pop().unwrap();
//
//         if let Statement::Expression(Expression::Call(CallExpression { expression, arguments: parameters, type_args })) = stmt {
//             assert_eq!(expression.deref(), &Expression::Identifier(IdentifierExpression("test".to_string())));
//             assert_eq!(parameters.len(), 1);
//             let parameter = parameters[0];
//             assert!(parameter.name.is_none());
//
//             let expr = *parameter.value.clone();
//             if let Expression::FunctionDeclaration(FunctionDeclarationExpression { name, parameters, return_type, body }) = expr {
//                 assert_eq!(name, None);
//                 assert_eq!(parameters, vec![]);
//                 assert_eq!(return_type, None);
//                 assert_eq!(body, BlockExpression { body: vec![Return(ReturnExpression { label: None, expr: Some(Box::new(Literal(Boolean(true)))) })] })
//             } else {
//                 panic!("Expected function declaration as first parameter");
//             }
//
//             assert!(type_args.is_empty());
//         } else {
//             panic!("Expected single statement with function call, got {:?}", stmt)
//         }
//     }
//
//     // #[test]
//     // fn call_function_with_lambda_function() {
//     //     let tokens = Lexer::lex("test(() => { return true })").unwrap();
//     //     let mut result = Parser::parse(&tokens).unwrap();
//     //     assert_eq!(result.block.statements.len(), 1);
//     //
//     //     let stmt = result.block.statements.pop().unwrap();
//     //
//     //     if let Statement::Expression(Expression::Call(CallExpression { expression, arguments: parameters, type_args })) = stmt {
//     //
//     //         assert_eq!(expression.deref(), &Expression::Identifier(IdentifierExpression("test".to_string())));
//     //         assert_eq!(parameters.len(), 1);
//     //         let parameter = parameters[0];
//     //         assert!(parameter.name.is_none());
//     //
//     //         let expr = *parameter.value.clone();
//     //         if let Expression::LambdaDeclaration(LambdaDeclarationExpression { parameters, return_type, body }) = expr {
//     //             assert_eq!(parameters, vec![]);
//     //             assert_eq!(return_type, None);
//     //             assert_eq!(body, BlockExpression { body: vec![Return(ReturnExpression { label: None, expr: Some(Box::new(Literal(Boolean(true)))) })] })
//     //         } else {
//     //             panic!("Expected function declaration as first parameter");
//     //         }
//     //
//     //         assert!(type_args.is_empty());
//     //     } else {
//     //         panic!("Expected single statement with function call, got {:?}", stmt)
//     //     }
//     // }
// }