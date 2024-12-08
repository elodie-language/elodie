pub use ir::*;
pub use modifier::*;

use crate::common::StringTable;
use crate::r#type::TypeTable;
use crate::run::scope::Scope;

mod ir;
mod modifier;

#[derive(Debug)]
pub struct Context {
    pub file: ir::SourceFile,
    pub string_table: StringTable,
    pub type_table: TypeTable,

    // FIXME something which contains information about the core lib
    pub core_scope: Scope,
}