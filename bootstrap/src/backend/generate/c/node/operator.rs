use crate::backend::generate::c::Expression;

#[derive(Debug)]
pub enum InfixOperator {
    Add,         // +
    Subtract,    // -
    Multiply,    // *
    Divide,      // /
    Modulo,      // %
    Equal,       // ==
    NotEqual,    // !=
    LessThan,    // <
    GreaterThan, // >
    Assign,      // =
}

#[derive(Debug)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: InfixOperator,
    pub right: Box<Expression>,
}
