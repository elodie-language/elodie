use crate::ast::{Expression, IfExpression};
use crate::core::token::{Keyword, Operator, TokenKind};
use crate::core::token::Keyword::Else;
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {

    pub(crate) fn parse_if_expression(&mut self) -> crate::parser::Result<Expression> {
        self.previous_expect(TokenKind::Keyword(Keyword::If))?;
        let condition = self.parse_expression(Precedence::None)?;
        self.consume(TokenKind::Operator(Operator::OpenCurly))?;
        let then = self.parse_block_expression()?;
        let otherwise = if self.current_token_kind()? == &TokenKind::Keyword(Else){
            self.consume(TokenKind::Keyword(Else))?;
            self.consume(TokenKind::Operator(Operator::OpenCurly))?;
            Some(self.parse_block_expression()?)
        }else {
            None
         };
        Ok(Expression::If(IfExpression {
            condition: Box::new(condition),
            then,
            otherwise: otherwise,
        }))
    }
}

// #[cfg(test)]
// mod tests {
//
//     use crate::ast::{BinaryExpression, BinaryOperator, BlockExpression, Expression, IdentifierExpression, IfExpression, Statement};
//     use crate::ast::Expression::{Binary, Block, Identifier, Literal};
//     use crate::ast::Literal::Boolean;
//     use crate::ast::lex::Lexer;
//     use crate::parser::Parser;
//
//     #[test]
//     fn parse_if_expression() {
//         let tokens = Lexer::lex("if x == true { false }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//         let stmt = result.block.statements.first().unwrap();
//
//         if let Statement::Expression(Expression::If(IfExpression { condition, then, otherwise })) = stmt {
//             println!("{stmt:?}");
//             println!("test");
//             assert_eq!(**condition, Binary(BinaryExpression{
//                 left: Box::new(Identifier(IdentifierExpression("x".to_string()))),
//                 operator: BinaryOperator::Equal,
//                 right: Box::new(Literal(Boolean(true))),
//             }));
//             assert_eq!(*then, BlockExpression{ body: vec![Literal(Boolean(false))]});
//             assert_eq!(*otherwise, None);
//         } else {
//             panic!("Expected single statement with if expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_if_expression_multiline_then() {
//         let tokens = Lexer::lex(r#"if x == true {
//             false
//             true
//         }"#).unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//         let stmt = result.block.statements.first().unwrap();
//
//         if let Statement::Expression(Expression::If(IfExpression { condition, then, otherwise })) = stmt {
//             assert_eq!(**condition, Binary(BinaryExpression{
//                 left: Box::new(Identifier(IdentifierExpression("x".to_string()))),
//                 operator: BinaryOperator::Equal,
//                 right: Box::new(Literal(Boolean(true))),
//             }));
//             assert_eq!(*then, BlockExpression{ body: vec![Literal(Boolean(false)), Literal(Boolean(true))]});
//             assert_eq!(*otherwise, None);
//         } else {
//             panic!("Expected single statement with if expression, got {:?}", stmt)
//         }
//     }
//
//     #[test]
//     fn parse_if_else_expression() {
//         let tokens = Lexer::lex("if x == true { true } else { false }").unwrap();
//         let result = Parser::parse(&tokens).unwrap();
//         assert_eq!(result.block.statements.len(), 1);
//         let stmt = result.block.statements.first().unwrap();
//
//         if let Statement::Expression(Expression::If(IfExpression { condition, then, otherwise })) = stmt {
//             assert_eq!(**condition, Binary(BinaryExpression{
//                 left: Box::new(Identifier(IdentifierExpression("x".to_string()))),
//                 operator: BinaryOperator::Equal,
//                 right: Box::new(Literal(Boolean(true))),
//             }));
//             assert_eq!(*then, BlockExpression{ body: vec![Literal(Boolean(true))]});
//             assert_eq!(*otherwise, Some(BlockExpression{ body: vec![Literal(Boolean(false))]}));
//         } else {
//             panic!("Expected single statement with if expression, got {:?}", stmt)
//         }
//     }
//
// }