use crate::common::{StringTable, TypeTable};
use crate::common::symbol::SymbolTable;

#[derive(Debug)]
pub struct Context {
    pub string_table: StringTable,
    pub symbol_table: SymbolTable,
    pub type_table: TypeTable,
}
