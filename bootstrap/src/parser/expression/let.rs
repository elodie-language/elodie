use crate::ast::{Expression, LetExpression};
use crate::core::token::{Keyword, Operator, TokenKind};
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_let_expression(&mut self) -> crate::parser::Result<Expression> {
        let name = self.parse_identifier()?;
        self.consume(TokenKind::Operator(Operator::Equal))?;
        let expression = self.parse_expression(Precedence::None)?;
        Ok(Expression::Let(LetExpression {
            name: Box::new(name),
            value: Box::new(expression),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, LetExpression, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn parse_let() {
        let tokens = Lexer::lex("let name = 'Elodie'").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Let(LetExpression { name, value, .. })) = stmt {
            assert_eq!(**name, Expression::Identifier("name".to_string()));
        } else {
            panic!("Expected single statement with let expression, got {:?}", stmt)
        }
    }
}