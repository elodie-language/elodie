#[derive(Debug)]
pub enum LiteralExpression {
    Bool(LiteralBooleanExpression),
    Float4(LiteralFloat4Expression),
    Float8(LiteralFloat8Expression),
    Int1(LiteralInt1Expression),
    Int2(LiteralInt2Expression),
    Int4(LiteralInt4Expression),
    Int8(LiteralInt8Expression),
    Int16(LiteralInt16Expression),
    String(LiteralStringExpression),
    Uint1(LiteralUint1Expression),
    Uint2(LiteralUint2Expression),
    Uint4(LiteralUint4Expression),
    Uint8(LiteralUint8Expression),
    Uint16(LiteralUint16Expression),
}

#[derive(Debug)]
pub struct LiteralBooleanExpression {
    pub value: bool,
}

#[derive(Debug)]
pub struct LiteralFloat4Expression {
    pub value: f32,
}

#[derive(Debug)]
pub struct LiteralFloat8Expression {
    pub value: f64,
}

#[derive(Debug)]
pub struct LiteralInt1Expression {
    pub value: i8,
}

#[derive(Debug)]
pub struct LiteralInt2Expression {
    pub value: i16,
}

#[derive(Debug)]
pub struct LiteralInt4Expression {
    pub value: i32,
}

#[derive(Debug)]
pub struct LiteralInt8Expression {
    pub value: i64,
}

#[derive(Debug)]
pub struct LiteralInt16Expression {
    pub value: i128,
}


#[derive(Debug)]
pub struct LiteralStringExpression {
    pub value: String,
}

#[derive(Debug)]
pub struct LiteralUint1Expression {
    pub value: u8,
}

#[derive(Debug)]
pub struct LiteralUint2Expression {
    pub value: u16,
}

#[derive(Debug)]
pub struct LiteralUint4Expression {
    pub value: u32,
}

#[derive(Debug)]
pub struct LiteralUint8Expression {
    pub value: u64,
}

#[derive(Debug)]
pub struct LiteralUint16Expression {
    pub value: u128,
}
