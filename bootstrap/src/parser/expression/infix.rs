use crate::ast::{BinaryExpression, Expression, IdentifierExpression, ParameterExpression};
use crate::ast::Expression::{Identifier, Parameter};
use crate::core::token::{Operator, TokenKind};
use crate::parser::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_infix_expression(&mut self, left: Expression) -> crate::parser::Result<Expression> {
        let current = self.current_token()?;
        if current.kind == TokenKind::Operator(Operator::OpenParen) {
            return self.parse_call_expression(left);
        }

        if current.kind == TokenKind::Operator(Operator::Arrow) {
            return Ok(Expression::LambdaDeclaration(self.lambda_declaration(left)?));
        }

        if current.kind == TokenKind::Operator(Operator::Colon) {
            self.consume(TokenKind::Operator(Operator::Colon))?;

            let name = if let Identifier(IdentifierExpression(name)) = left {
                IdentifierExpression(name)
            } else {
                todo!()
            };

            return Ok(Parameter(ParameterExpression {
                name,
                r#type: Some(self.parse_type_expression()?),
            }));
        }

        if current.kind == TokenKind::Operator(Operator::Dot) {
            let previous = self.previous()?;
            assert_eq!(previous.kind, TokenKind::Identifier);
            let _ = self.consume(TokenKind::Operator(Operator::Dot));
            return self.parse_property_access(left);
        }

        let operator = self.parse_binary_operator()?;

        let precedence = self.current_precedence()?;
        let right = self.parse_expression(precedence)?;

        Ok(Expression::Binary(BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinaryOperator, Expression, Statement};
    use crate::new_ast::lex::Lexer;
    use crate::parser::Parser;

    macro_rules! parameterized_expression {
    ($($name:ident, $input:expr => $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let tokens = Lexer::lex($input).unwrap();
                let result = Parser::parse(&tokens).unwrap();
                let stmt = result.block.statements.first().unwrap();

                if let Statement::Expression(Expression::Binary(got)) = stmt {
                     assert_eq!(
                        &got.operator, &$expected,
                        "Failed on input '{}', expected {:?} but got {:?}",
                        $input, $expected, got.operator
                     );
                } else{
                    panic!("Expected binary expression");
                }
            }
        )*
    };
}

    parameterized_expression! {
        add, "5 + 5" => BinaryOperator::Add,
        subtract, "5 - 5" => BinaryOperator::Subtract,
        multiply, "5 * 5" => BinaryOperator::Multiply,
        divide, "5 / 5" => BinaryOperator::Divide,
        modulo, "5 % 5" => BinaryOperator::Modulo,
        greater_than, "5 > 5" => BinaryOperator::GreaterThan,
        greater_than_or_equal, "5 >= 5" => BinaryOperator::GreaterThanOrEqual,
        less_than, "5 < 5" => BinaryOperator::LessThan,
        less_than_or_equal, "5 <= 5" => BinaryOperator::LessThanOrEqual,
        equal, "5 == 5" => BinaryOperator::Equal,
        not_equal, "5 != 5" => BinaryOperator::NotEqual,
    }
}