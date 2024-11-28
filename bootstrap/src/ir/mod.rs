pub use ir::*;
pub use modifier::*;
use crate::common::StringCache;

mod ir;
mod modifier;

#[derive(Debug)]
pub struct Context {
    pub file: ir::SourceFile,
    pub string_cache: StringCache
}