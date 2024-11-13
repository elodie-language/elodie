use crate::ast::BlockExpression;
use crate::core::token::{Operator, TokenKind};
use crate::core::token::Separator::NewLine;
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_block_expression(&mut self) -> crate::parser::Result<BlockExpression> {
        self.previous_expect(TokenKind::Operator(Operator::OpenCurly))?;

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
            BlockExpression {
                body
            }
        );
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::ast::{BlockExpression, Expression, Statement};
//     use crate::ast::Expression::{Block, Literal};
//     use crate::ast::Literal::Boolean;
//     use crate::ast::lex::Lexer;
//     use crate::parser::Parser;
//
//     #[test]
//     fn parse_block_expression_empty() {
//         let tokens = Lexer::lex("{}").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//         let stmt = result.block.statements.first().unwrap();
//
//         if let Statement::Expression(Expression::Block(BlockExpression { body })) = stmt {
//             assert_eq!(body, &vec![]);
//         } else {
//             panic!("Expected single statement with block expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_block_expression_with_whitespace() {
//         let tokens = Lexer::lex("{    }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.first().unwrap();
//         if let Statement::Expression(Expression::Block(BlockExpression { body })) = stmt {
//             assert_eq!(body, &vec![]);
//         } else {
//             panic!("Expected single statement with block expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_block_expression_single_expression() {
//         let tokens = Lexer::lex("{ true }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.first().unwrap();
//         if let Statement::Expression(Expression::Block(BlockExpression { body })) = stmt {
//             assert_eq!(body, &vec![Literal(Boolean(true))]);
//         } else {
//             panic!("Expected single statement with block expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_block_expression_multiple_expressions() {
//         let tokens = Lexer::lex(r#"{
//             true
//             false
//         }"#).unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.first().unwrap();
//         if let Statement::Expression(Expression::Block(BlockExpression { body })) = stmt {
//             assert_eq!(body, &vec![
//                 Literal(Boolean(true)),
//                 Literal(Boolean(false)),
//             ]);
//         } else {
//             panic!("Expected single statement with block expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_block_expression_nested_empty_block() {
//         let tokens = Lexer::lex("{ {} }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.first().unwrap();
//         if let Statement::Expression(Expression::Block(BlockExpression { body })) = stmt {
//             assert_eq!(body, &vec![
//                 Block(BlockExpression { body: vec![] })
//             ]);
//         } else {
//             panic!("Expected single statement with block expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_block_expression_multi_layer_nesting() {
//         let tokens = Lexer::lex("{ { \
//         {{ true }}\
//         } }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//
//         let stmt = result.block.statements.first().unwrap();
//         if let Statement::Expression(Expression::Block(BlockExpression { body })) = stmt {
//             assert_eq!(body, &vec![
//                 Block(BlockExpression {
//                     body: vec![
//                         Expression::Block(BlockExpression {
//                             body: vec![
//                                 Expression::Block(BlockExpression { body: vec![Literal(Boolean(true))] })
//                             ]
//                         })
//                     ]
//                 })
//             ]);
//         } else {
//             panic!("Expected single statement with block expression, got {:?}", stmt)
//         }
//     }
// }