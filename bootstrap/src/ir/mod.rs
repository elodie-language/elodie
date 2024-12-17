pub use ir::*;
pub use modifier::*;

use crate::backend::run::scope::Scope;
use crate::common::StringTable;
use crate::common::TypeTable;

mod ir;
mod modifier;
pub mod compile;

#[derive(Debug)]
pub struct Context {
    pub file: ir::SourceFile,
    pub string_table: StringTable,
    pub type_table: TypeTable,

    // FIXME something which contains information about the core lib
    pub core_scope: Scope,
}
