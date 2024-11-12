let x = true

if x == true {
    if x != false {
        console.log('true story') // Expect: true story
    }
}

if 10 != 10 { } else { console.log('10 == 10') } // Expect: 10 == 10