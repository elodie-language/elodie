let value = 23
{
    let value = 42
    rt::io::println('${value}')
}
rt::io::println('${value}')

if true {
    let value = 123
    rt::io::println('${value}')
}
rt::io::println('${value}')

if false { let value = 24 } else {
    let value = 111
    rt::io::println('${value}')
}

rt::io::println('${value}')

let v = 1

{
    {
        {
            {
                {
                    let v = 2
                    rt::io::println('${v}')
                }
            }
        }
    }
}
rt::io::println('${v}')

// out:42
// out:23
// out:123
// out:23
// out:111
// out:23
// out:2
// out:1