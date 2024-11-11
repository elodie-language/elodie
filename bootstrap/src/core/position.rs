use std::cmp::PartialEq;

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
pub struct SourceIndex(pub usize);

impl PartialEq<usize> for SourceIndex {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub row: Row,
    pub column: Column,
    pub index: SourceIndex,
}

impl Position {
    pub fn new(line: Row, column: Column, index: SourceIndex) -> Self {
        Self {
            row: line,
            column,
            index,
        }
    }
}


impl PartialEq<(usize, usize, usize)> for Position {
    fn eq(&self, other: &(usize, usize, usize)) -> bool {
        self.row == other.0 &&
            self.column == other.1 &&
            self.index == other.2
    }
}