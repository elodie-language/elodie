test('Point(x: Number, y: Number) instantiation') {
    type Point (
        x: Number,
        y: Number
    )
    describe('A point instantiated with Point(x = 1, y = 2)') {
        let p = Point( x = 1, y = 2)
        should('have a x of 1') { p.x == 1 }
        should('have a y of 2') { p.y == 2 }
    }
}
