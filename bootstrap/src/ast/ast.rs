#[derive(Debug)]
pub struct SourceFile {
    // imports
    // exports
    pub body: Vec<Ast>,
}

#[derive(Debug)]
pub enum Ast {
    CallFunctionOfObject {
        object: String,
        function: String,
        arguments: Vec<Ast>,
    },
    StringValue(String),
}
