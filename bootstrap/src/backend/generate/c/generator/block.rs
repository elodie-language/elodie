use crate::backend::generate::c;
use crate::backend::generate::c::generator::Generator;
use crate::backend::generate::c::{BlockStatement, Indent};
use crate::frontend::old_ast;

impl Generator {
    pub(crate) fn generate_block(
        &mut self,
        node: &old_ast::BlockNode,
    ) -> c::generator::Result<BlockStatement> {
        self.scope.enter();

        let mut statements = vec![];
        for node in &node.body {
            statements.extend(self.generate_statements(node)?)
        }

        let result = BlockStatement {
            indent: Indent::none(),
            statements,
        };
        self.scope.leave();
        Ok(result)
    }
}
