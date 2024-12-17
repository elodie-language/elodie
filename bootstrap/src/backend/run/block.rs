use crate::ir::BlockNode;
use crate::backend::run::{Interrupt, Runner};
use crate::backend::run::value::Value;

impl<'a> Runner<'a> {
    pub(crate) fn run_block(&mut self, node: &BlockNode) -> crate::backend::run::Result<Value> {
        let mut value = Value::Unit;
        self.scope.enter();

        for node in &node.body {
            if let Some(Interrupt::Return(return_value)) = &self.interrupt {
                return Ok(return_value.clone());
            }

            value = self.run_node(node)?;
        }

        self.scope.leave();
        Ok(value)
    }
}