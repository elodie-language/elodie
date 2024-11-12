// use crate::ast::{Expression, FunctionDeclarationExpression, LambdaDeclarationExpression, ParameterExpression};
// use crate::core::token::{Keyword, Operator, Separator, TokenKind};
// use crate::parser::Parser;
//
// impl<'a> Parser<'a> {
//
//     pub(crate) fn lambda_declaration(&mut self, left: Expression) -> crate::parser::Result<LambdaDeclarationExpression> {
//         self.previous_expect(TokenKind::Operator(Operator::Arrow))?;
//         // let name = if self.current_token_kind()? == &TokenKind::Identifier {
//         //     Some(self.parse_identifier()?)
//         // } else {
//         //     None
//         // };
//         //
//         // self.consume(TokenKind::Operator(Operator::OpenParen))?;
//         //
//         // let mut parameters = vec![];
//         // loop {
//         //     if self.current_token_kind()? == &TokenKind::Operator(Operator::CloseParen) {
//         //         break;
//         //     }
//         //     parameters.push(self.parameter_expression()?);
//         //     self.consume_if(TokenKind::Separator(Separator::Comma))?;
//         // }
//         //
//         // self.consume(TokenKind::Operator(Operator::CloseParen))?;
//         //
//         // let return_type = if self.current_token_kind()? == &TokenKind::Operator(Operator::Colon) {
//         //     self.consume(TokenKind::Operator(Operator::Colon))?;
//         //     Some(self.parse_type_expression()?)
//         // } else {
//         //     None
//         // };
//
//         self.consume(TokenKind::Operator(Operator::OpenCurly))?;
//         let body = self.parse_block_expression()?;
//
//         Ok(
//             LambdaDeclarationExpression {
//                 parameters: vec![],
//                 return_type: None,
//                 body,
//             }
//         )
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::ast::{BlockExpression, Expression, LambdaDeclarationExpression, Statement};
//     use crate::lexer::Lexer;
//     use crate::parser::Parser;
//
//     #[test]
//     fn lambda_without_args_and_result_type() {
//         let tokens = Lexer::lex("(arg_1) => {}").unwrap();
//         let mut result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.pop().unwrap();
//
//         if let Statement::Expression(Expression::LambdaDeclaration(LambdaDeclarationExpression {
//                                                                          parameters,
//                                                                          return_type,
//                                                                          body
//                                                                      })) = stmt {
//             assert_eq!(parameters, vec![]);
//             assert_eq!(return_type, None);
//             assert_eq!(body, BlockExpression { body: vec![] })
//         } else {
//             panic!("Expected single statement with lambda declaration, got {:?}", stmt)
//         }
//     }
// }