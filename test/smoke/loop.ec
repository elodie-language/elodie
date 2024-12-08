let result = loop {
    let x = 2
    if x > 1 {
        break x * 10
    }
}

std::io::println(result)

// out:20