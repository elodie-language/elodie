pub use node::*;
pub use modifier::*;

use crate::backend::run::scope::Scope;
use crate::common::StringTable;
use crate::common::TypeTable;

mod node;
mod modifier;
pub mod compile;
mod infer;
mod check;
mod unify;

#[derive(Debug)]
pub struct Context {
    pub file: node::SourceFile,
    pub string_table: StringTable,
    pub type_table: TypeTable,

    // FIXME something which contains information about the core lib
    pub core_scope: Scope,
}
