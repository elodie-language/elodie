use crate::generate::c::Indent;

#[derive(Debug)]
pub enum LiteralExpression {
    Double(LiteralDoubleExpression),
    Int(LiteralIntExpression),
    String(LiteralStringExpression),
}

#[derive(Debug)]
pub struct LiteralStringExpression {
    pub indent: Indent,
    pub value: String,
}

#[derive(Debug)]
pub struct LiteralIntExpression {
    pub indent: Indent,
    pub value: i32,
}

#[derive(Debug)]
pub struct LiteralDoubleExpression {
    pub indent: Indent,
    pub value: f64,
}
