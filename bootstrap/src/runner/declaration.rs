use std::ops::Deref;

use crate::ast::DeclareVariableNode;
use crate::runner::Runner;
use crate::runner::value::Value;

impl Runner {
    pub(crate) fn run_variable_declaration(&mut self, node: &DeclareVariableNode) -> crate::runner::Result<Value> {
        let name = node.identifier.0.clone();
        let value = self.run_node(node.value.deref())?;
        self.scope.insert(name, value);
        Ok(Value::Unit)
    }
}