use crate::ast::BlockExpression;
use crate::core::token::{Operator, TokenKind};
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_block_expression(&mut self) -> crate::parser::Result<BlockExpression> {
        self.previous_expect(TokenKind::Operator(Operator::OpenCurly))?;

        let mut body = Vec::new();
        loop {
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

#[cfg(test)]
mod tests {
    use crate::ast::{BlockExpression, Expression, IfExpression, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn parse_block_expression_empty() {
        let tokens = Lexer::lex("{}").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        assert_eq!(result.block.statements.len(), 1);
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Block(BlockExpression { body })) = stmt {
            assert_eq!(body, &vec![]);
        } else {
            panic!("Expected single statement with block expression, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_block_expression_with_whitespace() {
        let tokens = Lexer::lex("{    }").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Block(BlockExpression { body })) = stmt {
            assert_eq!(body, &vec![]);
        } else {
            panic!("Expected single statement with block expression, got {:?}", stmt)
        }
    }

// empty block
    // single
    // single nested block
    // multi nested blocks
    //
    // #[test]
    // fn parse_block_expression() {
    //     let tokens = Lexer::lex("{ console.log('hit') }").unwrap();
    //     let result = Parser::parse(&tokens).unwrap();
    //     let stmt = result.block.statements.first().unwrap();
    //
    //     if let Statement::Expression(Expression::If(IfExpression { condition, then, otherwise })) = stmt {
    //         println!("test");
    //     } else {
    //         panic!("Expected single statement with block expression, got {:?}", stmt)
    //     }
    // }
}