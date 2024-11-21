fun add_one (value: Number) -> Number {
    return value + 1
}

log.info(add_one(41)) // Expect: 42

fun it(description: String, test_case: fun() -> Bool) {
    let result = test_case()
    if result {
        log.info('✔ ' + description)
    } else {
        log.info('✘ ' + description)
    }
}

fun false_fn () { return false }
fun true_fn () { return true }

it('true value', true_fn) // Expect: ✔ true value
it('not so true value', false_fn) // Expect: ✘ not so true value