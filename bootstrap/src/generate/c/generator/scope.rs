use std::collections::HashMap;

use crate::common::{StringCache, StringCacheIdx};
use crate::ir::Identifier;

pub(crate) struct Scope {
    pub variables: Vec<HashMap<Identifier, Variable>>,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub identifier: Identifier,
    pub increment: u64,
}

impl Variable {
    pub fn to_string(&self, cache: &StringCache) -> String {
        format!("{}_{}", cache.get(self.identifier.0), self.increment)
    }
}


impl Scope {
    pub(crate) fn new() -> Self {
        let mut result = Self {
            variables: vec![]
        };
        result.enter();
        result
    }

    pub(crate) fn enter(&mut self) {
        self.variables.push(HashMap::new());
    }

    pub(crate) fn leave(&mut self) {
        self.variables.pop().unwrap();
    }

    pub(crate) fn get_variable(&self, identifier: &Identifier) -> Option<&Variable> {
        for scope in self.variables.iter().rev() {
            if let Some(value) = scope.get(identifier) {
                return Some(value);
            }
        }
        None
    }

    pub(crate) fn push_variable(&mut self, variable: Variable) {
        self.variables.last_mut().unwrap().insert(variable.identifier.clone(), variable);
    }
}