use crate::common::Span;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidLiteral(InvalidLiteralError),
    TypeMissMatch(TypeMissMatchError),
    Undefined(UndefinedError),

}

#[derive(Debug, PartialEq)]
pub enum InvalidLiteralError {
    Float4 { got: String, span: Span },
    Float8 { got: String, span: Span },
    Int1 { got: String, span: Span },
    Int2 { got: String, span: Span },
    Int4 { got: String, span: Span },
    Int8 { got: String, span: Span },
    Int16 { got: String, span: Span },
    Uint1 { got: String, span: Span },
    Uint2 { got: String, span: Span },
    Uint4 { got: String, span: Span },
    Uint8 { got: String, span: Span },
    Uint16 { got: String, span: Span },
}

#[derive(Debug, PartialEq)]
pub enum TypeMissMatchError {
    DeclaredTypeMissMatch { expected: String, got: String, span: Span }
}

#[derive(Debug, PartialEq)]
pub enum UndefinedError {
    UndefinedVariable { variable: String, span: Span }
}
