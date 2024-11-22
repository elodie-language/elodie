use std::collections::HashMap;

use crate::common::StringCacheIdx;
use crate::r#type::Type;
use crate::run::value::Value;

pub struct Scope {
    pub values: Vec<HashMap<StringCacheIdx, Value>>,
    pub types: Vec<HashMap<StringCacheIdx, Type>>,
}

impl Scope {
    pub fn new(
        root_values: HashMap<StringCacheIdx, Value>,
        root_types: HashMap<StringCacheIdx, Type>,
    ) -> Self {
        Self {
            values: vec![root_values],
            types: vec![root_types],
        }
    }

    pub fn get_value(&self, key: &StringCacheIdx) -> Option<&Value> {
        for scope in self.values.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn get_type(&self, key: &StringCacheIdx) -> Option<&Type> {
        for scope in self.types.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn insert_value(&mut self, name: StringCacheIdx, value: Value) {
        self.values.last_mut().unwrap().insert(name, value);
    }

    pub fn insert_type(&mut self, name: StringCacheIdx, r#type: Type) {
        self.types.last_mut().unwrap().insert(name, r#type);
    }

    pub fn enter(&mut self) {
        self.values.push(HashMap::new());
        self.types.push(HashMap::new());
    }

    pub fn leave(&mut self) {
        self.values.pop().unwrap();
        self.types.pop().unwrap();
    }
}

