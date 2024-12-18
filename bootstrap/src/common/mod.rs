pub use package::PackagePath;
pub use r#type::{BaseType, DefaultTypeIds, Property, Type, TypeId, TypeName, TypeTable};
pub use string::{StringTable, StringTableId};
pub use symbol::{Symbol, SymbolId, SymbolName};
pub use util::*;

mod util;
mod string;
mod package;
mod r#type;
mod symbol;
mod context;


pub struct Context {
    pub string_table: StringTable,
    pub type_table: TypeTable,
}

impl Context {
    pub fn new() -> Self {
        Self {
            string_table: StringTable::new(),
            type_table: TypeTable::new(),
        }
    }
}

impl Context {
    pub fn get_str(&self, idx: StringTableId) -> &str { self.string_table.get(idx) }
}