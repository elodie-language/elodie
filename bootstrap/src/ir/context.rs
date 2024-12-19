use crate::common::{StringTable, StringTableId};
use crate::frontend;
use crate::frontend::Ast;
use crate::ir::r#type::TypeTable;

#[derive(Debug)]
pub struct Context {
    pub string_table: StringTable,
    pub type_table: TypeTable,
    pub ast: Ast,
}

impl Context {
    pub fn new(ctx: frontend::Context, ast: frontend::Ast) -> Self {
        Self {
            string_table: ctx.string_table,
            type_table: TypeTable::new(),
            ast,
        }
    }

    pub fn get_str(&self, idx: StringTableId) -> &str {
        self.string_table.get(idx)
    }
}
