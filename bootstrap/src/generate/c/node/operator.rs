use crate::generate::c::Expression;

#[derive(Debug)]
pub enum BinaryOperator {
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
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}