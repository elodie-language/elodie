function some_function() -> Bool {
    return true
}

if some_function() {
    rt::io::println('true')
}

function early_exit() -> Number {
    return 2
    return 4
    return 8
}

rt::io::println('${early_exit()}')

function nested() -> Number {
    function inner() -> Number {
        return 1
    }

    return inner() + inner () + inner()
}

rt::io::println('${nested()}')

// out:true
// out:2
// out:3