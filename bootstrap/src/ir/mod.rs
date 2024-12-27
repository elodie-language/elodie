use std::ops::Index;


use crate::{frontend, ir};
use crate::common::Context;
use crate::frontend::ast_from_str;
use crate::ir::analyse::analyse;
use crate::ir::generate::generate;
pub use crate::ir::node::*;

mod analyse;
mod generate;
pub(crate) mod node;

#[derive(Debug)]
pub enum Error {
    Frontend(frontend::Error),
    Analyse(analyse::Error),
    Generate(generate::Error),
}

impl From<frontend::Error> for Error {
    fn from(value: frontend::Error) -> Self {
        Self::Frontend(value)
    }
}

impl From<analyse::Error> for Error {
    fn from(value: analyse::Error) -> Self {
        Self::Analyse(value)
    }
}

impl From<generate::Error> for Error {
    fn from(value: generate::Error) -> Self { Self::Generate(value) }
}

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub struct Ir {
    pub nodes: Vec<IrTreeNode>,
}

impl Index<usize> for Ir {
    type Output = IrTreeNode;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

impl Ir {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

pub fn ir_from_str(ctx: &mut Context, str: &str) -> Result<ir::Ir> {
    let ast = ast_from_str(ctx, str)?;
    let typed = analyse(ctx, ast)?;
    Ok(generate(ctx, typed)?)
}
