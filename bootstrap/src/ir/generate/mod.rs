use crate::ir::analyse::Analysed;
use crate::ir::Context;
use crate::ir::Ir;

mod literal;

pub(crate) fn generate<'a>(ctx: &'a mut Context, analysed: &'a Analysed) -> crate::ir::Result<Ir> {
    todo!()
}