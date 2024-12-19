use crate::common::{StringTable, StringTableId};
use crate::frontend;
use crate::ir::symbol::SymbolTable;
use crate::ir::TypeTable;

#[derive(Debug)]
pub struct Context {
    pub string_table: StringTable,
    pub symbol_table: SymbolTable,
    pub type_table: TypeTable,
}

impl Context {
    pub fn new(ctx: frontend::Context) -> Self {
        Self {
            string_table: ctx.string_table,
            symbol_table: SymbolTable::new(),
            type_table: TypeTable::new(),
        }
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
        Self {
            string_table: StringTable::new(),
            symbol_table: SymbolTable::new(),
            type_table: TypeTable::new(),
        }
    }
}