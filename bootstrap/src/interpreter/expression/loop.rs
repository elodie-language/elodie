use crate::ast::{BreakExpression, ContinueExpression, LoopExpression};
use crate::interpreter::Interpreter;
use crate::interpreter::scope::LoopInterrupt;
use crate::interpreter::value::Value;
use crate::interpreter::value::Value::Unit;

impl Interpreter {
    pub(crate) fn interpret_continue_expression(&mut self, _expr: &ContinueExpression) -> crate::interpreter::Result<Value> {
        self.scope.interrupt_loop(LoopInterrupt::Continue);
        Ok(Unit)
    }

    pub(crate) fn interpret_break_expression(&mut self, expr: &BreakExpression) -> crate::interpreter::Result<Value> {
        let value = if let Some(result) = expr.result.as_ref() {
            self.interpret_expression(result)?
        } else {
            Value::Unit
        };
        self.scope.interrupt_loop(LoopInterrupt::Break(value.clone()));
        Ok(value)
    }

    pub(crate) fn interpret_loop_expression(&mut self, expr: &LoopExpression) -> crate::interpreter::Result<Value> {
        'main: loop {
            self.scope.enter();

            for expr in &expr.body.body {
                self.interpret_expression(expr)?;

                if let Some(interrupt) = &self.scope.loop_interrupt{
                    let interrupt = interrupt.clone();
                    match interrupt {
                        LoopInterrupt::Break(v) => {
                            self.scope.reset_loop_interrupt();
                            self.scope.leave();
                            return Ok(v.clone());
                        }
                        LoopInterrupt::Continue => {
                            self.scope.reset_loop_interrupt();
                            self.scope.leave();
                            continue 'main;
                        }
                        LoopInterrupt::Return(_) => {}
                    }
                }
            }
        }
    }
}