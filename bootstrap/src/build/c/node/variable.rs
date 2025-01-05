use crate::build::c::Expression;

#[derive(Debug)]
pub struct DeclareArrayStatement {
    pub identifier: String,
    pub r#type: String,
    pub size: usize,
    // pub expression: Expression, some initialisation
}

#[derive(Debug)]
pub struct DeclareVariableStatement {
    pub variable: String,
    pub r#type: String,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct VariableExpression {
    pub variable: String,
    pub cast: Option<String>,
}

#[derive(Debug)]
pub struct DefineGlobalVariableNode {
    pub identifier: String,
    pub r#type: String,
    pub expression: Expression,
}
