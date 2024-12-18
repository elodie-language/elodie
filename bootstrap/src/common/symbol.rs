use std::collections::HashMap;

use crate::common::StringTableId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

impl AsRef<SymbolId> for SymbolId {
    fn as_ref(&self) -> &SymbolId {
        &self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SymbolName(pub StringTableId);

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Argument { id: SymbolId, name: SymbolName },
    Function { id: SymbolId, name: SymbolName },
    Package { id: SymbolId, name: SymbolName },
    Type { id: SymbolId, name: SymbolName },
    Variable { id: SymbolId, name: SymbolName },
}

impl Symbol {}

#[derive(Debug)]
pub struct SymbolTable {
    next_id: usize,
    symbols: HashMap<SymbolId, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            symbols: HashMap::new(),
        }
    }

    pub fn register_argument(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.next_id);
        self.next_id += 1;

        self.symbols.insert(
            new_id.clone(),
            Symbol::Argument {
                id: new_id.clone(),
                name,
            },
        );

        new_id
    }

    pub fn register_function(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.next_id);
        self.next_id += 1;

        self.symbols.insert(
            new_id.clone(),
            Symbol::Function {
                id: new_id.clone(),
                name,
            },
        );

        new_id
    }

    pub fn register_package(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.next_id);
        self.next_id += 1;

        self.symbols.insert(
            new_id.clone(),
            Symbol::Package {
                id: new_id.clone(),
                name,
            },
        );

        new_id
    }

    pub fn register_type(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.next_id);
        self.next_id += 1;

        self.symbols.insert(
            new_id.clone(),
            Symbol::Type {
                id: new_id.clone(),
                name,
            },
        );

        new_id
    }

    pub fn register_variable(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.next_id);
        self.next_id += 1;

        self.symbols.insert(
            new_id.clone(),
            Symbol::Variable {
                id: new_id.clone(),
                name,
            },
        );

        new_id
    }
}
