test('passing and failing check'){
    describe('Now its on'){
        check('a passing check'){ 99 == 99 }
        check('a failing check'){ 1 == 2 }
    }
}

// out: passing and failing check
// out:   Now its on
// out:    \x1b[0;32mPass\x1b[0m - a passing check
// out:    \x1b[0;31mFail\x1b[0m - a failing check
