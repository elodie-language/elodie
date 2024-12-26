test('passing and failing check'){
    describe('Now its on'){
        check('a passing check'){ return true }
        check('a failing check'){ return false }
    }
}

// out: passing and failing check
// out:   Now its on
// out:    \x1b[0;32mPass\x1b[0m - a passing check
// out:    \x1b[0;31mFail\x1b[0m - a failing check
