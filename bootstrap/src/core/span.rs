use crate::core::position::Position;

/// Represents a span of text between two positions, including the literal text.
#[derive(Debug)]
pub struct TextSpan {
    pub start: Position,
    pub end: Position,
    pub text: String,
}

impl TextSpan {
    pub fn new(start: Position, end: Position, text: String) -> Self {
        Self {
            start,
            end,
            text,
        }
    }
}