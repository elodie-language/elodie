pub use control::*;
pub use directive::*;
pub use function::*;
pub use literal::*;
pub use operator::*;
pub use r#struct::*;
pub use variable::*;

mod control;
mod directive;
mod function;
mod literal;
mod modifier;
mod operator;
mod r#struct;
mod variable;

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
    Infix(InfixExpression),
    Literal(LiteralExpression),
    Variable(VariableExpression),
    StructInitialisation(InitialiseStructExpression),
}

#[derive(Debug)]
pub enum Statement {
    Block(BlockStatement),
    CallFunction(CallFunctionStatement),
    DeclareArray(DeclareArrayStatement),
    DeclareVariable(DeclareVariableStatement),
    If(IfStatement),
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
