fun some_function() -> Bool {
    return true
}

if some_function() {
    std::io::print_line('true')
}

fun early_exit() -> Number {
    return 2
    return 4
    return 8
}

std::io::print_line(early_exit())

fun nested() -> Number {
    fun inner() -> Number {
        return 1
    }

    return inner() + inner () + inner()
}

std::io::print_line(nested())

// out:true
// out:2
// out:3