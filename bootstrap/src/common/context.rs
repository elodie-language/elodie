use crate::common::{StringTable, StringTableId, TypeId, TypeTable};
use crate::common::symbol::{SymbolId, SymbolTable};

#[derive(Debug)]
pub struct Context {
    pub string_table: StringTable,
    pub symbol_table: SymbolTable,
    pub type_table: TypeTable,
}

impl Context {
    pub fn new() -> Self { Self::default() }

    pub fn str_push(&mut self, s: &str) -> StringTableId {
        self.string_table.push_str(s)
    }
    pub fn str_get(&self, idx: StringTableId) -> &str {
        self.string_table.get(idx)
    }

    pub fn symbol_name(&self, id: SymbolId) -> &str { self.symbol_table[id].name_str(&self.string_table) }
    pub fn symbol_type_id(&self, id: SymbolId) -> Option<TypeId> { self.symbol_table[id].type_id().clone() }

    pub fn symbol_is_boolean(&self, id: SymbolId) -> bool { self.symbol_type_id(id) == Some(TypeId::BOOLEAN) }
    pub fn symbol_is_number(&self, id: SymbolId) -> bool { self.symbol_type_id(id) == Some(TypeId::NUMBER) }
    pub fn symbol_is_string(&self, id: SymbolId) -> bool { self.symbol_type_id(id) == Some(TypeId::STRING) }

    #[cfg(test)]
    pub fn testing() -> Self {
        let mut string_table = StringTable::new();
        let type_table = TypeTable::new(&mut string_table);
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
        let type_table = TypeTable::new(&mut string_table);
        Self {
            string_table,
            symbol_table: SymbolTable::new(),
            type_table,
        }
    }
}
