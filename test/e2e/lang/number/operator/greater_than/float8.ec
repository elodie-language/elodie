let n1 : Float8 = 1
let n2 : Float8 = 2

rt::io::println('n1 > n1 => ${ n1 > n1 }')
rt::io::println('n1 > n2 => ${ n1 > n2 }')
rt::io::println('n2 > n1 => ${ n2 > n1 }')

// out: n1 > n1 => false
// out: n1 > n2 => false
// out: n2 > n1 => true