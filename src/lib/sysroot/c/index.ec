export package c {
    type int = Number
    type char = String

    export package stdio{
        export external fun printf(c: char) -> int
    }
}