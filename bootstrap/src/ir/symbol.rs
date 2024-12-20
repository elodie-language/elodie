use std::ops::Index;

use crate::common::StringTableId;
use crate::frontend::ast;
use crate::ir::Context;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

impl AsRef<SymbolId> for SymbolId {
    fn as_ref(&self) -> &SymbolId {
        &self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SymbolName(pub StringTableId);

impl From<&ast::Identifier> for SymbolName {
    fn from(value: &ast::Identifier) -> Self {
        SymbolName(value.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Argument { id: SymbolId, name: SymbolName },
    Function { id: SymbolId, name: SymbolName },
    Package { id: SymbolId, name: SymbolName },
    Type { id: SymbolId, name: SymbolName },
    Variable { id: SymbolId, name: SymbolName },
}

impl Symbol {
    pub fn id(&self) -> SymbolId {
        match self {
            Symbol::Argument { id, .. } => id.clone(),
            Symbol::Function { id, .. } => id.clone(),
            Symbol::Package { id, .. } => id.clone(),
            Symbol::Type { id, .. } => id.clone(),
            Symbol::Variable { id, .. } => id.clone()
        }
    }

    pub fn name(&self, ctx: &Context) -> SymbolName {
        match self {
            Symbol::Argument { name, .. } => name.clone(),
            Symbol::Function { name, .. } => name.clone(),
            Symbol::Package { name, .. } => name.clone(),
            Symbol::Type { name, .. } => name.clone(),
            Symbol::Variable { name, .. } => name.clone(),
        }
    }

    pub fn name_str<'a>(&self, ctx: &'a Context) -> &'a str {
        match self {
            Symbol::Argument { name, .. } => ctx.get_str(name.0),
            Symbol::Function { name, .. } => ctx.get_str(name.0),
            Symbol::Package { name, .. } => ctx.get_str(name.0),
            Symbol::Type { name, .. } => ctx.get_str(name.0),
            Symbol::Variable { name, .. } => ctx.get_str(name.0),
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub(crate) fn register_argument(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.len() + 1);
        self.symbols.push(Symbol::Argument { id: new_id.clone(), name });
        new_id
    }

    pub(crate) fn register_function(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.len() + 1);
        self.symbols.push(Symbol::Function { id: new_id.clone(), name });
        new_id
    }

    pub(crate) fn register_package(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.len() + 1);
        self.symbols.push(Symbol::Package { id: new_id.clone(), name });
        new_id
    }

    pub(crate) fn register_type(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.len() + 1);
        self.symbols.push(Symbol::Type { id: new_id.clone(), name });
        new_id
    }

    pub(crate) fn register_variable(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.len() + 1);
        self.symbols.push(Symbol::Variable { id: new_id.clone(), name });
        new_id
    }
}

impl Index<SymbolId> for SymbolTable {
    type Output = Symbol;

    fn index(&self, index: SymbolId) -> &Self::Output {
        self.index(index.0)
    }
}

impl Index<usize> for SymbolTable {
    type Output = Symbol;

    fn index(&self, index: usize) -> &Self::Output {
        &self.symbols[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::Context;

    use super::*;

    #[test]
    fn register_argument() {
        let mut ctx = Context::default();
        let mut table = SymbolTable::new();

        let id = table.register_argument(SymbolName(ctx.push_str("argument")));
        assert_eq!(id, SymbolId(1));
        assert_eq!(table.len(), 1);

        let symbol = &table[0];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "argument");
    }

    #[test]
    fn register_function() {
        let mut ctx = Context::default();
        let mut table = SymbolTable::new();

        let id = table.register_function(SymbolName(ctx.push_str("function")));
        assert_eq!(id, SymbolId(1));
        assert_eq!(table.len(), 1);

        let symbol = &table[0];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "function");
    }

    #[test]
    fn register_package() {
        let mut ctx = Context::default();
        let mut table = SymbolTable::new();

        let id = table.register_package(SymbolName(ctx.push_str("package")));
        assert_eq!(id, SymbolId(1));
        assert_eq!(table.len(), 1);

        let symbol = &table[0];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "package");
    }

    #[test]
    fn register_type() {
        let mut ctx = Context::default();
        let mut table = SymbolTable::new();

        let id = table.register_type(SymbolName(ctx.push_str("type")));
        assert_eq!(id, SymbolId(1));
        assert_eq!(table.len(), 1);

        let symbol = &table[0];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "type");
    }

    #[test]
    fn register_variable() {
        let mut ctx = Context::default();
        let mut table = SymbolTable::new();

        let id = table.register_variable(SymbolName(ctx.push_str("variable")));
        assert_eq!(id, SymbolId(1));
        assert_eq!(table.len(), 1);

        let symbol = &table[0];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "variable");
    }


    #[test]
    fn register_multiple_symbols() {
        let mut ctx = Context::default();
        let mut table = SymbolTable::new();

        let arg_id = table.register_argument(SymbolName(ctx.push_str("argument")));
        let func_id = table.register_function(SymbolName(ctx.push_str("function")));
        let var_id = table.register_variable(SymbolName(ctx.push_str("variable")));

        assert_eq!(arg_id, SymbolId(1));
        assert_eq!(func_id, SymbolId(2));
        assert_eq!(var_id, SymbolId(3));

        assert_eq!(table.len(), 3);

        let symbol = &table[0];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "argument");

        let symbol = &table[1];
        assert_eq!(symbol.id(), SymbolId(2));
        assert_eq!(symbol.name_str(&ctx), "function");

        let symbol = &table[2];
        assert_eq!(symbol.id(), SymbolId(3));
        assert_eq!(symbol.name_str(&ctx), "variable");
    }
}