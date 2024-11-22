use std::collections::HashMap;
use std::rc::Rc;

use crate::r#type::Type;
use crate::run::value::{HostFunctionValue, ObjectValue, Value};
use crate::run::value::Value::HostFunction;

pub struct Scope {
    pub values: Vec<HashMap<String, Value>>,
    pub types: Vec<HashMap<String, Type>>,
}

impl Scope {
    pub fn new() -> Self {
        let mut result = Self {
            values: vec![],
            types: vec![],
        };

        let mut root = HashMap::new();

        let mut logger = ObjectValue::new();
        logger.set_property(
            "print",
            HostFunction(HostFunctionValue(Rc::new(|args: &[Value]| {
                for arg in args {
                    if arg.to_string() == "\\n" {
                        println!();
                    } else {
                        print!("{} ", arg.to_string());
                    }
                }
                Ok(Value::Unit)
            }))),
        );

        root.insert("intrinsics".to_string(), Value::Object(logger));

        result.values.push(root);

        result.types.push(HashMap::new());

        result
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        for scope in self.values.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn get_type(&self, key: &str) -> Option<&Type> {
        for scope in self.types.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn insert_value(&mut self, name: impl Into<String>, value: Value) {
        self.values.last_mut().unwrap().insert(name.into(), value);
    }

    pub fn insert_type(&mut self, name: impl Into<String>, r#type: Type) {
        self.types.last_mut().unwrap().insert(name.into(), r#type);
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

