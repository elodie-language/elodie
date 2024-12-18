pub use modifier::*;

use crate::common::StringTable;
use crate::common::TypeTable;
use crate::frontend::{ast, Ast};

mod check;
mod infer;
mod modifier;
mod unify;

#[derive(Debug)]
pub struct Context {
    pub file: Ast,
    pub string_table: StringTable,
    pub type_table: TypeTable,
}
