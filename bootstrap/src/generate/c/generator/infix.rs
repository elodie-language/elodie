use crate::generate::c;
use crate::generate::c::{InfixExpression, InfixOperator, Statement};
use crate::generate::c::generator::Generator;
use crate::ir::{CompareNode, CompareOperator};

impl Generator {
    pub(crate) fn generate_compare(&mut self, node: &CompareNode) -> c::generator::Result<(Vec<Statement>, InfixExpression)> {
        let mut statements = vec![];

        let (left_statements, left_expression) = self.generate_expression(&node.left)?;
        let (right_statements, right_expression) = self.generate_expression(&node.right)?;

        statements.extend(left_statements);
        statements.extend(right_statements);

        let operator = match node.operator {
            CompareOperator::Equal => InfixOperator::Equal,
            CompareOperator::NotEqual => InfixOperator::NotEqual,
            CompareOperator::GreaterThan => InfixOperator::GreaterThan
        };

        return Ok((statements, InfixExpression {
            left: Box::new(left_expression),
            operator,
            right: Box::new(right_expression),
        }));
    }
}