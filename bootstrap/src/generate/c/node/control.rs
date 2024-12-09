use crate::generate::c::{BlockStatement, Expression};

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub then: BlockStatement,
    pub otherwise: Option<BlockStatement>,
}