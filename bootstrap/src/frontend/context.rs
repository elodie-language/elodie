use crate::common::{StringTable, StringTableId};

#[derive(Debug)]
pub struct Context {
    pub string_table: StringTable,
}

impl Context {
    pub fn new() -> Self {
        Self {
            string_table: StringTable::new(),
        }
    }

    pub fn get_str(&self, idx: StringTableId) -> &str {
        self.string_table.get(idx)
    }
}
