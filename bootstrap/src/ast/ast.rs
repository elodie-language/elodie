use crate::core::Value;

#[derive(Debug)]
pub struct SourceFile {
    // imports
    // exports
    pub body: Vec<Node>,
}

#[derive(Debug)]
pub enum Node {
    CallFunctionOfObject(CallFunctionOfObjectNode),
    Value(Value),
}

#[derive(Debug)]
pub struct CallFunctionOfObjectNode {
    pub object: String,
    pub function: String,
    pub arguments: Vec<Node>,
}