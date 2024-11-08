let value = 23
{
    let value = 42
    console.log(value)  // Expect: 42
}
console.log(value) // Expect: 23

if true {
    let value = 123
    console.log(value)  // Expect: 123
}
console.log(value) // Expect: 23

if false { let value = 24 } else {
    let value = 111
    console.log(value) // Expect: 111
}
console.log(value) // Expect: 23

let v = 1

{{{{{
let v = 2
console.log(v) // Expect: 2
}}}}}
console.log(v) // Expect: 1