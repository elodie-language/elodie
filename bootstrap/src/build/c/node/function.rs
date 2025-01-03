use crate::build::c::{BlockStatement, Expression};

#[derive(Debug)]
pub struct CallFunctionStatement {
    pub function: String,
    pub arguments: Box<[Expression]>,
    pub result: Option<CallFunctionStatementResult>,
}

#[derive(Debug)]
pub struct CallFunctionExpression {
    pub function: String,
    pub arguments: Box<[Expression]>,
}


#[derive(Debug)]
pub struct CallFunctionStatementResult {
    pub identifier: String,
    pub r#type: String,
}

#[derive(Debug)]
pub struct DeclareFunctionNode {
    pub identifier: String,
    pub arguments: Box<[DeclareFunctionArgumentNode]>,
    pub ty: String,
}

#[derive(Debug)]
pub struct DeclareFunctionArgumentNode {
    pub identifier: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct DefineFunctionNode {
    pub identifier: String,
    pub arguments: Box<[DefineFunctionArgumentNode]>,
    pub ty: String,
    pub block: BlockStatement,
}

#[derive(Debug)]
pub struct DefineFunctionArgumentNode {
    pub identifier: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct ReturnFromFunctionStatement {
    pub node: Option<Expression>,
}
