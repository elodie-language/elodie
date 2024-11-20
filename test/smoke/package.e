package my_package {
    export fun hello() {
        log.info('Hello, hellO')
    }
}

my_package::hello() // Expect: Hello, hellO