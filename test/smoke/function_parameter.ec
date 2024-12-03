function add_one (value: Number) -> Number {
    return value + 1
}

std::io::print_line(add_one(41))

function it(description: String, test_case: function() -> Bool) {
    let result = test_case()
    if result {
        std::io::print_line('✔ ' + description)
    } else {
        std::io::print_line('✘ ' + description)
    }
}

function false_fn () { return false }
function true_fn () { return true }

it('true value', true_fn)
it('not so true value', false_fn)

// out:42
// out:✔ true value
// out:✘ not so true value