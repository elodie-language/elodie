use crate::ast::{ElodieFile, Statement};
use crate::interpreter::environment::Environment;

mod statement;
mod environment;
mod value;
mod expression;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfFile
}

pub type Result<T, E = Error> = core::result::Result<T, E>;


pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new()
        }
    }

    pub fn interpret(&self, file: ElodieFile) -> Result<()> {
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