function add_one (value: Number) : Number {
    return value + 1
}

console.log(add_one(41)) // Expect: 42

function it(description: String, test_case: function() : Bool) {
    let result = test_case()
    if result {
        console.log('✔ ' + description)
    } else {
        console.log('✘ ' + description)
    }
}

function false_fn () { return false }
function true_fn () { return true }

it('true value', true_fn) // Expect: ✔ true value
it('not so true value', true_fn) // Expect: ✔ not so true value