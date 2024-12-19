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
            type_table: TypeTable::new(),
            ast: Ast::from(vec![]),
        }
    }
}