test('passing and failing assertion'){
    describe('Now its on'){
        should('a passing assertion'){ 99 == 99 }
        should('a failing assertion'){ 1 == 2 }
    }
}

// out:Test: passing and failing assertion
// out:  Describe: Now its on
// out:    \x1b[0;32mPass\x1b[0m -  a passing assertion
// out:    \x1b[0;31mFail\x1b[0m -  a failing assertion
