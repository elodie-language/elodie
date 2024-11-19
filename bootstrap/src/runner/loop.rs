use crate::ast::{BreakLoopNode, ContinueLoopNode, LoopNode};
use crate::runner::{Interrupt, Runner};
use crate::runner::value::Value;

impl Runner {

    pub(crate) fn run_continue(&mut self, _node: &ContinueLoopNode) -> crate::runner::Result<Value> {
        self.interrupt(Interrupt::Continue);
        Ok(Value::Unit)
    }

    pub(crate) fn run_break(&mut self, node: &BreakLoopNode) -> crate::runner::Result<Value> {
        let value = if let Some(result) = node.body.as_ref() {
            self.run_node(result)?
        } else {
            Value::Unit
        };
        self.interrupt(Interrupt::Break(value.clone()));
        Ok(value)
    }

    pub(crate) fn run_loop(&mut self, node: &LoopNode) -> crate::runner::Result<Value> {
        'main: loop {
            self.scope.enter();

            if let Some(Interrupt::Return(return_value)) = &self.interrupt {
                return Ok(return_value.clone());
            }

            for node in &node.body {
                self.run_node(node)?;

                if let Some(interrupt) = &self.interrupt {
                    let interrupt = interrupt.clone();
                    match interrupt {
                        Interrupt::Break(v) => {
                            self.reset_interrupt();
                            self.scope.leave();
                            return Ok(v.clone());
                        }
                        Interrupt::Continue => {
                            self.reset_interrupt();
                            self.scope.leave();
                            continue 'main;
                        }
                        Interrupt::Return(v) => {
                            self.scope.leave();
                            return Ok(v);
                        }
                    }
                }
            }
        }
    }
}