use crate::ast::{Expression, LetExpression, TypeExpression};
use crate::core::token::{Operator, TokenKind};
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_let_expression(&mut self) -> crate::parser::Result<Expression> {
        let name = self.parse_identifier()?;
        let r#type = if self.current_token_kind()? == &TokenKind::Operator(Operator::Colon) {
            // parse type
            self.consume(TokenKind::Operator(Operator::Colon))?;
            self.parse_type_expression()?
        } else {
            TypeExpression::Fundamentals("Any".to_string())
        };

        self.consume(TokenKind::Operator(Operator::Equal))?;
        let expression = self.parse_expression(Precedence::None)?;
        Ok(Expression::Let(LetExpression {
            name,
            value: Box::new(expression),
            r#type: r#type,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, IdentifierExpression, LetExpression, Statement};
    use crate::ast::TypeExpression::Fundamentals;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn parse_let() {
        let tokens = Lexer::lex("let name = 'Elodie'").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Let(LetExpression { name, value, r#type })) = stmt {
            assert_eq!(name, &IdentifierExpression("name".to_string()));
            assert_eq!(r#type, &Fundamentals("Any".to_string()));
        } else {
            panic!("Expected single statement with let expression, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_let_with_type_string() {
        let tokens = Lexer::lex("let name : String = 'Elodie'").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Let(LetExpression { name, value, r#type })) = stmt {
            assert_eq!(name, &IdentifierExpression("name".to_string()));
            assert_eq!(r#type, &Fundamentals("String".to_string()));
        } else {
            panic!("Expected single statement with let expression, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_let_with_type_any() {
        let tokens = Lexer::lex("let name : Any = 'Elodie'").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Let(LetExpression { name, value, r#type })) = stmt {
            assert_eq!(name, &IdentifierExpression("name".to_string()));
            assert_eq!(r#type, &Fundamentals("Any".to_string()));
        } else {
            panic!("Expected single statement with let expression, got {:?}", stmt)
        }
    }
}