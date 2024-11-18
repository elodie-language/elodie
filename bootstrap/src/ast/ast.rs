use std::ops::Deref;

#[derive(Debug)]
pub struct SourceFile {
    // imports
    // exports
    pub body: Vec<Ast>,
}

#[derive(Debug)]
pub enum Ast {
    CallFunctionOfObject {
        object: ObjectIdentifier,
        function: FunctionIdentifier,
        arguments: Vec<Ast>,
    },
    NumberValue(f64),
    StringValue(String),
}

#[derive(Debug)]
pub struct ObjectIdentifier(pub String);

impl Deref for ObjectIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

#[derive(Debug)]
pub struct FunctionIdentifier(pub String);

impl Deref for FunctionIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl AsRef<FunctionIdentifier> for FunctionIdentifier{
    fn as_ref(&self) -> &FunctionIdentifier {
        &self
    }
}

pub enum Identifier {
    Function(FunctionIdentifier),
    Object(ObjectIdentifier),
}