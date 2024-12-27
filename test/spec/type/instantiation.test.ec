test('Type instantiation') {
    describe('Instantiate empty type'){
        type Some_Type ()

        let some_type = Some_Type()
    }
    describe('Instantiate Point(x: Number, y: Number)'){
        type Point (
            x: Number,
            y: Number
        )
        describe('with Point(x = 1, y = 2)') {
            let p = Point( x = 1, y = 2)
            check('p.x == 1') { p.x == 1 }
            check('p.y == 2') { p.y == 2 }
        }
    }
}
