pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Bool(v) => v.to_string(),
            Value::Number(v) => v.to_string(),
            Value::String(v) => v.clone()
        }
    }
}