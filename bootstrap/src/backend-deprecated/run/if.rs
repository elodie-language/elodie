use crate::backend::run::value::Value;
use crate::backend::run::Runner;
use crate::frontend::ast::AstIfNode;

impl<'a> Runner<'a> {
    pub(crate) fn run_if(&mut self, node: &AstIfNode) -> crate::backend::run::Result<Value> {
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
                    if let Some(otherwise) = &node.otherwise {
                        self.scope.enter();
                        let result = self.run_block(otherwise)?;
                        self.scope.leave();
                        Ok(result)
                    } else {
                        Ok(Value::Unit)
                    }
                }
            }
            v => unimplemented!("{v:?}"),
        }
    }
}
