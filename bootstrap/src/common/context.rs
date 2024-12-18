use crate::common::symbol::SymbolTable;
use crate::common::{StringTable, StringTableId, TypeTable};

#[derive(Debug)]
pub struct Context {
    pub string_table: StringTable,
    pub symbol_table: SymbolTable,
    pub type_table: TypeTable,
}

impl Context {
    pub fn new() -> Self {
        Self {
            string_table: StringTable::new(),
            symbol_table: SymbolTable::new(),
            type_table: TypeTable::new(),
        }
    }

    pub fn get_str(&self, idx: StringTableId) -> &str {
        self.string_table.get(idx)
    }
}
