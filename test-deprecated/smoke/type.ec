type Point (
    x: Number,
    y: Number
)

let point = Point( x = 1, y = 2 )
rt::io::println('${point.x}')
rt::io::println('${point.y}')

// out:1
// out:2
