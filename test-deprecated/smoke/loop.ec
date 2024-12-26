let result = loop {
    let x = 2
    if x > 1 {
        break x * 10
    }
}

rt::io::println(result)

// out:20