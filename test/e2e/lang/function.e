fun some_function() -> Bool {
    return true
}

if some_function() {
    console.log('true') // Expect: true
}

fun early_exit() -> Number {
    return 2
    return 4
    return 8
}

console.log(early_exit())  // Expect: 2

fun nested() -> Number {
    fun inner() -> Number {
        return 1
    }

    return inner() + inner () + inner()
}

console.log(nested()) // Expect: 3