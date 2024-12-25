use crate::backend::build::c;
use crate::backend::build::c::generator::Generator;
use crate::backend::build::c::{BlockStatement, Indent};
use crate::frontend::ast::AstBlockNode;

impl Generator {
    pub(crate) fn generate_block(
        &mut self,
        node: &AstBlockNode,
    ) -> c::generator::Result<BlockStatement> {
        self.scope.enter();

        let mut statements = vec![];
        for node in &node.nodes {
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
