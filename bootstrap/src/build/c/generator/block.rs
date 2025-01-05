use crate::build::c;
use crate::build::c::{BlockStatement, Statement};
use crate::build::c::generator::Generator;
use crate::ir::IrBlockNode;

impl Generator {
    pub(crate) fn block(&mut self, node: &IrBlockNode) -> c::generator::Result<()> {
        self.scope.enter();

        for node in &node.nodes {
            self.nodes(node.as_ref())?
        }

        // self.scope.leave();

        let mut frame = self.scope.leave();
        let cleanup_statements = frame.cleanup();

        let mut statements = vec![];
        statements.extend(frame.statements);
        statements.extend(cleanup_statements);

        self.statements().push(Statement::Block(BlockStatement { statements }));

        Ok(())
    }
}