// use crate::ast::{BreakExpression, ElodieFile, Expression, Statement};
// use crate::interpreter::scope::Scope;
// use crate::interpreter::value::Value;
//
// mod statement;
// mod scope;
// mod value;
// mod expression;
//
// #[derive(Debug)]
// pub enum Error {
//     UnexpectedEndOfFile
// }
//
// pub type Result<T, E = Error> = core::result::Result<T, E>;
//
// #[derive(Clone)]
// pub enum Interrupt {
//     Break(Value),
//     Continue,
//     Return(Value),
// }
//
//
// pub struct Interpreter {
//     scope: Scope,
//     pub interrupt: Option<Interrupt>,
// }
//
// impl Interpreter {
//
//     pub fn new() -> Self {
//         Self {
//             scope: Scope::new(),
//             interrupt: None
//         }
//     }
//
//     pub fn interpret(&mut self, file: ElodieFile) -> Result<()> {
//         for stmt in &file.block.statements {
//             match stmt {
//                 Statement::Expression(expression) => {
//                     self.interpret_expression(expression)?;
//                 }
//             }
//         }
//         Ok(())
//     }
//
//     pub fn interrupt(&mut self, loop_interrupt: Interrupt) {
//         self.interrupt = Some(loop_interrupt)
//     }
//
//     pub fn reset_interrupt(&mut self) {
//         self.interrupt = None
//     }
// }