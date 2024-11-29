pub use ir::*;
pub use modifier::*;

use crate::common::StringCache;
use crate::run::scope::Scope;

mod ir;
mod modifier;

#[derive(Debug)]
pub struct Context {
    pub file: ir::SourceFile,
    pub string_cache: StringCache,

    // FIXME something which contains information about the core lib
    pub core_scope: Scope,
}