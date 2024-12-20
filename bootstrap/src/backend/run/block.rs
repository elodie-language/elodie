use crate::backend::run::value::Value;
use crate::backend::run::{Interrupt, Runner};
use crate::frontend::ast;
use crate::frontend::ast::node::AstNode;

impl<'a> Runner<'a> {
    pub(crate) fn run_block(
        &mut self,
        node: &ast::BlockNode<AstNode>,
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
