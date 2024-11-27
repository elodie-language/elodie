use crate::generate::c;
use crate::ir;

pub enum Error {}

type Result<T> = core::result::Result<T, Error>;

pub(crate) fn generate(ctx: &ir::Context) -> Result<c::Node> {
    let mut generator = Generator {};
    generator.generate(ctx)
}

pub(crate) struct Generator {}

impl Generator {
    pub(crate) fn generate(&mut self, ctx: &ir::Context) -> Result<c::Node> {
        ///...
        Ok(c::Node::Block())
    }
}