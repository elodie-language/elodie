test("Position") {
    describe('A new position created with Position::new(1,2,3)') {
        let pos = Position::new(1, 2, 3)
        it("should have a line of 1") { return pos.line == 1 }
        it("should have a column of 2") { return pos.column == 2 }
        it("should have an index of 3") { return pos.index == 3 }
    }
}
