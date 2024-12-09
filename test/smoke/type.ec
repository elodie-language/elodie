type Point (
    x: Number,
    y: Number
)

let point = Point( x = 1, y = 2 )
std::io::println('${point.x}')
std::io::println('${point.y}')

// out:1
// out:2
