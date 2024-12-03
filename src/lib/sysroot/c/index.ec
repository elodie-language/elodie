export package c {
    type int = Number
    type char = String

    export package stdio{
        export external function printf(c: char) -> int
    }
}