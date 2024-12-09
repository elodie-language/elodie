use crate::generate::c;
use crate::generate::c::{BlockStatement, Indent, Statement};
use crate::generate::c::generator::Generator;
use crate::ir::BlockNode;

impl Generator {
    pub(crate) fn generate_block(&mut self, node: &BlockNode) -> c::generator::Result<BlockStatement> {
        self.scope.enter();

        let mut statements = vec![];
        for node in &node.body {
            statements.extend(self.generate_statements(node)?)
        }

        let result = BlockStatement { indent: Indent::none(), statements };
        self.scope.leave();
        Ok(result)
    }
}