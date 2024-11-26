test('passing and failing assertion'){
    describe('One'){
        should('a passing assertion'){ 99 == 99 }
        should('a failing assertion'){ 1 == 2 }
    }
    describe('Two'){
        should('a passing assertion'){ 99 == 99 }
        should('a passing assertion'){ 99 == 99 }
        should('a passing assertion'){ 99 == 99 }
    }
    describe('Three'){
        should('a failing assertion'){ 1 == 2 }
        should('a failing assertion'){ 1 == 2 }
        should('a failing assertion'){ 1 == 2 }
    }
}

// out:Test: passing and failing assertion
// out:  Describe: One
// out:    \x1b[0;32mPass\x1b[0m -  a passing assertion
// out:    \x1b[0;31mFail\x1b[0m -  a failing assertion
// out:  Describe: Two
// out:    \x1b[0;32mPass\x1b[0m -  a passing assertion
// out:    \x1b[0;32mPass\x1b[0m -  a passing assertion
// out:    \x1b[0;32mPass\x1b[0m -  a passing assertion
// out:  Describe: Three
// out:    \x1b[0;31mFail\x1b[0m -  a failing assertion
// out:    \x1b[0;31mFail\x1b[0m -  a failing assertion
// out:    \x1b[0;31mFail\x1b[0m -  a failing assertion