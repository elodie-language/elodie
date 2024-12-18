use std::ops::Deref;

use crate::backend::generate::c;
use crate::backend::generate::c::{IfStatement, Statement};
use crate::backend::generate::c::generator::Generator;
use crate::frontend::ast;

impl Generator {
    pub(crate) fn generate_if(&mut self, node: &ast::IfNode) -> c::generator::Result<Vec<Statement>> {
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