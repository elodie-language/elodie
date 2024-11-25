test('test'){
    describe('desc'){
        should('a passing assertion'){ return true }
        should('a failing assertion'){ return false }
    }
}

// out:Test: test
// out:  Describe: desc
// out:    Pass -  a passing assertion 1
// out:    Fail -  a failing assertion
