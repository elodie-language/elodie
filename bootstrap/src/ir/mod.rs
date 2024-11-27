pub use ir::*;
pub use modifier::*;

mod ir;
mod modifier;

pub struct Context {
    pub node: ir::Node,
}