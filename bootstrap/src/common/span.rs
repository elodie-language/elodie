#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self {
            start: Position::default(),
            end: Position::default(),
        }
    }
}

pub trait WithSpan {
    fn span(&self) -> Span;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Row(pub usize);

impl PartialEq<usize> for Row {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Column(pub usize);

impl PartialEq<usize> for Column {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Index(pub usize);

impl PartialEq<usize> for Index {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub row: Row,
    pub column: Column,
    pub index: Index,
}

impl Position {
    pub fn new(row: Row, column: Column, index: Index) -> Self {
        Self { row, column, index }
    }
}

impl PartialEq<(usize, usize, usize)> for Position {
    fn eq(&self, other: &(usize, usize, usize)) -> bool {
        self.row == other.0 && self.column == other.1 && self.index == other.2
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            row: Row(1),
            column: Column(1),
            index: Index(0),
        }
    }
}