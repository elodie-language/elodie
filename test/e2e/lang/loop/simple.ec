rt::io::println('before loop')
loop{
    rt::io::println('before break')
    break
    rt::io::println('after break')
}
rt::io::println('after loop')

// out:before loop
// out:before break
// out:after loop
