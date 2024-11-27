use crate::generate::c::{Expression, Indent, Node};

#[derive(Debug)]
pub struct AssignVariableStatement {
    pub indent: Indent,
    pub identifier: String,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct DefineGlobalVariableNode {
    pub indent: Indent,
    pub identifier: String,
    pub expression: Expression,
}