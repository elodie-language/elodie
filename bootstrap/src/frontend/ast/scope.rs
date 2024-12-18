use std::collections::HashMap;

use crate::common::StringTableId;
use crate::common::TypeId;
use crate::frontend::ast::node::Identifier;

#[derive(Debug)]
pub struct Scope {
    pub types: Vec<HashMap<StringTableId, TypeId>>,
    pub identifiers: Vec<HashMap<Identifier, TypeId>>,
}

impl Scope {
    pub fn new() -> Self {
        let mut result = Self {
            types: vec![],
            identifiers: vec![],
        };
        result.enter();
        result
    }

    pub fn get_type(&self, key: &StringTableId) -> Option<TypeId> {
        for scope in self.types.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(*value);
            }
        }
        None
    }

    pub fn insert_type(&mut self, name: StringTableId, type_id: TypeId) {
        self.types.last_mut().unwrap().insert(name, type_id);
    }

    pub fn insert_identifier(&mut self, identifier: Identifier, type_id: TypeId) {
        self.identifiers.last_mut().unwrap().insert(identifier, type_id);
    }

    pub fn get_identifier_type(&self, identifier: &Identifier) -> Option<TypeId> {
        for scope in self.identifiers.iter().rev() {
            if let Some(value) = scope.get(identifier) {
                return Some(*value);
            }
        }
        None
    }

    pub fn enter(&mut self) {
        self.types.push(HashMap::new());
        self.identifiers.push(HashMap::new());
    }

    pub fn leave(&mut self) {
        self.types.pop().unwrap();
        self.identifiers.pop().unwrap();
    }
}