#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
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
    Primary,
}
