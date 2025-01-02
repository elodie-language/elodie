#[derive(Debug)]
pub enum LiteralExpression {
    Bool(LiteralBooleanExpression),
    Double(LiteralDoubleExpression),
    Int(LiteralIntExpression),
    String(LiteralStringExpression),
}

#[derive(Debug)]
pub struct LiteralBooleanExpression {
    pub value: bool,
}

#[derive(Debug)]
pub struct LiteralDoubleExpression {
    pub value: f64,
}

#[derive(Debug)]
pub struct LiteralIntExpression {
    pub value: i32,
}

#[derive(Debug)]
pub struct LiteralStringExpression {
    pub value: String,
}
