function some_function() -> Bool {
    return true
}

if some_function() {
    console.log('true') // Expect: true
}

function early_exit() -> Number {
    return 2
    return 4
    return 8
}

console.log(early_exit())  // Expect: 2

function nested() -> Number {
    function inner() -> Number {
        return 1
    }

    return inner() + inner () + inner()
}

console.log(nested()) // Expect: 3