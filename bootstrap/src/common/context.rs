use crate::common::{StringTable, StringTableId};
use crate::ir::{SymbolId, SymbolTable, TypeId, TypeTable};

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

    pub fn symbol_name(&self, id: SymbolId) -> &str { self.symbol_table[id].name_str(self) }
    pub fn symbol_type_id(&self, id: SymbolId) -> Option<TypeId> { self.symbol_table[id].type_id().clone() }

    pub fn symbol_is_boolean(&self, id: SymbolId) -> bool { self.symbol_type_id(id) == Some(self.type_id_boolean()) }
    pub fn symbol_is_number(&self, id: SymbolId) -> bool { self.symbol_type_id(id) == Some(self.type_id_number()) }
    pub fn symbol_is_string(&self, id: SymbolId) -> bool { self.symbol_type_id(id) == Some(self.type_id_string()) }

    pub fn type_id_boolean(&self) -> TypeId { self.type_table.type_id_boolean() }
    pub fn type_id_number(&self) -> TypeId { self.type_table.type_id_number() }
    pub fn type_id_string(&self) -> TypeId { self.type_table.type_id_string() }


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
