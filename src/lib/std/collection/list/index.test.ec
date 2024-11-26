test('List') {
    describe('A newly created empty list') {
        let list = std::collection::list::empty()
        should('have a length of 0') { list.length() == 0 }
    }
}