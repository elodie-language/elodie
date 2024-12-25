use crate::build::c::{BlockStatement, Expression, Indent};

#[derive(Debug)]
pub struct CallFunctionStatement {
    pub indent: Indent,
    pub identifier: String,
    pub arguments: Box<[Expression]>,
    pub result: Option<CallFunctionStatementResult>,
}

#[derive(Debug)]
pub struct CallFunctionStatementResult {
    pub indent: Indent,
    pub identifier: String,
    pub r#type: String,
}

#[derive(Debug)]
pub struct DeclareFunctionNode {
    pub indent: Indent,
    pub identifier: String,
    pub arguments: Box<[DeclareFunctionArgumentNode]>,
    pub ty: String,
}

#[derive(Debug)]
pub struct DeclareFunctionArgumentNode {
    pub indent: Indent,
    pub identifier: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct DefineFunctionNode {
    pub indent: Indent,
    pub identifier: String,
    pub arguments: Box<[DefineFunctionArgumentNode]>,
    pub ty: String,
    pub statements: BlockStatement,
}

#[derive(Debug)]
pub struct DefineFunctionArgumentNode {
    pub indent: Indent,
    pub identifier: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct ReturnFromFunctionStatement {
    pub indent: Indent,
    pub node: Option<Expression>,
}
