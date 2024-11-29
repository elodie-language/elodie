pub use directive::*;
pub use function::*;
pub use literal::*;
pub use operator::*;
pub use r#struct::*;
pub use variable::*;

mod directive;
mod modifier;
mod r#struct;
mod function;
mod control;
mod literal;
mod variable;
mod operator;

#[derive(Debug)]
pub enum Node {
    DeclareFunction(DeclareFunctionNode),
    DeclareStruct(DeclareStructNode),
    DefineFunction(DefineFunctionNode),
    DefineGlobalVariable(DefineGlobalVariableNode),
    DefineStruct(DefineStructNode),
    Directive(DirectiveNode),
}

#[derive(Debug)]
pub enum Expression {
    CallFunction(CallFunctionExpression),
    Literal(LiteralExpression),
    Unary(UnaryExpression),
    Variable(VariableExpression),
}


#[derive(Debug)]
pub enum Statement {
    Block(BlockStatement),
    DeclareArray(DeclareArrayStatement),
    DeclareVariable(DeclareVariableStatement),
    Expression(Expression),
    ReturnFromFunction(ReturnFromFunctionStatement),
}

#[derive(Debug)]
pub struct BlockStatement {
    pub indent: Indent,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Indent(pub usize);

impl Indent {
    pub fn none() -> Self {
        Indent(0)
    }
}
