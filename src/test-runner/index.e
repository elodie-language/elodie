fun describe(message: String, fn: fun()){
    fn()
}

fun it(message: String, fn: fun(): Bool){
}

describe('some string'){

}


fun log_summary(total: Number, passed: Number) {
    log.info('Total tests run: ', total)
    log.info('Tests passed: ', passed)
    log.info('Tests failed: ', (total - passed))
    if passed == total {
        log.info('All tests passed! 🎉')
    } else {
        log.info('Some tests failed.')
    }
}

log_summary(100, 100)


