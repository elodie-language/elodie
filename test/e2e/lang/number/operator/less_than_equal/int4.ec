let n1 : Int4 = 1
let n2 : Int4 = 1
let n3 : Int4 = 2

rt::io::println('n1 <= n1 => ${ n1 <= n1 }')
rt::io::println('n1 <= n2 => ${ n1 <= n2 }')
rt::io::println('n2 <= n1 => ${ n2 <= n1 }')
rt::io::println('n1 <= n3 => ${ n1 <= n3 }')
rt::io::println('n3 <= n1 => ${ n3 <= n1 }')

// out: n1 <= n1 => true
// out: n1 <= n2 => true
// out: n2 <= n1 => true
// out: n1 <= n3 => true
// out: n3 <= n1 => false