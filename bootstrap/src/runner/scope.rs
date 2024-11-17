use std::collections::HashMap;
use std::rc::Rc;
use Value::HostFunction;

use crate::core::{HostFunctionValue, ObjectValue, Value};

pub struct Scope {
    pub values: Vec<HashMap<String, Value>>,
}

impl Scope {
    pub fn new() -> Self {
        let mut result = Self {
            values: vec![],
        };

        let mut root = HashMap::new();

        let mut console = ObjectValue::new();
        console.set_property(
            "log",
            HostFunction(HostFunctionValue(Rc::new(|args: &[&Value]| {
                for arg in args {
                    print!("{} ", arg.to_string());
                }
                println!();
                Ok(Value::Unit)
            }))),
        );

        root.insert("console".to_string(), Value::Object(console));

        result.values.push(root);

        result
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        for scope in self.values.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn insert(&mut self, name: impl Into<String>, value: Value) {
        self.values.last_mut().unwrap().insert(name.into(), value);
    }

    pub fn enter(&mut self) {
        self.values.push(HashMap::new());
    }

    pub fn leave(&mut self) {
        self.values.pop().unwrap();
    }
}

