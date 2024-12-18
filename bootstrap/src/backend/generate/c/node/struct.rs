use crate::backend::generate::c::{Expression, Indent};

#[derive(Debug)]
pub struct DeclareStructNode {
    pub indent: Indent,
    pub identifier: String,
}

#[derive(Debug)]
pub struct DefineStructNode {
    pub indent: Indent,
    pub identifier: String,
    pub fields: Box<[DefineStructFieldNode]>,
}

#[derive(Debug)]
pub struct DefineStructFieldNode {
    pub indent: Indent,
    pub identifier: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct InitialiseStructExpression {
    pub fields: Box<[InitialiseStructField]>,
}

#[derive(Debug)]
pub struct InitialiseStructField {
    pub indent: Indent,
    pub identifier: String,
    pub expression: Expression,
}
