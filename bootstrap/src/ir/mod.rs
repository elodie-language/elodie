use std::ops::Index;

pub use r#type::{Type, TypeId, TypeName, TypeTable, TypeVariable};
pub use context::Context;

use crate::{frontend, ir};
use crate::frontend::new_ast_from_str;

use crate::ir::analyse::analyse;
use crate::ir::generate::generate;

mod analyse;
pub(crate) mod node;
mod symbol;
mod r#type;
mod generate;
mod context;

#[derive(Debug)]
pub enum Error {
    Frontend(frontend::Error),
    Analyse(analyse::Error),
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

pub(crate) type Result<T, E = Error> = core::result::Result<T, E>;

pub struct Ir {
    pub nodes: Vec<ir::node::Node>,
}

impl Index<usize> for Ir {
    type Output = node::Node;
    fn index(&self, index: usize) -> &Self::Output {
        self.nodes.index(index)
    }
}

impl Ir {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

pub fn ir_from_str(str: &str) -> Result<ir::Ir> {
    let mut ctx = frontend::Context::new();
    let ast = new_ast_from_str(&mut ctx, str)?;

    let mut ctx = ir::Context::new(ctx);
    let analysed = analyse(&mut ctx, ast)?;

    // let mut ctx = generate::Context::new(ctx, analysed);
    // generate(&mut ctx, &analysed);
    todo!()
}
