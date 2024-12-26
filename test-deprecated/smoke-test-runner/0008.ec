test('One'){
    describe('1'){
        check('a passing check'){ true }
        check('a failing check'){ false }
    }
}

test('Two'){
    describe('2'){
        check('a passing check'){ true }
        check('a failing check'){ false }
    }
}

test('Three'){
    describe('3'){
        check('a passing check'){ true }
        check('a failing check'){ false }
    }
}

// out: One
// out:   1
// out:    \x1b[0;32mPass\x1b[0m - a passing check
// out:    \x1b[0;31mFail\x1b[0m - a failing check

// out: Two
// out:   2
// out:    \x1b[0;32mPass\x1b[0m - a passing check
// out:    \x1b[0;31mFail\x1b[0m - a failing check

// out: Three
// out:   3
// out:    \x1b[0;32mPass\x1b[0m - a passing check
// out:    \x1b[0;31mFail\x1b[0m - a failing check