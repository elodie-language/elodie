pub use control::*;
pub use directive::*;
pub use function::*;
pub use literal::*;
pub use operator::*;
pub use r#struct::*;
pub use variable::*;

use crate::build::c;
use crate::common::node::CompareOperator;

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
    Code(CodeNode),
    DeclareFunction(DeclareFunctionNode),
    DeclareStruct(DeclareStructNode),
    DefineFunction(DefineFunctionNode),
    DefineGlobalVariable(DefineGlobalVariableNode),
    DefineStruct(DefineStructNode),
    Directive(DirectiveNode),
}

#[derive(Debug)]
pub struct CodeNode {
    pub code: String,
}

#[derive(Debug)]
pub enum Expression {
    AccessVariableOfStruct(AccessVariableOfStructExpression),
    CallFunction(CallFunctionExpression),
    Code(CodeExpression),
    Compare(CompareExpression),
    Infix(InfixExpression),
    Literal(LiteralExpression),
    Variable(VariableExpression),
    StructInitialisation(InitialiseStructExpression),
}

impl Expression {
    pub fn compare_operator(op: &CompareOperator) -> c::Expression {
        match op {
            CompareOperator::Equal => {
                c::Expression::Code(CodeExpression { code: "COMPARE_OPERATOR_EQUAL".to_string() })
            }
            CompareOperator::NotEqual => {
                c::Expression::Code(CodeExpression { code: "COMPARE_OPERATOR_NOT_EQUAL".to_string() })
            }
            CompareOperator::GreaterThan => {
                c::Expression::Code(CodeExpression { code: "COMPARE_OPERATOR_GREATER_THAN".to_string() })
            }
            CompareOperator::GreaterThanEqual => {
                c::Expression::Code(CodeExpression { code: "COMPARE_OPERATOR_GREATER_THAN_EQUAL".to_string() })
            }
            CompareOperator::LessThan => {
                c::Expression::Code(CodeExpression { code: "COMPARE_OPERATOR_LESS_THAN".to_string() })
            }
            CompareOperator::LessThanEqual => {
                c::Expression::Code(CodeExpression { code: "COMPARE_OPERATOR_LESS_THAN_EQUAL".to_string() })
            }
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Block(BlockStatement),
    #[deprecated]
    CallFunction(CallFunctionStatement), // Use expression
    #[deprecated]
    Code(CodeStatement), // Use expressions
    DeclareArray(DeclareArrayStatement),
    DeclareVariable(DeclareVariableStatement),
    Expression(ExpressionStatement),
    If(IfStatement),
    ReturnFromFunction(ReturnFromFunctionStatement),
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub result: Option<ExpressionStatementResult>,
}

#[derive(Debug)]
pub struct ExpressionStatementResult {
    pub variable: String,
    pub r#type: String,
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct CodeStatement {
    pub code: String,
}

#[derive(Debug)]
pub struct CodeExpression {
    pub code: String,
}

#[derive(Debug)]
pub struct AccessVariableOfStructExpression {
    pub r#struct: String,
    pub variable: String,
}

#[derive(Debug)]
pub struct CompareExpression {
    pub left: Box<Expression>,
    pub operator: CompareOperator,
    pub right: Box<Expression>,
}