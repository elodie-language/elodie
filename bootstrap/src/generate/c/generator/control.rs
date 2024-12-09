use std::ops::Deref;

use crate::generate::c;
use crate::generate::c::{IfStatement, Statement};
use crate::generate::c::generator::Generator;
use crate::ir::IfNode;

impl Generator {
    pub(crate) fn generate_if(&mut self, node: &IfNode) -> c::generator::Result<Vec<Statement>> {
        let mut result = vec![];

        let (statements, condition) = self.generate_expression(node.condition.deref())?;
        result.extend(statements);

        let then = self.generate_block(&node.then)?;
        let otherwise = if let Some(block) = &node.otherwise {
            Some(self.generate_block(block)?)
        } else {
            None
        };

        result.push(Statement::If(IfStatement {
            condition,
            then,
            otherwise,
        }));

        return Ok(result);
    }
}