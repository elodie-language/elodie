use std::ops::{Index, IndexMut};

use crate::common::{StringTableId, TypeId};
use crate::common::context::Context;
use crate::common::package::PackageId;
use crate::frontend::ast;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

impl AsRef<SymbolId> for SymbolId {
    fn as_ref(&self) -> &SymbolId {
        &self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SymbolName(pub StringTableId);

impl From<&ast::AstIdentifier> for SymbolName {
    fn from(value: &ast::AstIdentifier) -> Self {
        SymbolName(value.0)
    }
}

pub trait SymbolInner {
    fn id(&self) -> SymbolId;
    fn name(&self) -> SymbolName;
    fn name_str<'a>(&self, ctx: &'a Context) -> &'a str;
    fn type_id(&self) -> Option<TypeId>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Argument(ArgumentSymbol),
    Function(FunctionSymbol),
    Package(PackageSymbol),
    Type(TypeSymbol),
    Variable(VariableSymbol),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentSymbol {
    pub id: SymbolId,
    pub name: SymbolName,
    pub type_id: Option<TypeId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSymbol {
    pub id: SymbolId,
    pub name: SymbolName,
    pub type_id: Option<TypeId>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct PackageSymbol {
    pub id: SymbolId,
    pub name: SymbolName,
    pub package_id: Option<PackageId>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct TypeSymbol {
    pub id: SymbolId,
    pub name: SymbolName,
    pub type_id: Option<TypeId>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct VariableSymbol {
    pub id: SymbolId,
    pub name: SymbolName,
    pub type_id: Option<TypeId>,
}

impl Symbol {
    pub fn id(&self) -> SymbolId {
        match self {
            Symbol::Argument(inner) => inner.id,
            Symbol::Function(inner) => inner.id,
            Symbol::Package(inner) => inner.id,
            Symbol::Type(inner) => inner.id,
            Symbol::Variable(inner) => inner.id
        }
    }
    pub fn name(&self) -> SymbolName {
        match self {
            Symbol::Argument(inner) => inner.name,
            Symbol::Function(inner) => inner.name,
            Symbol::Package(inner) => inner.name,
            Symbol::Type(inner) => inner.name,
            Symbol::Variable(inner) => inner.name
        }
    }
    pub fn name_str<'a>(&self, ctx: &'a Context) -> &'a str {
        match self {
            Symbol::Argument(inner) => ctx.str_get(inner.name.0),
            Symbol::Function(inner) => ctx.str_get(inner.name.0),
            Symbol::Package(inner) => ctx.str_get(inner.name.0),
            Symbol::Type(inner) => ctx.str_get(inner.name.0),
            Symbol::Variable(inner) => ctx.str_get(inner.name.0)
        }
    }
    pub fn type_id(&self) -> Option<TypeId> {
        match self {
            Symbol::Argument(inner) => inner.type_id,
            Symbol::Function(inner) => inner.type_id,
            Symbol::Package(inner) => unreachable!(),
            Symbol::Type(inner) => inner.type_id,
            Symbol::Variable(inner) => inner.type_id
        }
    }

    pub fn set_type_id(&mut self, type_id: TypeId) {
        match self {
            Symbol::Argument(inner) => inner.type_id = Some(type_id),
            Symbol::Function(inner) => inner.type_id = Some(type_id),
            Symbol::Package(_) => unreachable!(),
            Symbol::Type(inner) => inner.type_id = Some(type_id),
            Symbol::Variable(inner) => inner.type_id = Some(type_id)
        }
    }

    pub fn set_package_id(&mut self, package_id: PackageId) {
        match self {
            Symbol::Package(inner) => inner.package_id = Some(package_id),
            _ => unreachable!()
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
        self.symbols.push(Symbol::Argument(ArgumentSymbol {
            id: new_id.clone(),
            name,
            type_id: None,
        }));
        new_id
    }

    pub(crate) fn register_function(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.len() + 1);
        self.symbols.push(Symbol::Function(FunctionSymbol {
            id: new_id.clone(),
            name,
            type_id: None,
        }));
        new_id
    }

    pub(crate) fn register_package(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.len() + 1);
        self.symbols.push(Symbol::Package(PackageSymbol {
            id: new_id.clone(),
            name,
            package_id: None,
        }));
        new_id
    }

    pub(crate) fn register_type(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.len() + 1);
        self.symbols.push(Symbol::Type(TypeSymbol {
            id: new_id.clone(),
            name,
            type_id: None,
        }));
        new_id
    }

    pub(crate) fn register_variable(&mut self, name: SymbolName) -> SymbolId {
        let new_id = SymbolId(self.len() + 1);
        self.symbols.push(Symbol::Variable(VariableSymbol {
            id: new_id.clone(),
            name,
            type_id: None,
        }));
        new_id
    }
}

impl Index<SymbolId> for SymbolTable {
    type Output = Symbol;

    fn index(&self, index: SymbolId) -> &Self::Output {
        self.index(index.0)
    }
}

impl IndexMut<SymbolId> for SymbolTable {
    fn index_mut(&mut self, index: SymbolId) -> &mut Self::Output {
        self.index_mut(index.0)
    }
}

impl Index<usize> for SymbolTable {
    type Output = Symbol;

    fn index(&self, index: usize) -> &Self::Output {
        &self.symbols[index - 1]
    }
}

impl IndexMut<usize> for SymbolTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.symbols[index - 1]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_argument() {
        let mut ctx = Context::testing();
        let mut table = SymbolTable::new();

        let id = table.register_argument(SymbolName(ctx.str_push("argument")));
        assert_eq!(id, SymbolId(1));
        assert_eq!(table.len(), 1);

        let symbol = &table[1];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "argument");
    }

