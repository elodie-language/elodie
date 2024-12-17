#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum Precedence {
    None,
    Assignment,
    LogicalOr,
    LogicalAnd,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    LambdaCall,
    Primary,
}
