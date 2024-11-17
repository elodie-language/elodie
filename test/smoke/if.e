let x = true

if x == true {
    if x != false {
        log.info('true story') // Expect: true story
    }
}

if 10 != 10 { } else { log.info('10 == 10') } // Expect: 10 == 10