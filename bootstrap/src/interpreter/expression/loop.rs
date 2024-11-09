use crate::ast::{BreakExpression, ContinueExpression, LoopExpression};
use crate::interpreter::{Interpreter, Interrupt};
use crate::interpreter::value::Value;
use crate::interpreter::value::Value::Unit;

impl Interpreter {
    pub(crate) fn interpret_continue_expression(&mut self, _expr: &ContinueExpression) -> crate::interpreter::Result<Value> {
        self.interrupt(Interrupt::Continue);
        Ok(Unit)
    }

    pub(crate) fn interpret_break_expression(&mut self, expr: &BreakExpression) -> crate::interpreter::Result<Value> {
        let value = if let Some(result) = expr.result.as_ref() {
            self.interpret_expression(result)?
        } else {
            Value::Unit
        };
        self.interrupt(Interrupt::Break(value.clone()));
        Ok(value)
    }

    pub(crate) fn interpret_loop_expression(&mut self, expr: &LoopExpression) -> crate::interpreter::Result<Value> {
        'main: loop {
            self.scope.enter();

            if let Some(Interrupt::Return(return_value)) = &self.interrupt {
                return Ok(return_value.clone());
            }

            for expr in &expr.body.body {
                self.interpret_expression(expr)?;

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