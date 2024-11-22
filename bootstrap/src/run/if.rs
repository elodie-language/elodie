use crate::ast::IfNode;
use crate::run::Runner;
use crate::run::value::Value;

impl Runner {
    pub(crate) fn run_if(&mut self, node: &IfNode) -> crate::run::Result<Value> {
        let condition = self.run_node(&node.condition)?;
        match condition {
            Value::Bool(v) => {
                if v {
                    self.scope.enter();
                    // let result = self.interpret_expression(&Expression::Block(expr.then.clone()));
                    let result = self.run_block(&node.then)?;
                    self.scope.leave();
                    Ok(result)
                } else {
                    self.scope.enter();
                    let result = self.run_block(&node.otherwise)?;
                    self.scope.leave();
                    Ok(result)
                }
            }
            v => unimplemented!("{v:?}")
        }
    }
}