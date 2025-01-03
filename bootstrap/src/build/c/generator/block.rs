use crate::build::c;
use crate::build::c::generator::Generator;
use crate::ir::IrBlockNode;

impl Generator {
    pub(crate) fn block(&mut self, node: &IrBlockNode) -> c::generator::Result<()> {
        self.scope.enter();

        for node in &node.nodes {
            self.nodes(node.as_ref())?
        }

        self.scope.leave();
        Ok(())
    }
}