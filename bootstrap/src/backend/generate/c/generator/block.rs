use crate::backend::generate::c;
use crate::backend::generate::c::{BlockStatement, Indent};
use crate::backend::generate::c::generator::Generator;
use crate::common::tree::BlockNode;
use crate::frontend::ast::node::AstVariant;

impl Generator {
    pub(crate) fn generate_block(
        &mut self,
        node: &impl  BlockNode<AstVariant>,
    ) -> c::generator::Result<BlockStatement> {
        self.scope.enter();

        let mut statements = vec![];
        for node in &node.nodes() {
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
