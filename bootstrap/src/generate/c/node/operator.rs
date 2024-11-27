use crate::generate::c::Expression;

#[derive(Debug)]
pub enum UnaryOperator {
    Add,       // +
    Subtract,  // -
    Multiply,  // *
    Divide,    // /
    Modulo,    // %
    Equals,    // ==
    NotEquals, // !=
    LessThan,  // <
    GreaterThan, // >
    And,       // &&
    Or,        // ||
    Assign,    // =
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub left: Box<Expression>,
    pub operator: UnaryOperator,
    pub right: Box<Expression>,
}