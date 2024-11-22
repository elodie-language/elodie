use std::collections::HashMap;
use crate::compile::symbol::Symbol;

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            scopes: Vec::new()
        }
    }
}
