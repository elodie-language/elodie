use crate::common::{StringTable, StringTableId};
use crate::ir::{SymbolTable, TypeTable};

#[derive(Debug)]
pub struct Context {
    pub string_table: StringTable,
    pub symbol_table: SymbolTable,
    pub type_table: TypeTable,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_str(&mut self, s: &str) -> StringTableId {
        self.string_table.push_str(s)
    }

    pub fn get_str(&self, idx: StringTableId) -> &str {
        self.string_table.get(idx)
    }
}

impl Default for Context {
    fn default() -> Self {
        let mut string_table = StringTable::new();
        let type_table = TypeTable::new(&mut string_table, 0);
        Self {
            string_table,
            symbol_table: SymbolTable::new(),
            type_table,
        }
    }
}
