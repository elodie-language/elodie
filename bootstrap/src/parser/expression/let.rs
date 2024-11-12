use crate::ast::{Expression, LetExpression};
use crate::core::token::{Operator, TokenKind};
use crate::parser::Parser;
use crate::parser::precedence::Precedence;

impl<'a> Parser<'a> {
    pub(crate) fn parse_let_expression(&mut self) -> crate::parser::Result<Expression> {
        let name = self.parse_identifier()?;
        let r#type = if self.current_token_kind()? == &TokenKind::Operator(Operator::Colon) {
            // parse type
            self.consume(TokenKind::Operator(Operator::Colon))?;
            Some(self.parse_type_expression()?)
        } else {
            None
        };

        self.consume(TokenKind::Operator(Operator::Equal))?;
        let expression = self.parse_expression(Precedence::None)?;
        Ok(Expression::Let(LetExpression {
            name,
            value: Box::new(expression),
            r#type,
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::ast::{Expression, IdentifierExpression, LetExpression, Literal, Statement};
    use crate::ast::TypeExpression::Fundamentals;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn parse_let() {
        let tokens = Lexer::lex("let value = 'Elodie'").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Let(LetExpression { name, value, r#type })) = stmt {
            assert_eq!(name, &IdentifierExpression("value".to_string()));
            assert_eq!(*r#type, None);
            assert_eq!(value.deref(), &Expression::Literal(Literal::String("Elodie".to_string())));
        } else {
            panic!("Expected single statement with let expression, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_let_with_type_string() {
        let tokens = Lexer::lex("let value : String = 'Elodie'").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Let(LetExpression { name, value, r#type })) = stmt {
            assert_eq!(name, &IdentifierExpression("value".to_string()));
            assert_eq!(*r#type, Some(Fundamentals("String".to_string())));
            assert_eq!(value.deref(), &Expression::Literal(Literal::String("Elodie".to_string())));
        } else {
            panic!("Expected single statement with let expression, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_let_with_type_any() {
        let tokens = Lexer::lex("let value : Any = 'Elodie'").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Let(LetExpression { name, value, r#type })) = stmt {
            assert_eq!(name, &IdentifierExpression("value".to_string()));
            assert_eq!(*r#type, Some(Fundamentals("Any".to_string())));
            assert_eq!(value.deref(), &Expression::Literal(Literal::String("Elodie".to_string())));
        } else {
            panic!("Expected single statement with let expression, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_let_with_type_number() {
        let tokens = Lexer::lex("let value : Number = 99").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Let(LetExpression { name, value, r#type })) = stmt {
            assert_eq!(name, &IdentifierExpression("value".to_string()));
            assert_eq!(*r#type, Some(Fundamentals("Number".to_string())));
            assert_eq!(value.deref(), &Expression::Literal(Literal::Number(99.0)));
        } else {
            panic!("Expected single statement with let expression, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_let_with_type_boolean_true() {
        let tokens = Lexer::lex("let value : Bool = true").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Let(LetExpression { name, value, r#type })) = stmt {
            assert_eq!(name, &IdentifierExpression("value".to_string()));
            assert_eq!(*r#type, Some(Fundamentals("Bool".to_string())));
            assert_eq!(value.deref(), &Expression::Literal(Literal::Boolean(true)));
        } else {
            panic!("Expected single statement with let expression, got {:?}", stmt)
        }
    }

    #[test]
    fn parse_let_with_type_boolean_false() {
        let tokens = Lexer::lex("let value : Bool = false").unwrap();
        let result = Parser::parse(&tokens).unwrap();
        let stmt = result.block.statements.first().unwrap();

        if let Statement::Expression(Expression::Let(LetExpression { name, value, r#type })) = stmt {
            assert_eq!(name, &IdentifierExpression("value".to_string()));
            assert_eq!(*r#type, Some(Fundamentals("Bool".to_string())));
            assert_eq!(value.deref(), &Expression::Literal(Literal::Boolean(false)));
        } else {
            panic!("Expected single statement with let expression, got {:?}", stmt)
        }
    }
}