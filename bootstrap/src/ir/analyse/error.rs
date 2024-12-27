use crate::common::Span;

#[derive(Debug, PartialEq)]
pub enum Error {
    TypeMissMatch(TypeMissMatchError),
    Undefined(UndefinedError),

}

#[derive(Debug, PartialEq)]
pub enum TypeMissMatchError {
    DeclaredTypeMissMatch { expected: String, got: String, span: Span }
}

#[derive(Debug, PartialEq)]
pub enum UndefinedError {
    UndefinedVariable { variable: String, span: Span }
}
