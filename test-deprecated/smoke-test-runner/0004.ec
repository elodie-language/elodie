test('passing and failing assertion'){
    describe('Now its on'){
        should('a passing assertion'){ return true }
        should('a failing assertion'){ return false }
    }
}

// out: passing and failing assertion
// out:   Now its on
// out:    \x1b[0;32mPass\x1b[0m - a passing assertion
// out:    \x1b[0;31mFail\x1b[0m - a failing assertion
