use crate::ast::{ElodieFile, Statement};
use crate::interpreter::scope::Scope;

mod statement;
mod scope;
mod value;
mod expression;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfFile
}

pub type Result<T, E = Error> = core::result::Result<T, E>;


pub struct Interpreter {
    scope: Scope,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scope: Scope::new()
        }
    }

    pub fn interpret(&mut self, file: ElodieFile) -> Result<()> {
        for stmt in &file.block.statements {
            match stmt {
                Statement::Declaration(_) => unimplemented!(),
                Statement::Expression(expression) => {
                    self.interpret_expression(expression)?;
                }
            }
        }
        Ok(())
    }
}