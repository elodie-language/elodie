test('passing and failing assertion'){
    describe('Now its on'){
        should('a passing assertion'){ true }
        should('a failing assertion'){ false }
    }
}

// out:Test: passing and failing assertion
// out:  Describe: Now its on
// out:    \x1b[0;32mPass\x1b[0m - should a passing assertion
// out:    \x1b[0;31mFail\x1b[0m - should a failing assertion
