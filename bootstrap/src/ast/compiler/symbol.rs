use std::collections::HashMap;
use std::hash::Hash;

pub(crate) enum Symbol {}

pub(crate) struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            scopes: Vec::new()
        }
    }
}

