use std::collections::HashMap;
use std::ops::Index;

pub use node::*;

use crate::common::StringTableId;
use crate::frontend::NewAst;
use crate::ir::analyse::infer::Inference;
use crate::ir::Context;

mod node;
mod infer;

#[derive(Debug, Clone, PartialEq)]
pub enum InferredType {
    Unknown,

    Boolean,
    Number,
    String,
    Tuple(Vec<InferredType>),
    ObjectType(HashMap<StringTableId, InferredType>),

    OneOf(Vec<InferredType>),
    AllOf(Vec<InferredType>),

}

impl InferredType {}

#[derive(Debug)]
pub enum Error {}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Analysed {
    pub nodes: Vec<AnalysedNode>,
}

impl Index<usize> for Analysed {
    type Output = AnalysedNode;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

pub(crate) fn analyse(ctx: &mut Context, ast: NewAst) -> Result<Analysed> {
    let inferred = Inference::new(ctx).infer(ast)?;

    Ok(Analysed { nodes: inferred })
}

