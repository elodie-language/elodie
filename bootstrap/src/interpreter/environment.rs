use std::collections::HashMap;
use std::rc::Rc;

use crate::interpreter::value::{Function, Object, Value};

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Self {
            values: HashMap::new(),
        };
        
        let mut console = Object::new();
        console.set_property(
            "log",
            Value::Function(Function(Rc::new(|args: &[Value]| {
                for arg in args {
                    print!("{} ", arg.to_string());
                }
                println!();
                Value::Unit
            }))),
        );

        env.values.insert("console".to_string(), Value::Object(console));

        env
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
}
