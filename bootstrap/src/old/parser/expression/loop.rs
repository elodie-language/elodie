use crate::ast::{BlockExpression, BreakExpression, ContinueExpression, Expression, LoopExpression};
use crate::ast::Expression::{Break, Continue};
use crate::core::token::{Keyword, Operator, Separator, TokenKind};
use crate::core::token::Operator::CloseCurly;
use crate::core::token::Separator::NewLine;
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_loop_expression(&mut self) -> crate::parser::Result<Expression> {
        self.consume(TokenKind::Operator(Operator::OpenCurly))?;

        let mut body = Vec::new();
        loop {
            self.consume_if(TokenKind::Separator(NewLine))?;
            if self.current_token_kind()? == &TokenKind::Operator(Operator::CloseCurly) {
                break;
            }
            body.push(self.parse_expression(Precedence::None)?);
        }
        self.consume(TokenKind::Operator(Operator::CloseCurly))?;
        return Ok(
            Expression::Loop(
                LoopExpression {
                    body: BlockExpression {
                        body
                    }
                }
            )
        );
    }

    pub(crate) fn parse_continue_expression(&mut self) -> crate::parser::Result<Expression> {
        self.previous_expect(TokenKind::Keyword(Keyword::Continue))?;
        Ok(Continue(ContinueExpression { label: None }))
    }

    pub(crate) fn parse_break_expression(&mut self) -> crate::parser::Result<Expression> {
        self.previous_expect(TokenKind::Keyword(Keyword::Break))?;

        let has_result = self.current_token_kind()? != &TokenKind::Separator(Separator::NewLine) &&
            self.current_token_kind()? != &TokenKind::Operator(CloseCurly);

        let result = if has_result {
            Some(Box::new(self.parse_expression(Precedence::None)?))
        } else {
            Option::None
        };
        Ok(Break(BreakExpression { label: None, result }))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::ast::{BlockExpression, BreakExpression, ContinueExpression, Expression, LoopExpression, Statement};
//     use crate::ast::Expression::Literal;
//     use crate::ast::Literal::Number;
//     use crate::ast::lex::Lexer;
//     use crate::parser::Parser;
//
//     #[test]
//     fn parse_loop_expression_empty() {
//         let tokens = Lexer::lex("loop{ }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//         let stmt = result.block.statements.first().unwrap();
//
//         if let Statement::Expression(Expression::Loop(LoopExpression { body })) = stmt {
//             assert_eq!(*body, BlockExpression { body: vec![] });
//         } else {
//             panic!("Expected single statement with loop expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_loop_expression_single_expression() {
//         let tokens = Lexer::lex("loop{ 42 }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//         let stmt = result.block.statements.first().unwrap();
//
//         if let Statement::Expression(Expression::Loop(LoopExpression { body })) = stmt {
//             assert_eq!(*body, BlockExpression { body: vec![Literal(Number(42.0))] });
//         } else {
//             panic!("Expected single statement with loop expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_loop_expression_nested_loop() {
//         let tokens = Lexer::lex("loop{ loop{42} }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//         let stmt = result.block.statements.first().unwrap();
//
//         if let Statement::Expression(Expression::Loop(LoopExpression { body })) = stmt {
//             assert_eq!(*body, BlockExpression {
//                 body: vec![
//                     Expression::Loop(LoopExpression { body: BlockExpression { body: vec![Literal(Number(42.0))] } })
//                 ]
//             });
//         } else {
//             panic!("Expected single statement with loop expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_loop_expression_continue() {
//         let tokens = Lexer::lex("loop { continue }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//         let stmt = result.block.statements.first().unwrap();
//
//         if let Statement::Expression(Expression::Loop(LoopExpression { body })) = stmt {
//             assert_eq!(*body, BlockExpression {
//                 body: vec![
//                     Expression::Continue(ContinueExpression { label: None })
//                 ]
//             });
//         } else {
//             panic!("Expected single statement with loop expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_loop_expression_break() {
//         let tokens = Lexer::lex("loop { break }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//         let stmt = result.block.statements.first().unwrap();
//
//         if let Statement::Expression(Expression::Loop(LoopExpression { body })) = stmt {
//             assert_eq!(*body, BlockExpression {
//                 body: vec![
//                     Expression::Break(BreakExpression { label: None, result: None })
//                 ]
//             });
//         } else {
//             panic!("Expected single statement with loop expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_loop_expression_break_literal() {
//         let tokens = Lexer::lex("loop { break 99 }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//         let stmt = result.block.statements.first().unwrap();
//
//         if let Statement::Expression(Expression::Loop(LoopExpression { body })) = stmt {
//             assert_eq!(*body, BlockExpression {
//                 body: vec![
//                     Expression::Break(BreakExpression { label: None, result: Some(Box::new(Expression::Literal(Number(99.0)))) })
//                 ]
//             });
//         } else {
//             panic!("Expected single statement with loop expression, got {:?}", stmt)
//         }
//     }
// }