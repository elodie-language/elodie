let value = 23
{
    let value = 42
    log.info(value)  // Expect: 42
}
log.info(value) // Expect: 23

if true {
    let value = 123
    log.info(value)  // Expect: 123
}
log.info(value) // Expect: 23

if false { let value = 24 } else {
    let value = 111
    log.info(value) // Expect: 111
}
log.info(value) // Expect: 23

let v = 1

{{{{{
let v = 2
log.info(v) // Expect: 2
}}}}}
log.info(v) // Expect: 1