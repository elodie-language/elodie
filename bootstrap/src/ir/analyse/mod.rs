use std::ops::Index;

pub use node::*;

use crate::common::Context;
use crate::frontend::Ast;
pub use crate::ir::analyse::error::*;
use crate::ir::analyse::infer::Inferrer;
use crate::ir::analyse::pre::Pre;

mod infer;
mod node;
mod pre;
mod scope;
mod error;


pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct TypedAst {
    pub nodes: Vec<TypedTreeNode>,
}

impl Index<usize> for TypedAst {
    type Output = TypedTreeNode;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

pub(crate) fn prepare(ctx: &mut Context, ast: Ast) -> Result<TypedAst> {
    let mut nodes = Pre::new(ctx).process(ast)?;
    Ok(TypedAst { nodes })
}

pub(crate) fn infer(ctx: &mut Context, ast: TypedAst) -> Result<TypedAst> {
    let mut nodes = ast.nodes;
    Inferrer::new(ctx).infer_nodes(&mut nodes)?;
    Ok(TypedAst { nodes })
}

pub(crate) fn analyse(ctx: &mut Context, ast: Ast) -> Result<TypedAst> {
    let prepared = prepare(ctx, ast)?;
    infer(ctx, prepared)
}
