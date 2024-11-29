test('One'){
    describe('1'){
        should('a passing assertion'){ true }
        should('a failing assertion'){ false }
    }
}

test('Two'){
    describe('2'){
        should('a passing assertion'){ true }
        should('a failing assertion'){ false }
    }
}

test('Three'){
    describe('3'){
        should('a passing assertion'){ true }
        should('a failing assertion'){ false }
    }
}

// out: One
// out:   1
// out:    \x1b[0;32mPass\x1b[0m - a passing assertion
// out:    \x1b[0;31mFail\x1b[0m - a failing assertion

// out: Two
// out:   2
// out:    \x1b[0;32mPass\x1b[0m - a passing assertion
// out:    \x1b[0;31mFail\x1b[0m - a failing assertion

// out: Three
// out:   3
// out:    \x1b[0;32mPass\x1b[0m - a passing assertion
// out:    \x1b[0;31mFail\x1b[0m - a failing assertion