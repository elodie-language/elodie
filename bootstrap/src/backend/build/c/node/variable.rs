use crate::backend::build::c::{Expression, Indent};

#[derive(Debug)]
pub struct DeclareArrayStatement {
    pub indent: Indent,
    pub identifier: String,
    pub r#type: String,
    pub size: usize,
    // pub expression: Expression, some initialisation
}

#[derive(Debug)]
pub struct DeclareVariableStatement {
    pub indent: Indent,
    pub identifier: String,
    pub r#type: String,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct VariableExpression {
    pub indent: Indent,
    pub identifier: String,
}

#[derive(Debug)]
pub struct DefineGlobalVariableNode {
    pub indent: Indent,
    pub identifier: String,
    pub r#type: String,
    pub expression: Expression,
}
