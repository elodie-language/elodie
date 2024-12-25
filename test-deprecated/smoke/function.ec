function some_function() -> Bool {
    return true
}

if some_function() {
    std::io::println('true')
}

function early_exit() -> Number {
    return 2
    return 4
    return 8
}

std::io::println('${early_exit()}')

function nested() -> Number {
    function inner() -> Number {
        return 1
    }

    return inner() + inner () + inner()
}

std::io::println('${nested()}')

// out:true
// out:2
// out:3