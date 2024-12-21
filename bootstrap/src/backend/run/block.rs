use crate::backend::run::{Interrupt, Runner};
use crate::backend::run::value::Value;
use crate::frontend::ast::AstBlockNode;

impl<'a> Runner<'a> {
    pub(crate) fn run_block(
        &mut self,
        node: &AstBlockNode,
    ) -> crate::backend::run::Result<Value> {
        let mut value = Value::Unit;
        self.scope.enter();

        for node in &node.nodes {
            if let Some(Interrupt::Return(return_value)) = &self.interrupt {
                return Ok(return_value.clone());
            }

            value = self.run_node(node)?;
        }

        self.scope.leave();
        Ok(value)
    }
}
