pub use node::*;

use crate::backend::generate::c::emitter::emit;
use crate::backend::generate::c::generator::generate;
use crate::frontend::Ast;
use crate::{frontend, ir};
use crate::common::Context;

mod emitter;
mod generator;
mod node;

#[derive(Debug)]
pub enum Error {
    // generator error
    // writer error
}

impl From<generator::Error> for Error {
    fn from(value: generator::Error) -> Self {
        todo!()
    }
}

impl From<emitter::Error> for Error {
    fn from(value: emitter::Error) -> Self {
        todo!()
    }
}

type Result<T> = core::result::Result<T, Error>;

pub fn generate_c_code(ctx: Context, ast: Ast) -> Result<String> {
    let node = generate(ctx, ast)?;
    Ok(emit(&node))
}
