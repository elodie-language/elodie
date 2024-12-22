use crate::common::{StringTable, StringTableId};
use crate::ir::{BuiltinType, SymbolTable, TypeId, TypeTable};

#[derive(Debug)]
pub struct Context {
    pub string_table: StringTable,
    pub symbol_table: SymbolTable,
    pub type_table: TypeTable,
}

impl Context {
    pub fn new() -> Self { Self::default() }

    pub fn push_str(&mut self, s: &str) -> StringTableId {
        self.string_table.push_str(s)
    }
    pub fn get_str(&self, idx: StringTableId) -> &str {
        self.string_table.get(idx)
    }

    pub fn type_id_boolean(&self) -> TypeId { self.type_table.builtin(BuiltinType::Boolean) }
    pub fn type_id_number(&self) -> TypeId { self.type_table.builtin(BuiltinType::Number) }
    pub fn type_id_string(&self) -> TypeId { self.type_table.builtin(BuiltinType::String) }


    #[cfg(test)]
    pub fn testing() -> Self {
        let mut string_table = StringTable::new();
        let type_table = TypeTable::new(&mut string_table, 1);
        Self {
            string_table,
            symbol_table: SymbolTable::new(),
            type_table,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        let mut string_table = StringTable::new();
        let type_table = TypeTable::new(&mut string_table, 0); // FIXME seed needs to be random
        Self {
            string_table,
            symbol_table: SymbolTable::new(),
            type_table,
        }
    }
}
