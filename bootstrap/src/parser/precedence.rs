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

impl Precedence {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Precedence::None => 0,
            Precedence::Assignment => 1,
            Precedence::LogicalOr => 2,
            Precedence::LogicalAnd => 3,
            Precedence::Equality => 4,
            Precedence::Comparison => 5,
            Precedence::Term => 6,
            Precedence::Factor => 7,
            Precedence::Unary => 8,
            Precedence::Call => 9,
            Precedence::Primary => 10,
        }
    }
}