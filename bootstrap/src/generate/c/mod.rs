pub(crate) use node::*;

use crate::generate::c::generator::generate;
use crate::generate::c::writer::write;
use crate::ir;

mod node;
mod writer;
mod generator;

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

impl From<writer::Error> for Error {
    fn from(value: writer::Error) -> Self {
        todo!()
    }
}

type Result<T> = core::result::Result<T, Error>;

pub fn generate_c_code(ctx: &ir::Context) -> Result<String> {
    let node = generate(ctx)?;
    Ok(write(&node)?)
}