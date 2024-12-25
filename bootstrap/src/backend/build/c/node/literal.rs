use crate::backend::build::c::Indent;

#[derive(Debug)]
pub enum LiteralExpression {
    Bool(LiteralBooleanExpression),
    Double(LiteralDoubleExpression),
    Int(LiteralIntExpression),
    String(LiteralStringExpression),
}

#[derive(Debug)]
pub struct LiteralBooleanExpression {
    pub indent: Indent,
    pub value: bool,
}

#[derive(Debug)]
pub struct LiteralDoubleExpression {
    pub indent: Indent,
    pub value: f64,
}

#[derive(Debug)]
pub struct LiteralIntExpression {
    pub indent: Indent,
    pub value: i32,
}

#[derive(Debug)]
pub struct LiteralStringExpression {
    pub indent: Indent,
    pub value: String,
}
