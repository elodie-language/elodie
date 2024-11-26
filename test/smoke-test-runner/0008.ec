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

// out:Test: One
// out:  Describe: 1
// out:    \x1b[0;32mPass\x1b[0m -  a passing assertion
// out:    \x1b[0;31mFail\x1b[0m -  a failing assertion

// out:Test: Two
// out:  Describe: 2
// out:    \x1b[0;32mPass\x1b[0m -  a passing assertion
// out:    \x1b[0;31mFail\x1b[0m -  a failing assertion

// out:Test: Three
// out:  Describe: 3
// out:    \x1b[0;32mPass\x1b[0m -  a passing assertion
// out:    \x1b[0;31mFail\x1b[0m -  a failing assertion