let result = loop {
    let x = 2
    let y = 10
    break x * y
}

rt::io::println('${result}')
// out:20