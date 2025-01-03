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

        let then_frame = self.scope.frame();

        let otherwise = if let Some(otherwise) = &node.otherwise {
            self.scope.enter();
            for node in &otherwise.nodes {
                self.nodes(node.as_ref())?
            }
            let otherwise_frame = self.scope.frame();
            Some(BlockStatement { statements: otherwise_frame.statements })
        } else {
            None
        };


        self.statements().push(If(IfStatement {
            condition,
            then: BlockStatement { statements: then_frame.statements },
            otherwise,
        }));

        return Ok(());
    }
}