    #[test]
    fn register_function() {
        let mut ctx = Context::testing();
        let mut table = SymbolTable::new();

        let id = table.register_function(SymbolName(ctx.str_push("function")));
        assert_eq!(id, SymbolId(1));
        assert_eq!(table.len(), 1);

        let symbol = &table[1];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "function");
    }

    #[test]
    fn register_package() {
        let mut ctx = Context::testing();
        let mut table = SymbolTable::new();

        let id = table.register_package(SymbolName(ctx.str_push("package")));
        assert_eq!(id, SymbolId(1));
        assert_eq!(table.len(), 1);

        let symbol = &table[1];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "package");
    }

    #[test]
    fn register_type() {
        let mut ctx = Context::testing();
        let mut table = SymbolTable::new();

        let id = table.register_type(SymbolName(ctx.str_push("type")));
        assert_eq!(id, SymbolId(1));
        assert_eq!(table.len(), 1);

        let symbol = &table[1];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "type");
    }

    #[test]
    fn register_variable() {
        let mut ctx = Context::testing();
        let mut table = SymbolTable::new();

        let id = table.register_variable(SymbolName(ctx.str_push("variable")));
        assert_eq!(id, SymbolId(1));
        assert_eq!(table.len(), 1);

        let symbol = &table[1];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "variable");
    }

    #[test]
    fn register_multiple_symbols() {
        let mut ctx = Context::testing();
        let mut table = SymbolTable::new();

        let arg_id = table.register_argument(SymbolName(ctx.str_push("argument")));
        let func_id = table.register_function(SymbolName(ctx.str_push("function")));
        let var_id = table.register_variable(SymbolName(ctx.str_push("variable")));

        assert_eq!(arg_id, SymbolId(1));
        assert_eq!(func_id, SymbolId(2));
        assert_eq!(var_id, SymbolId(3));

        assert_eq!(table.len(), 3);

        let symbol = &table[1];
        assert_eq!(symbol.id(), SymbolId(1));
        assert_eq!(symbol.name_str(&ctx), "argument");

        let symbol = &table[2];
        assert_eq!(symbol.id(), SymbolId(2));
        assert_eq!(symbol.name_str(&ctx), "function");

        let symbol = &table[3];
        assert_eq!(symbol.id(), SymbolId(3));
        assert_eq!(symbol.name_str(&ctx), "variable");
    }
}
