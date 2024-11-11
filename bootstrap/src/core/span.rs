use crate::core::position::Position;

/// Represents a span of text between two positions, including the literal text.
#[derive(Debug,Clone)]
pub struct TextSpan {
    pub start: Position,
    pub end: Position,
    pub value: String,
}

impl TextSpan {
    pub fn new(start: Position, end: Position, value: String) -> Self {
        Self {
            start,
            end,
            value,
        }
    }
}