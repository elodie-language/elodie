test('passing and failing check'){
    describe('One'){
        check('a passing check'){ 99 == 99 }
        check('a failing check'){ 1 == 2 }
    }
    describe('Two'){
        check('a passing check'){ 99 == 99 }
        check('a passing check'){ 99 == 99 }
        check('a passing check'){ 99 == 99 }
    }
    describe('Three'){
        check('a failing check'){ 1 == 2 }
        check('a failing check'){ 1 == 2 }
        check('a failing check'){ 1 == 2 }
    }
}

// out: passing and failing check
// out:   One
// out:    \x1b[0;32mPass\x1b[0m - a passing check
// out:    \x1b[0;31mFail\x1b[0m - a failing check
// out:   Two
// out:    \x1b[0;32mPass\x1b[0m - a passing check
// out:    \x1b[0;32mPass\x1b[0m - a passing check
// out:    \x1b[0;32mPass\x1b[0m - a passing check
// out:   Three
// out:    \x1b[0;31mFail\x1b[0m - a failing check
// out:    \x1b[0;31mFail\x1b[0m - a failing check
// out:    \x1b[0;31mFail\x1b[0m - a failing check