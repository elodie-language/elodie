use std::collections::HashMap;

use crate::common::StringTable;
use crate::ir::Identifier;

pub(crate) struct Scope {
    pub variables: Vec<HashMap<Identifier, Variable>>,
    pub next_arguments: Vec<Argument>,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub identifier: Identifier,
    pub id: u64,
}

impl Variable {
    pub fn to_string(&self, cache: &StringTable) -> String {
        format!("{}_{}", cache.get(self.identifier.0), self.id)
    }
}

#[derive(Clone, Debug)]
pub struct Argument {
    pub id: u64,
}

impl Argument {
    pub fn to_string(&self) -> String {
        format!("arg_{}", self.id)
    }
}

impl Scope {
    pub(crate) fn new() -> Self {
        let mut result = Self {
            variables: vec![],
            next_arguments: vec![],
        };
        result.enter();
        result
    }

    pub(crate) fn enter(&mut self) {
        self.variables.push(HashMap::new());
        self.next_arguments.push(Argument { id: 1 })
    }

    pub(crate) fn leave(&mut self) {
        self.variables.pop().unwrap();
        self.next_arguments.pop().unwrap();
    }

    pub(crate) fn get_variable(&self, identifier: &Identifier) -> Option<&Variable> {
        for scope in self.variables.iter().rev() {
            if let Some(value) = scope.get(identifier) {
                return Some(value);
            }
        }
        None
    }

    pub(crate) fn push_variable(&mut self, identifier: &Identifier) -> Variable {
        let result = self.get_variable(&identifier).cloned()
            .map(|v| {
                Variable { identifier: identifier.clone(), id: v.id + 1 }
            })
            .unwrap_or(
                Variable { identifier: identifier.clone(), id: 1 }
            );

        self.variables.last_mut().unwrap().insert(result.identifier.clone(), result.clone());

        result
    }

    pub(crate) fn push_argument(&mut self) -> Argument {
        let next_arg = self.next_arguments.last_mut().unwrap();
        let result = next_arg.clone();

        next_arg.id += 1;

        result
    }
}