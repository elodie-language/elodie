use crate::ast::{Expression, IfExpression};
use crate::core::token::{Operator, TokenKind};
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_if_expression(&mut self) -> crate::parser::Result<Expression> {
        let condition = self.parse_expression(Precedence::None)?;
        self.current_expect(TokenKind::Operator(Operator::OpenCurly))?;
        let then = self.parse_block_expression()?;

        // self.consume(TokenKind::Operator(Operator::OpenCurly))?;
        // self.consume(TokenKind::Operator(Operator::CloseCurly))?;

        Ok(Expression::If(IfExpression {
            condition: Box::new(condition),
            then,
            otherwise: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, IfExpression, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    // #[test]
    // fn parse_if_expression() {
    //     let tokens = Lexer::lex("if x == true { console.log('hit') }").unwrap();
    //     let result = Parser::parse(&tokens).unwrap();
    //     let stmt = result.block.statements.first().unwrap();
    //
    //     if let Statement::Expression(Expression::If(IfExpression { condition, then, otherwise })) = stmt {
    //         println!("test");
    //     } else {
    //         panic!("Expected single statement with if expression, got {:?}", stmt)
    //     }
    // }
    //
    // #[test]
    // fn parse_if_multiline_then() {
    //     let tokens = Lexer::lex("if x == true { console.log('hit') }").unwrap();
    //     let result = Parser::parse(&tokens).unwrap();
    //     let stmt = result.block.statements.first().unwrap();
    //
    //     if let Statement::Expression(Expression::If(IfExpression { condition, then, otherwise })) = stmt {
    //         println!("test");
    //     } else {
    //         panic!("Expected single statement with if expression, got {:?}", stmt)
    //     }
    // }
}