let value = 23
{
    let value = 42
    std::io::print_line(value)
}
std::io::print_line(value)

if true {
    let value = 123
    std::io::print_line(value)
}
std::io::print_line(value)

if false { let value = 24 } else {
    let value = 111
    std::io::print_line(value)
}

std::io::print_line(value)

let v = 1

{{{{{
let v = 2
std::io::print_line(v)
}}}}}
std::io::print_line(v)

// out:42
// out:23
// out:123
// out:23
// out:111
// out:23
// out:2
// out:1