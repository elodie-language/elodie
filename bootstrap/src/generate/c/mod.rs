pub use node::*;

use crate::generate::c::generator::generate;
use crate::generate::c::emitter::emit;
use crate::ir;

mod generator;
mod node;
mod emitter;

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

pub fn generate_c_code(ctx: &ir::Context) -> Result<String> {
    let node = generate(ctx)?;
    Ok(emit(&node))
}