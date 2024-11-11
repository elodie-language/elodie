value type Line = usize
value type Row = usize
value type Source_Index = usize

type Position {
    line: Line
    colum: Row
    index: Source_Index
}

implement Position {
    function new(line: Line, row: Row, index: Source_Index) : Self {
        Self {
            line,
            row,
            index
        }
    }
}

type Text_Span {
    start: Position,
    end: Position,
    value: String
}

implement Text_Span {
    function new(start: Position, end: Position, value: String) : Self {
        Self {
            start,
            end,
            value
        }
    }
}