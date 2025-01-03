let x = true

if x == true {
    if false != x {
        rt::io::println('true story')
    }
}

if 10 != 10 { } else { rt::io::println('10 == 10') }

// out:true story
// out:10 == 10