test('test'){
    describe('desc'){
        should('a passing assertion'){ return true }
        should('a failing assertion'){ return false }
    }
}

// out:Test: test
// out:  Describe: desc
// out:    \x1b[0;32mPass\x1b[0m -  a passing assertion
// out:    \x1b[0;31mFail\x1b[0m -  a failing assertion
