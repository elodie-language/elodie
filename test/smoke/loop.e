let result = loop {
    let x = 2
    if x > 1 {
        break x * 10
    }
}

console.log(result) // Expect: 20