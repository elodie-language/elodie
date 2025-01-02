use crate::build::c::Expression;

#[derive(Debug)]
pub struct DeclareStructNode {
    pub identifier: String,
}

#[derive(Debug)]
pub struct DefineStructNode {
    pub identifier: String,
    pub fields: Box<[DefineStructFieldNode]>,
}

#[derive(Debug)]
pub struct DefineStructFieldNode {
    pub identifier: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct InitialiseStructExpression {
    pub fields: Box<[InitialiseStructField]>,
}

#[derive(Debug)]
pub struct InitialiseStructField {
    pub identifier: String,
    pub expression: Expression,
}
