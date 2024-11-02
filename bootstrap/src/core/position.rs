use std::cmp::PartialEq;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineNumber(pub usize);

impl PartialEq<usize> for LineNumber {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineColumn(pub usize);

impl PartialEq<usize> for LineColumn {
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

#[derive(Debug, PartialEq)]
pub struct Position {
    pub line: LineNumber,
    pub column: LineColumn,
    pub index: SourceIndex,
}

impl Position {
    pub fn new(line: LineNumber, column: LineColumn, index: SourceIndex) -> Self {
        Self {
            line,
            column,
            index,
        }
    }
}


impl PartialEq<(usize, usize, usize)> for Position {
    fn eq(&self, other: &(usize, usize, usize)) -> bool {
        self.line == other.0 &&
            self.column == other.1 &&
            self.index == other.2
    }
}