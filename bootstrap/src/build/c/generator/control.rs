use crate::build::c;
use crate::build::c::{BlockStatement, IfStatement};
use crate::build::c::generator::Generator;
use crate::build::c::Statement::If;
use crate::ir::IrIfNode;

impl Generator {
    pub(crate) fn r#if(&mut self, node: &IrIfNode) -> c::generator::Result<()> {
        let condition = self.expression(node.condition.as_ref())?;

        self.scope.enter();
        for node in &node.then.nodes {
            self.nodes(node.as_ref())?
        }

        let mut then_frame = self.scope.leave();
        let then_cleanup_statements = then_frame.cleanup();

        let mut then_statements = vec![];
        then_statements.extend(then_frame.statements);
        then_statements.extend(then_cleanup_statements);


        let otherwise = if let Some(otherwise) = &node.otherwise {
            self.scope.enter();
            for node in &otherwise.nodes {
                self.nodes(node.as_ref())?
            }

            let mut frame = self.scope.leave();
            let cleanup_statements = frame.cleanup();

            let mut statements = vec![];
            statements.extend(frame.statements);
            statements.extend(cleanup_statements);

            Some(BlockStatement { statements })
        } else {
            None
        };


        self.statements().push(If(IfStatement {
            condition,
            then: BlockStatement { statements: then_statements },
            otherwise,
        }));

        return Ok(());
    }
}