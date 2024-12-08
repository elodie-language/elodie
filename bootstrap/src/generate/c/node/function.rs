use crate::generate::c::{BlockStatement, Expression, Indent};

#[derive(Debug)]
pub struct CallFunctionExpression {
    pub indent: Indent,
    pub identifier: String,
    pub arguments: Box<[Expression]>,
}


#[derive(Debug)]
pub struct CallFunctionStatement {
    pub indent: Indent,
    pub identifier: String,
    pub arguments: Box<[Expression]>,
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