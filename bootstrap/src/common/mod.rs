pub use package::PackagePath;
pub use strings::{StringTable, StringTableId};
pub use util::*;

use crate::r#type::TypeTable;

mod util;
mod strings;
mod package;


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