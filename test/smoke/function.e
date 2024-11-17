fun some_function() -> Bool {
    return true
}

if some_function() {
    log.info('true') // Expect: true
}

fun early_exit() -> Number {
    return 2
    return 4
    return 8
}

log.info(early_exit())  // Expect: 2

fun nested() -> Number {
    fun inner() -> Number {
        return 1
    }

    return inner() + inner () + inner()
}

log.info(nested()) // Expect: 3