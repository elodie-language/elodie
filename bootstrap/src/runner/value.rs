use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Debug)]
pub enum ValueType {
    Bool,
    HostFunction,
    Number,
    Object,
    String,
    Tuple,
    Unit,
}


#[derive(Debug)]
pub enum Value {
    Bool(bool),
    HostFunction(HostFunctionValue),
    Number(f64),
    Object(ObjectValue),
    String(String),
    Tuple(TupleValue),
    Unit,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Bool(v) => v.to_string(),
            Value::HostFunction(_) => "[HostFunction]".to_string(),
            Value::Number(v) => v.to_string(),
            Value::Object(_) => "[Object]".to_string(),
            Value::String(v) => v.clone(),
            Value::Tuple(_) => "[Tuple]".to_string(),
            Value::Unit => "Unit".to_string()
        }
    }
}


pub struct HostFunctionValue(pub Rc<dyn Fn(&[&Value]) -> crate::runner::Result<Value>>);

impl Debug for HostFunctionValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[HostFunction]")
    }
}

#[derive(Debug)]
pub struct ObjectValue {
    properties: HashMap<String, Value>,
}

impl ObjectValue {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new()
        }
    }

    pub fn set_property(&mut self, key: &str, value: Value) {
        self.properties.insert(key.to_string(), value);
    }

    pub fn get_property(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }
}

#[derive(Debug)]
pub struct TupleValue {
    values: Vec<Value>,
}